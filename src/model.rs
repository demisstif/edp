use serde::{Serialize, Deserialize};
use serde::de::{DeserializeOwned};

#[derive(Debug, Serialize, Deserialize)]
pub struct KData {
    pub ts: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    // base
    pub vol: f64,
    //quote
    pub turnover: f64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub bid_price: f64,
    #[serde(with="string_or_float")]
    pub bid_qty: f64,
    #[serde(with="string_or_float")]
    pub ask_price: f64,
    #[serde(with="string_or_float")]
    pub ask_qty: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    pub symbol: String,
    pub base: String,
    pub quote: String,
    pub price_precision: u8,
    pub quantity_precision: u8,
    pub base_precision: u8,
    pub quote_precision: u8,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OpenInterest {
    pub symbol: String,
    pub amount: f64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub free: f64,
    #[serde(with = "string_or_float")]
    pub locked: f64,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResult {
    pub symbol: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResult {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub side: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderResult {
    pub symbol: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub side: String,
    pub stop_price: String,
    pub iceberg_qty: String,
    pub time: i64,
    pub update_time: i64,
    pub is_working: bool,
    pub orig_quote_order_qty: String,
}


mod string_or_float {
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