use async_trait::async_trait;
use crate::rest::{PublicAPI, PrivateAPI};
use anyhow::Result;
use crate::model::{
    KData, 
    SymbolInfo, 
    Ticker,
    OrderResp,
    QueryOrderResult,
    CancelOrderResult,
    Balance
};
use serde::{Serialize, Deserialize};

const BASE_URL: &'static str = "https://binance.com";
pub struct BinanceSpot<'a> {
    pub base_url: &'a str,
    pub credential: Option<(&'a str, &'a str)>,
}

pub struct BinanceSpotBuilder<'a> {
    pub base_url: &'a str,
    pub credential: Option<(&'a str, &'a str)>,
}

impl<'a> BinanceSpot<'a> {
    fn new() -> BinanceSpotBuilder<'a> {
        BinanceSpotBuilder {
            base_url: BASE_URL,
            credential: None,
        }
    }
}

impl<'a> BinanceSpotBuilder<'a> {
    fn base_url(&mut self, base_url: &'a str) -> &mut Self {
        self.base_url = base_url;
        self
    }

    fn credential(&mut self, api_key: &'a str, sec_key: &'a str) -> &mut Self {
        self.credential = Some((api_key, sec_key));
        self
    }

    fn build(&self) -> BinanceSpot {
        BinanceSpot {
            base_url: self.base_url.clone(),
            credential: self.credential.clone()
        }
    }
}

#[async_trait]
impl<'a> PublicAPI for BinanceSpot<'a> {

    async fn ping(&self) -> Result<()> {
        let end_point = "/api/v3/ping";
        let url = format!("{}{}", self.base_url, end_point);
        let _ = reqwest::get(&url).await?;
        Ok(())
    }

    async fn get_symbols(&self) -> Result<Vec<SymbolInfo>> {
        let end_point = "/api/v3/exchangeInfo";
        let url = format!("{}{}", self.base_url, end_point);
        let resp =  reqwest::get(&url).await?; 
        let resp_text = resp.text().await?;
        let raw_symbol_info: RawSymbolInfoResp= serde_json::from_str(&resp_text)?;
        Ok(Vec::<SymbolInfo>::from(raw_symbol_info))

    }

    async fn get_ticker(&self, symbol: &str) -> Result<Ticker> {
        let end_point = "/api/v3/ticker/bookTicker";
        let url = format!("{}{}?symbol={}", self.base_url, end_point, symbol);
        let resp = reqwest::get(&url).await?;
        let resp_text = resp.text().await?;
        let ticker: Ticker = serde_json::from_str(&resp_text)?;
        Ok(ticker)
    }

    async fn get_klines(
        &self,
        symbol: &str,
        interval: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<KData>> {
        let end_point = "/api/v3/klines";
        let mut url = format!("{}{}?symbol={}&interval={}", self.base_url, end_point, symbol, interval);
        start_time.map_or_else(||{}, |start_ts| url.push_str(format!("startTime={}", start_ts).as_str()));
        if let Some(end_ts) = end_time {
            url.push_str(format!("endTime={}", end_ts).as_str())
        }
        if let Some(lim) = limit {
            url.push_str(format!("limit={}", lim).as_str());
        }
        let resp_text = reqwest::get(&url).await?.text().await?;
        let kline: Vec<RawKResp> = serde_json::from_str(&resp_text)?;
        Ok(kline.iter().map(|rkp| KData::from(*rkp)).collect())
    }

}

#[async_trait]
impl<'a> PrivateAPI for BinanceSpot<'a> {
    async fn new_order(&self, symbol: &str, qty: f64, price: f64, type_: &str, side: &str) -> Result<OrderResp> {
        let end_point = "/api/v3/order";
        
        unimplemented!()
    }

    async fn cancel_order(&self, symbol: &str, order_id: u32) -> Result<CancelOrderResult> {
        unimplemented!()
    }

    async fn query_order(&self, symbol: &str, order_id: u32) -> Result<QueryOrderResult> {
        unimplemented!()
    }

    async fn query_balance(&self) -> Result<Balance> {
        unimplemented!()
    }
}

// =========================
impl From<RawSymbolInfoResp> for Vec<SymbolInfo> {
    fn from(raw: RawSymbolInfoResp) -> Self {
        let mut info_vec = Vec::new();
        for raw_symbol in raw.symbols {
            let symbol_info = SymbolInfo {
                symbol: raw_symbol.symbol,
                base: raw_symbol.base_asset,
                quote: raw_symbol.quote_asset,
                price_precision: raw_symbol.quote_precision,
                quantity_precision: raw_symbol.base_asset_precision,
                base_precision: raw_symbol.base_asset_precision,
                quote_precision: raw_symbol.quote_precision,
            };
            info_vec.push(symbol_info);
        }
        info_vec
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawSymbolInfoResp {
    pub timezone: String,
    pub server_time: i64,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<::serde_json::Value>,
    pub symbols: Vec<Symbol>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: i64,
    pub limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u8,
    pub quote_asset: String,
    pub quote_precision: u8,
    pub quote_asset_precision: u8,
    pub base_commission_precision: u8,
    pub quote_commission_precision: u8,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<Filter>,
    pub permissions: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub filter_type: String,
    pub min_price: Option<String>,
    pub max_price: Option<String>,
    pub tick_size: Option<String>,
    pub multiplier_up: Option<String>,
    pub multiplier_down: Option<String>,
    pub avg_price_mins: Option<i64>,
    pub min_qty: Option<String>,
    pub max_qty: Option<String>,
    pub step_size: Option<String>,
    pub min_notional: Option<String>,
    pub apply_to_market: Option<bool>,
    pub limit: Option<i64>,
    pub max_num_orders: Option<i64>,
    pub max_num_algo_orders: Option<i64>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawKResp {
    ts: u64,
    #[serde(with = "string_or_float")]
    open: f64,
    #[serde(with = "string_or_float")]
    high: f64,
    #[serde(with = "string_or_float")]
    low: f64,
    #[serde(with = "string_or_float")]
    close: f64,
    #[serde(with = "string_or_float")]
    vol: f64,
    close_time: u64,
    #[serde(with = "string_or_float")]
    turnover: f64,
    // 成交笔数
    number: i32,
    #[serde(with = "string_or_float")]
    p_vol: f64,
    #[serde(with = "string_or_float")]
    p_turnover: f64,
    #[serde(with = "string_or_float")]
    nothing: f64, 
}

impl From<RawKResp> for KData {
    fn from(raw: RawKResp) -> Self {
        KData {
            ts: raw.ts,
            open: raw.open,
            high: raw.high,
            low: raw.low,
            close: raw.close,
            vol: raw.vol,
            turnover: raw.turnover
        }
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
