use crate::app::RequestData;
use std::{
    collections::HashMap,
    fs::File,
    io::{Result, Write},
    path::Path,
};

pub fn write_collections(data: &HashMap<String, Vec<RequestData>>, path: &Path) -> Result<bool> {
    let result = serde_json::to_string(&data);
    match result {
        Err(_) => Ok(false),
        Ok(json) => {
            let mut file = match File::create(path) {
                Err(cause) => panic!("couldnt open file"),
                Ok(file) => file,
            };

            match file.write_all(json.as_bytes()) {
                Err(cause) => Ok(false),
                Ok(_) => Ok(true),
            }
        }
    }
}
