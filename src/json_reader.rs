use std::fs::File;
use std::io::BufReader;
use serde_json;
use std::error::Error;
use crate::types::FunctionList;

pub fn load_functions_from_json(file_path: &str) -> Result<FunctionList, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let functions: FunctionList = serde_json::from_reader(reader)?;
    Ok(functions)
}
