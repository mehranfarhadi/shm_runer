use libloading::{Library, Symbol};
use std::ffi::CStr;
use std::error::Error;
use std::sync::Arc;
use crate::types::Function;
use crate::shared_memory::free_c_string;

pub struct FunctionHandle {
    pub lib: Arc<Library>,
    pub func: Symbol<'static, unsafe extern "C" fn() -> *mut libc::c_char>,
}

pub fn load_external_function(data: &Function) -> Result<FunctionHandle, Box<dyn Error>> {
    // Create the library handle inside an Arc
    let lib = Arc::new(unsafe { Library::new(&data.path)? });

    // Box the Arc<Library> to extend its lifetime
    let lib_ref: &'static Library = Box::leak(Box::new(lib.clone()));

    // Extract the symbol from the library, ensuring the correct lifetime
    let func: Symbol<'static, unsafe extern "C" fn() -> *mut libc::c_char> = unsafe {
        lib_ref.get(data.name.as_bytes())?
    };

    // Return the FunctionHandle with the Arc<Library> and the symbol
    Ok(FunctionHandle { lib, func })
}

pub fn capture_function_result(handle: &FunctionHandle) -> Result<Vec<String>, Box<dyn Error>> {
    unsafe {
        let output_ptr = (handle.func)();
        if output_ptr.is_null() {
            return Err("Function returned null pointer".into());
        }
        let c_str: &CStr = CStr::from_ptr(output_ptr);
        let output = c_str.to_str()?.to_owned();
        free_c_string(output_ptr);
        Ok(vec![output])
    }
}
