use crate::traits::{ExchangeAPI, PerpetualAPI};
use async_trait::async_trait;
use reqwest::Client;

// 响应中如有数组，数组元素以时间升序排列，越早的数据越提前。
// 所有时间、时间戳均为UNIX时间，单位为毫秒
const REST_BASE_URL: &'static str = "https://fapi.binance.com";
const WS_BASE_URL: &'static str = "wss://fstream.binance.com";

pub struct BinancePerpetual {
    base_url: String,
    // rest_client: Client,
    // wss_client: String,
}

impl BinancePerpetual {
    pub fn new() -> Self {
        Self {
            base_url: REST_BASE_URL.to_string(),
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
    ) {
        // 1. convert params to request
        let path: &str = "/fapi/v1/order";
        let mut request_string = format!("{}{}", self.base_url, path);
        request_string.push_str(&format!("?symbol={}", symbol));
        request_string.push_str(&format!("&side={}", side));
        request_string.push_str(&format!("&type={}", type_));
        request_string.push_str(&format!("&quantity={}", quantity));
        request_string.push_str(&format!("&timeInForce={}", time_in_force));
        request_string.push_str(&format!("&recv_window={}", recv_window));
        if let Some(client_id) = new_client_order_id {
            request_string.push_str(&format!("&newClientOrderId={}", client_id));
        }
        if let Some(price_num) = price {
            request_string.push_str(&format!("&price={}", price_num));
        }
        // 2. get response
        println!("{:?}", request_string);
        // 3. response to info
    }

    async fn cancel_order() {
        unimplemented!()
    }

    async fn query_order() {
        unimplemented!()
    }

    async fn get_ticker() {
        unimplemented!()
    }

    async fn get_order_book() {
        unimplemented!()
    }

    async fn get_klines() {
        unimplemented!()
    }
}

impl PerpetualAPI for BinancePerpetual {}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::main]
    #[test]
    async fn test_order() {
        let bp = BinancePerpetual::new();

        bp.order(
            "btcusdt",
            "BUY",
            "LIMIT",
            0.65,
            Some(9732.2),
            "TIF",
            2000,
            Some("2343212"),
        ).await
    }
}
