use crate::app::RequestData;
use std::{
    collections::HashMap,
    fs::File,
    io::{Result, Write},
    path::Path,
};

pub mod http_method_serde {
    use reqwest::Method;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(method: &Method, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(method.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Method, D::Error>
    where
        D: Deserializer<'de>,
    {
        let method_str: &str = Deserialize::deserialize(deserializer)?;
        Method::from_str(method_str).map_err(serde::de::Error::custom)
    }
}

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
