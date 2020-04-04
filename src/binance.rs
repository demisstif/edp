use super::exchange::Exchange;
use super::model::Request;
use super::model::{
    CancelOrderResp, GetBalanceResp, KData, OrderResp, QueryOrderResp, SupportExchangeApi, Ticker,
};
use super::utils::{to_f64, to_u64};
use chrono::Utc;
use failure::Fallible;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const BINANCE_HOST_URL: &'static str = "https://www.binance.com";

pub struct Binance {
    exchange: Exchange,
}

impl Binance {
    pub fn new() -> Self {
        let exchange = Exchange::new(BINANCE_HOST_URL);
        Binance { exchange }
    }

    pub fn with_credential(api_key: &str, sec_key: &str) -> Self {
        let exchange = Exchange::with_credential(api_key, sec_key, BINANCE_HOST_URL);
        Binance { exchange }
    }
}

impl Binance {
    pub fn get_klines(
        &self,
        symbol: &str,
        interval: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Fallible<Vec<KData>> {
        let req = GetKlinesRequest {
            symbol: symbol.into(),
            interval: interval.into(),
            start_time,
            end_time,
            limit,
        };
        let resp_data = self.exchange.request(req)?;
        let mut vec_data = Vec::new();
        // todo try iter
        for one_k in resp_data {
            let kdata = KData {
                ts: to_u64(&one_k[0]),
                open: to_f64(&one_k[1]),
                high: to_f64(&one_k[2]),
                low: to_f64(&one_k[3]),
                close: to_f64(&one_k[4]),
                vol: to_f64(&one_k[5]),
                turnover: to_f64(&one_k[7]),
            };
            vec_data.push(kdata);
        }
        Ok(vec_data)
    }

    pub fn get_ticker(&self, symbol: &str) -> Fallible<Ticker> {
        let req = GetTickerRequest {
            symbol: symbol.into(),
        };
        self.exchange.request(req)
    }

    pub fn limit_order(
        &self,
        symbol: &str,
        quantity: f64,
        price: f64,
        side: &str,
    ) -> Fallible<OrderResp> {
        let req = LimitOrderRequest {
            symbol: symbol.into(),
            quantity: quantity,
            price: price,
            side: side.into(),
            type_: "LIMIT".into(),
            time_in_force: "GTC".into(),
            timestamp: Utc::now().timestamp_millis().to_string(),
            recv_window: 5000,
        };
        self.exchange.request(req)
    }

    pub fn cancel_order(&self, symbol: &str, order_id: u64) -> Fallible<CancelOrderResp> {
        let req = CancelOrderRequest {
            symbol: symbol.into(),
            order_id: order_id,
            timestamp: Utc::now().timestamp_millis() as u64,
        };
        self.exchange.request(req)
    }

    pub fn query_order(&self, symbol: &str, order_id: u64) -> Fallible<QueryOrderResp> {
        let req = QueryOrderRequest {
            symbol: symbol.into(),
            order_id: order_id,
            timestamp: Utc::now().timestamp_millis() as u64,
        };
        self.exchange.request(req)
    }

    pub fn get_balance(&self) -> Fallible<GetBalanceResp> {
        let req = GetBalanceRequest {
            timestamp: Utc::now().timestamp_millis() as u64,
            recv_window: 5000,
        };
        self.exchange.request(req)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetKlinesRequest {
    pub symbol: String,
    pub interval: String,
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>,
    pub limit: Option<u32>,
}

impl Request for GetKlinesRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: Option<SupportExchangeApi> = None;
    const ENDPOINT: &'static str = "/api/v3/klines";
    const HAS_PAYLOAD: bool = true;
    type ResponseData = Vec<Vec<Value>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTickerRequest {
    pub symbol: String,
}

impl Request for GetTickerRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: Option<SupportExchangeApi> = None;
    const ENDPOINT: &'static str = "/api/v3/ticker/bookTicker";
    const HAS_PAYLOAD: bool = true;
    type ResponseData = Ticker;
}

// buy and sell
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrderRequest {
    pub symbol: String,
    pub quantity: f64,
    pub price: f64,
    pub side: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub time_in_force: String,
    pub timestamp: String,
    pub recv_window: u32,
}

impl Request for LimitOrderRequest {
    const METHOD: Method = Method::POST;
    const SIGNED: Option<SupportExchangeApi> = Some(SupportExchangeApi::Binance);
    const ENDPOINT: &'static str = "/api/v3/order";
    const HAS_PAYLOAD: bool = true;
    type ResponseData = OrderResp;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub symbol: String,
    pub timestamp: u64,
    pub order_id: u64,
}

impl Request for CancelOrderRequest {
    const METHOD: Method = Method::DELETE;
    const SIGNED: Option<SupportExchangeApi> = Some(SupportExchangeApi::Binance);
    const ENDPOINT: &'static str = "/api/v3/order";
    const HAS_PAYLOAD: bool = true;
    type ResponseData = CancelOrderResp;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderRequest {
    pub symbol: String,
    pub timestamp: u64,
    pub order_id: u64,
}

impl Request for QueryOrderRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: Option<SupportExchangeApi> = Some(SupportExchangeApi::Binance);
    const ENDPOINT: &'static str = "/api/v3/order";
    const HAS_PAYLOAD: bool = true;
    type ResponseData = QueryOrderResp;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {
    pub timestamp: u64,
    pub recv_window: u32,
}

impl Request for GetBalanceRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: Option<SupportExchangeApi> = Some(SupportExchangeApi::Binance);
    const ENDPOINT: &'static str = "/api/v3/account";
    const HAS_PAYLOAD: bool = true;
    type ResponseData = GetBalanceResp;
}

#[cfg(test)]
mod tests {
    use super::*;
    const API_KEY: &'static str =
    "70X0erBoOnC6q2WmcD7NAytJxYNKNeCd9nbsB9r6To8MPO4q73h6dlafpUNHq9G8";
const SEC_KEY: &'static str =
    "cgFD6Z3aix8VnmjrVZUfKtHHHtRyUIfQkQaa50OOfP1N21j0vMXBxX95iAYBYUb2";
    #[test]
    fn test_get_klines() {
        let binance = Binance::new();
        let klines = binance.get_klines("BTCUSDT", "1d", None, None, Some(3));
        println!("{:#?}", klines);
    }

    #[test]
    fn test_get_ticker() {
        let binance = Binance::new();
        let tickers = binance.get_ticker("BTCUSDT");
        println!("{:#?}", tickers);
    }
    #[test]
    fn test_limit_order() {
       
        let binance = Binance::with_credential(API_KEY, SEC_KEY);
        let resp = binance.limit_order("BTCUSDT", 0.1, 6300.1, "SELL");
        println!("{:#?}", resp);
    }

    #[test]
    fn test_get_balance() {
        let binance = Binance::with_credential(API_KEY, SEC_KEY);
        let resp = binance.get_balance();
        println!("{:#?}", resp);
    }
}
