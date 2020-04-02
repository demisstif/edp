use serde::{Serialize, Deserialize};
use serde::de::{DeserializeOwned};
use reqwest::Method;

#[derive(Debug)]
pub struct KData {
    pub ts: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub base_vol: f64,
    pub quote_vol: f64
}

// open high low close
pub type K = [f64;4];

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResp {
    pub symbol: String,
    pub order_id: u32,
    pub order_list_id: i32,
    pub cilent_order_id: String,
    pub transact_time: u64,
    #[serde(with="string_or_float")]
    pub price: f64,
    #[serde(with="string_or_float")]
    pub ori_qty: f64,
    #[serde(with="string_or_float")]
    pub executed_qty: f64,
    #[serde(with="string_or_float")]
    pub cummulative_quote_qty: f64,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub side: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResp {
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
    pub type_: String,
    pub side: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderResp {
    pub symbol: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub side: String,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    #[serde(with = "string_or_float")]
    pub iceberg_qty: f64,
    pub time: i64,
    pub update_time: i64,
    pub is_working: bool,
    #[serde(with = "string_or_float")]
    pub orig_quote_order_qty: f64,
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResp {
    pub maker_commission: i64,
    pub taker_commission: i64,
    pub buyer_commission: i64,
    pub seller_commission: i64,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub update_time: i64,
    pub account_type: String,
    pub balances: Vec<Balance>,
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


pub trait Exchange {
    fn get_ticker() -> Ticker;
    fn get_kdata() -> Vec<KData>;
    fn get_order_book() -> ();
    fn get_info() -> ();
}

pub trait Request: Serialize {
    const METHOD: Method;
    const SIGNED: Option<SupportExchangeApi>;
    const ENDPOINT: &'static str;
    const HAS_PAYLOAD: bool = true;
    type ResponseData: DeserializeOwned;

    #[inline]
    fn no_payload(&self) -> bool {
        !Self::HAS_PAYLOAD
    }
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

pub enum SupportExchangeApi {
    Binance,
    // BinanceFutures,
    // Hbdm,
    // HuobiPro,
    // Okex,
    Bitmex
}