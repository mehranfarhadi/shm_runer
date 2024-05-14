mod function_loader;
mod shared_memory;
mod error_handling;
mod types;

use std::sync::Arc;
use std::thread;
use libc::printf;

use crate::function_loader::{load_external_function, capture_function_result};
use crate::types::Function;
use crate::shared_memory::SafeShmem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let function = Function {
        name: "example_function".to_string(),
        path: "./libexample.so".to_string(),
        function_type: "dynamic".to_string(),
        documentation: Some("This is an example external function.".to_string()),
        args: vec![],
        outputs: vec!["string".to_string()],
    };

    let shared_memory = SafeShmem::create("/test", 1024)?;
    let shared_memory = Arc::new(shared_memory);
    
    let mut handles = vec![];

    for _ in 0..4 {
        let function_clone = function.clone();
        let shared_memory_clone: Arc<SafeShmem> = Arc::clone(&shared_memory);

        let handle = thread::spawn(move || {
            match load_external_function(&function_clone) {
                Ok(handle) => {
                    match capture_function_result(&handle) {
                        Ok(results) => {
                            println!("hello");
                            shared_memory_clone.write(&serde_json::to_string(&results).unwrap()).unwrap();
                            let data = shared_memory_clone.read(1024).unwrap();
                            println!("Function output: {}", String::from_utf8_lossy(&data));
                        }
                        Err(e) => eprintln!("Error capturing function result: {}", e),
                    }
                }
                Err(e) => eprintln!("Error loading external function: {}", e),
            }
        });
        
        handles.push(handle);
    }


    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    Ok(())
}
