pub mod http_method {
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
