use super::{PublicAPI, PrivateAPI};
use crate::model::{KData, SymbolInfo, Ticker, OpenInterest};
use async_trait::async_trait;
use std::str::FromStr;
use serde_json::{Value};
use serde::{Serialize, Deserialize};
use std::convert::From;
use anyhow::Result;
use anyhow::{anyhow, format_err};

const BASE_URL: &'static str = "https://fapi.binance.com";

pub struct BinanceFutures {
    pub base_url: &'static str,
    pub credential: Option<(String, String)>,
}

pub struct BinanceFuturesBuilder {
    
}

impl BinanceFutures {
    pub fn new() -> Self{
        Self {
            base_url: BASE_URL,
            credential: None,
        }
    }

    pub fn with_credential(api_key: &str, sec_key: &str) -> Self {
        Self {
            base_url: BASE_URL,
            credential: Some((api_key.to_string(), sec_key.to_string())),
        }
    }
}

#[async_trait]
impl PublicAPI for BinanceFutures {
    async fn ping(&self) -> Result<()>{
        let end_point = "/fapi/v1/ping";
        let url = format!("{}{}", self.base_url, end_point);
        match reqwest::get(&url).await {
            Ok(_) => {
                Ok(())
            },
            Err(err) => {
                Err(anyhow!("{}", err))
            }
        }
    }

    async fn get_symbols(&self) -> Result<Vec<SymbolInfo>> {
        let url = format!("{}{}", self.base_url, "/fapi/v1/exchangeInfo");
        match reqwest::get(&url).await {
            Ok(resp) => {
               let resp_text = resp.text().await?;
               let raw_symbol_info: RawSymbolInfoResp = serde_json::from_str(&resp_text)?;
               Ok(Vec::<SymbolInfo>::from(raw_symbol_info))
            }, 
            Err(err) => {
                Err(format_err!("{}", err))
            }
        }
    }

    async fn get_ticker(&self, symbol: &str) -> Result<Ticker> {
        unimplemented!();
    }

    async fn get_klines(
        &self,
        symbol: &str,
        interval: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<KData>> {
        unimplemented!();
    }

    async fn get_open_interest(&self, symbol: &str) -> Result<OpenInterest> {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::main]
    #[test]
   async fn test_BinanceFutures_ping() {
        let exchange = BinanceFutures::new(); 
        match exchange.ping().await {
            Ok(_) => {
                println!("ping success")
            },
            Err(err) => {
                println!("{}", err)
            }
        }
   }

   #[tokio::main]
   #[test]
   async fn test_BinanceFutures_symbols() {
       let exchange = BinanceFutures::new();
       match exchange.get_symbols().await {
           Ok(symbols) => {
               println!("{:#?}", symbols)
           },
           Err(err) => {
               println!("{}", err)
           }
       }
   }
}




// ======================
impl From<RawSymbolInfoResp> for Vec<SymbolInfo> {
    fn from(raw: RawSymbolInfoResp) -> Self {
        let mut info_vec = Vec::new();
        for raw_symbol in raw.symbols {
            let symbol_info = SymbolInfo {
                symbol: raw_symbol.symbol,
                base: raw_symbol.base_asset,
                quote: raw_symbol.quote_asset,
                price_precision: raw_symbol.price_precision,
                quantity_precision: raw_symbol.quantity_precision,
                base_precision: raw_symbol.base_asset_precision,
                quote_precision: raw_symbol.quote_precision,
            };
            info_vec.push(symbol_info);
        }
        info_vec
    }
}
// Raw Response Struct
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawSymbolInfoResp {
    pub timezone: String,
    pub server_time: i64,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<::serde_json::Value>,
    pub symbols: Vec<Symbol>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: i64,
    pub limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Symbol {
    pub symbol: String,
    pub status: String,
    pub maint_margin_percent: String,
    pub required_margin_percent: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub price_precision: u8,
    pub quantity_precision: u8,
    pub base_asset_precision: u8,
    pub quote_precision: u8,
    pub filters: Vec<Filter>,
    pub order_types: Vec<String>,
    pub time_in_force: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Filter {
    pub min_price: Option<String>,
    pub max_price: Option<String>,
    pub filter_type: String,
    pub tick_size: Option<String>,
    pub step_size: Option<String>,
    pub max_qty: Option<String>,
    pub min_qty: Option<String>,
    pub limit: Option<i64>,
    pub multiplier_down: Option<String>,
    pub multiplier_up: Option<String>,
    pub multiplier_decimal: Option<String>,
}
