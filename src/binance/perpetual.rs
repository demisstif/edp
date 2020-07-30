use crate::model::{OrderBook, OrderResp, Ticker};
use crate::rest::rclient::RestClient;
use crate::ws::wclient::WssClient;
use crate::traits::{ExchangeAPI, PerpetualAPI};
use crate::utils::de2float;
use async_trait::async_trait;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::process;
use crate::indicator::{Indicator, EdpOrderBook, KlineBucket};
use std::collections::HashMap;

// 响应中如有数组，数组元素以时间升序排列，越早的数据越提前。
// 所有时间、时间戳均为UNIX时间，单位为毫秒


pub struct BinancePerpetual {
    rest_client: RestClient,
    wss_client: WssClient,
    order_book: EdpOrderBook,
    kline_bucket: KlineBucket,
}

impl BinancePerpetual {
    pub fn with_key(base_url_rest: String, base_url_ws: String,  keys: (String, String)) -> Self {
        let rest_client = RestClient::with_key(base_url_rest, keys.clone());
        let wss_client = WssClient::with_key(base_url_ws, keys);
        let order_book = EdpOrderBook {};
        let kline_bucket = KlineBucket {};
        Self {
            rest_client,
            wss_client,
            order_book,
            kline_bucket
        }
    }
}

#[async_trait]
impl ExchangeAPI for BinancePerpetual {
    async fn order(
        &self,
        symbol: &str,
        side: &str,
        type_: &str,
        quantity: f64,
        price: Option<f64>,
        time_in_force: &str,
        recv_window: u64,
        new_client_order_id: Option<&str>,
        timestamp: Option<u64>,
    ) -> anyhow::Result<OrderResp> {
        // 1. convert params to request
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("side".to_string(), side.to_string());
        params.insert("type".to_string(), type_.to_string());
        params.insert("quantity".to_string(), quantity.to_string());
        if let Some(price_num) = price {
            params.insert("price".to_string(), price_num.to_string());
        }
        params.insert("timeInForce".to_string(), time_in_force.to_string());
        params.insert("recvWindow".to_string(), recv_window.to_string());
        if let Some(client_id) = new_client_order_id {
            params.insert("newClientOrderId".to_string(), client_id.to_string());
        }

        if let Some(ts) = timestamp {
            params.insert("timestamp".to_string(), ts.to_string());
        } else {
            let ts = Utc::now().timestamp_millis();
            params.insert("timestamp".to_string(), ts.to_string());
        }

        let end_point: &str = "/fapi/v1/order";
        // 构建完整的url
        let url = self
            .rest_client
            .build_request_string(end_point, params, true)?;
        println!("{:#?}", url);
        // 进行请求
        let resp: String = self.rest_client.post_sign(url).await?;
        // 将返回的字符串转换为相应的类型
        let resp_typed: PPOrderResp = serde_json::from_str(&resp)?;
        let or = OrderResp::from(resp_typed);
        Ok(or)
    }

    async fn cancel_order(
        &self,
        symbol: &str,
        order_id: Option<u64>,
        client_order_id: Option<&str>,
    ) -> anyhow::Result<OrderResp> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        if let Some(oi) = order_id {
            params.insert("orderId".to_string(), oi.to_string());
        }
        if let Some(coi) = client_order_id {
            params.insert("origClientOrderId".to_string(), coi.to_string());
        }
        let ts = Utc::now().timestamp_millis();
        params.insert("timestamp".to_string(), ts.to_string());

        let end_point: &str = "/fapi/v1/order";
        let url = self
            .rest_client
            .build_request_string(end_point, params, true)?;
        println!("{:#?}", url);
        let resp = self.rest_client.delete_sign(url).await?;
        let resp_typed: PPOrderResp = serde_json::from_str(&resp)?;
        let or = OrderResp::from(resp_typed);
        Ok(or)
    }

    async fn query_order(
        &self,
        symbol: &str,
        order_id: Option<u64>,
        client_order_id: Option<&str>,
    ) -> anyhow::Result<OrderResp> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        if let Some(oi) = order_id {
            params.insert("orderId".to_string(), oi.to_string());
        }
        if let Some(coi) = client_order_id {
            params.insert("origClientOrderId".to_string(), coi.to_string());
        }
        let ts = Utc::now().timestamp_millis();
        params.insert("timestamp".to_string(), ts.to_string());

        let end_point: &str = "/fapi/v1/order";
        let url = self
            .rest_client
            .build_request_string(end_point, params, true)?;
        println!("{:#?}", url);
        let resp = self.rest_client.get_sign(url).await?;
        println!("{}", resp);
        let resp_typed: PPOrderResp = serde_json::from_str(&resp)?;
        let or = OrderResp::from(resp_typed);
        Ok(or)
    }

    async fn get_ticker(&self, symbol: &str) -> anyhow::Result<Ticker> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        let end_point = "/fapi/v1/ticker/bookTicker";
        let url = self
            .rest_client
            .build_request_string(end_point, params, false)?;
        let resp = self.rest_client.get_sign(url).await?;
        let ticker: Ticker = serde_json::from_str(&resp)?;
        Ok(ticker)
    }

    async fn get_order_book(&self, symbol: &str, limit: Option<u64>) -> anyhow::Result<OrderBook> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        // 默认100条
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let end_point = "/fapi/v1/depth";
        let url = self
            .rest_client
            .build_request_string(end_point, params, false)?;
        let resp = self.rest_client.get_sign(url).await?;
        let ob: OrderBook = serde_json::from_str(&resp)?;
        Ok(ob)
    }

    async fn get_klines() {
        unimplemented!()
    }
}

