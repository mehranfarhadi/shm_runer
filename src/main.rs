mod function_loader;
mod shared_memory;
mod error_handling;
mod types;
mod json_reader;

use std::sync::Arc;
use std::thread;
use libc::printf;

use crate::function_loader::{load_external_function, capture_function_result};
use crate::types::{Function, FunctionList};
use crate::shared_memory::SafeShmem;
use crate::json_reader::load_functions_from_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load functions from JSON file
    let function_list = load_functions_from_json("./functions.json")?;
    
    // Print the loaded functions
    for function in &function_list.functions {
        println!("{:?}", function);
    }

    let mut handles = vec![];

    // Iterate over the functions and create threads to handle them
    for function in function_list.functions {
        let function_clone = function.clone();
        let shared_memory = SafeShmem::create(&format!("/{}", function.name), 1024)?;
        let shared_memory = Arc::new(shared_memory);

        let shared_memory_clone = Arc::clone(&shared_memory);

        let handle = thread::spawn(move || {
            match load_external_function(&function_clone) {
                Ok(handle) => {
                    match capture_function_result(&handle) {
                        Ok(results) => {
                            println!("Function executed successfully");
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

    // Join all the threads
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    Ok(())
}
