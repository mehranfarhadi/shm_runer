use shared_memory::*;
use std::sync::{Arc, Mutex};
use std::error::Error;
use libc;

pub struct SafeShmem {
    shmem: Arc<Mutex<Shmem>>,
}

impl SafeShmem {
    pub fn create(key: &str, size: usize) -> Result<Self, Box<dyn Error>> {
        let shmem = ShmemConf::new().size(size).flink(key).create()?;
        Ok(SafeShmem { shmem: Arc::new(Mutex::new(shmem)) })
    }

    pub fn write(&self, data: &str) -> Result<(), Box<dyn Error>> {
        let mut shmem = self.shmem.lock().unwrap();
        let ptr = unsafe { shmem.as_slice_mut() };
        let bytes = data.as_bytes();
        if bytes.len() > ptr.len() {
            return Err("Data is too large for shared memory".into());
        }
        ptr[..bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    pub fn read(&self, size: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        let shmem = self.shmem.lock().unwrap();
        let ptr = unsafe { shmem.as_slice() };
        Ok(ptr[..size].to_vec())
    }
}

unsafe impl Send for SafeShmem {}
unsafe impl Sync for SafeShmem {}

pub unsafe fn free_c_string(s: *mut libc::c_char) {
    libc::free(s as *mut libc::c_void);
}
