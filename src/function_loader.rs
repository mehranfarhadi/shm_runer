use libloading::{Library, Symbol};
use std::ffi::CStr;
use std::error::Error;
use std::sync::Arc;
use crate::types::{Function};

pub struct FunctionHandle {
    pub lib: Arc<Library>,
    pub func: Symbol<'static, unsafe extern "C" fn() -> *const libc::c_char>, // Updated signature
}

pub fn load_external_function(data: &Function) -> Result<FunctionHandle, Box<dyn Error>> {
    let lib = Arc::new(unsafe { Library::new(&data.path)? });
    let lib_ref: &'static Library = Box::leak(Box::new(lib.clone()));
    let func: Symbol<'static, unsafe extern "C" fn() -> *const libc::c_char> = unsafe {
        lib_ref.get(b"example_function\0")?
    };
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
        // Do not free the static string
        Ok(vec![output])
    }
}
