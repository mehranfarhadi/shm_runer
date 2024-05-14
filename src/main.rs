mod function_loader;
mod shared_memory;
mod error_handling;
mod types;

use std::sync::Arc;
use std::thread;
use crate::function_loader::{load_external_function, capture_function_result};
use crate::types::Function;
use crate::shared_memory::SafeShmem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the function metadata
    let function = Function {
        name: "example_function".to_string(),
        path: "./libexample.so".to_string(), // Path to the dynamic library
        function_type: "dynamic".to_string(),
        documentation: Some("This is an example external function.".to_string()),
        args: vec![],
        outputs: vec!["string".to_string()],
    };

    // Create shared memory
    let shared_memory = SafeShmem::create("/my_shared_memory", 1024)?;
    let shared_memory = Arc::new(shared_memory);
    
    let mut handles = vec![];

    for _ in 0..4 { // Example: Create 4 threads
        let function_clone = function.clone();
        let shared_memory_clone: Arc<SafeShmem> = Arc::clone(&shared_memory);

        let handle = thread::spawn(move || {
            // Load the external function
            match load_external_function(&function_clone) {
                Ok(handle) => {
                    // Execute the function and capture the result
                    match capture_function_result(&handle) {
                        Ok(results) => {
                            // Write results to shared memory
                            shared_memory_clone.write(&serde_json::to_string(&results).unwrap()).unwrap();
                            
                            // Read results from shared memory
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

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    Ok(())
}