impl PerpetualAPI for BinancePerpetual {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PPOrderResp {
    pub order_id: u64,
    pub symbol: String,
    pub status: String,
    pub client_order_id: String,
    #[serde(with = "de2float")]
    pub price: f64,
    #[serde(with = "de2float")]
    pub avg_price: f64,
    #[serde(with = "de2float")]
    pub orig_qty: f64,
    #[serde(with = "de2float")]
    pub executed_qty: f64,
    // #[serde(with="de2float")]
    pub cum_qty: Option<String>,
    #[serde(with = "de2float")]
    pub cum_quote: f64,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: String,
    pub position_side: String,
    #[serde(with = "de2float")]
    pub stop_price: f64,
    pub working_type: String,
    pub orig_type: String,
    pub update_time: i64,
    pub time: Option<i64>,
}

impl From<PPOrderResp> for OrderResp {
    fn from(ppo: PPOrderResp) -> Self {
        Self {
            symbol: ppo.symbol,
            order_id: ppo.order_id,
            client_order_id: ppo.client_order_id,
            transact_time: ppo.update_time,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    const REST_BASE_URL: &'static str = "https://fapi.binance.com";
    const WS_BASE_URL: &'static str = "wss://fstream.binance.com";

    fn get_client() -> BinancePerpetual {
        dotenv().ok();
        let api_key = env::var("API_KEY").unwrap();
        let sec_key = env::var("SEC_KEY").unwrap();
        let bp = BinancePerpetual::with_key(REST_BASE_URL.to_string(), WS_BASE_URL.to_string(), (api_key, sec_key));
        bp
    }

    #[tokio::main]
    #[test]
    async fn test_order() {
        /*
        symbol=BTCUSDT
        &side=BUY
        &type=LIMIT
        &timeInForce=GTC
        &quantity=1
        &price=0.1
        &recvWindow=5000
        &timestamp=1499827319559
        */
        // let bp = BinancePerpetual::new();
        let secret_key = "2b5eb11e18796d12d88f13dc27dbbd02c2cc51ff7059765ed9821957d82bb4d9";
        let api_key = "dbefbc809e3e83c283a984c3a1459732ea7db1360ca80c5c2c8867408d28cc83";
        let target_signature = "3c661234138461fcc7a7d8746c6558c9842d4e10870d2ecbedf7777cad694af9";
        let bp = BinancePerpetual::with_key(REST_BASE_URL.to_string(), WS_BASE_URL.to_string(), (api_key.to_string(), secret_key.to_string()));

        //https://fapi.binance.com/fapi/v1/order?symbol=BTCUSDT&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943&signature=3c661234138461fcc7a7d8746c6558c9842d4e10870d2ecbedf7777cad694af9
        //https://fapi.binance.com/fapi/v1/order?symbol=BTCUSDT&side=BUY&type=LIMIT&quantity=1&price=9000&timeInForce=GTC&recvWindow=5000&timestamp=1591702613943&signature=3c661234138461fcc7a7d8746c6558c9842d4e10870d2ecbedf7777cad694af9

        bp.order(
            "BTCUSDT",
            "BUY",
            "LIMIT",
            1.,
            Some(9000.),
            "GTC",
            5000,
            None,
            Some(1591702613943),
            // None
        )
        .await;
    }

    #[tokio::main]
    #[test]
    async fn test_send_order() {
        let bp = get_client();
        let order_status = bp
            .order(
                "ETHUSDT",
                "BUY",
                "LIMIT",
                1.1,
                Some(210.1),
                "GTC",
                5000,
                None,
                None,
            )
            .await;
        match order_status {
            Ok(os) => {
                println!("{:#?}", os);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }

    #[tokio::main]
    #[test]
    async fn test_cancel_order() {
        let bp = get_client();
        let order_status = bp.cancel_order("ETHUSDT", Some(2163768930), None).await;
        match order_status {
            Ok(os) => {
                println!("{:#?}", os);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }

    #[tokio::main]
    #[test]
    async fn test_query_order() {
        let bp = get_client();
        let order_status = bp
            .query_order("ETHUSDT", None, Some("bzFftXVJkS3bPyF0MTZm6Q"))
            .await;
        match order_status {
            Ok(os) => {
                println!("{:#?}", os);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }

    #[tokio::main]
    #[test]
    async fn test_get_ticker() {
        let bp = get_client();
        let order_status = bp.get_ticker("ETHUSDT").await;
        match order_status {
            Ok(os) => {
                println!("{:#?}", os);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }

    #[tokio::main]
    #[test]
    async fn test_get_order_book() {
        let bp = get_client();
        let order_status = bp.get_order_book("ETHUSDT", None).await;
        match order_status {
            Ok(os) => {
                println!("{:#?}", os);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }
}
