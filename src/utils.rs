use serde_json::{Value};

pub fn to_u64(v: &Value) -> u64 {
    v.as_u64().unwrap()
}

pub fn to_f64(v: &Value) -> f64 {
    v.as_str().unwrap().parse().unwrap()
}

pub mod string2float {
    use std::fmt;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where T: fmt::Display,
              S: Serializer
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }
        
        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}
