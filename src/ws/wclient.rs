use async_tungstenite::tungstenite::Message;
use async_tungstenite::tokio::connect_async;
use futures::{future, pin_mut, StreamExt};
use tokio::task;

pub(crate)  struct WssClient {
    base_url: String,
    keys: (String, String),
}

impl WssClient {
    pub fn with_key(base_url: String, keys: (String, String)) -> Self {
        Self {
            base_url: base_url,
            keys: keys,
        }
    }

    pub async fn connect(&self) {
        let (ws_stream, _) = connect_async(&self.base_url).await.unwrap();
        let (write, read) = ws_stream.split();
        read.for_each(|m| async{
            let data = m.unwrap().into_data();
            println!("{:#?}", String::from_utf8(data).unwrap());
        }).await;

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    const WS_BASE_URL: &'static str = "wss://fstream.binance.com/stream?streams=btcusdt@depth/btcusdt@kline_1h";

    fn get_client() -> WssClient {
        dotenv().ok();
        let api_key = env::var("API_KEY").unwrap();
        let sec_key = env::var("SEC_KEY").unwrap();
        let client = WssClient::with_key(WS_BASE_URL.to_string(), (api_key, sec_key));
        client
    }

    #[tokio::main]
    #[test]
    async fn test_connect() {
        let client = get_client();
        client.connect().await;
    }
}