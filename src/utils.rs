use serde_json::{Value};

pub fn to_u64(v: &Value) -> u64 {
    v.as_u64().unwrap()
}

pub fn to_f64(v: &Value) -> f64 {
    v.as_str().unwrap().parse().unwrap()
}

