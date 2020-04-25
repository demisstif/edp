use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use reqwest;
use chrono::prelude::*;

const HBDM_API_HOST: &'static str = "https://api.hbdm.com";

#[derive(Debug, Serialize, Deserialize)]
pub struct Hbdm {
    pub api_host: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractInfo {
    pub symbol: String,
    pub contract_code: String,
    pub contract_type: String,
    pub contract_size:f64,
    pub price_tick: f64,
    pub delivery_date: String,
    pub create_date: String,
    pub contract_status: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractInfoResp {
    status: String,
    data: Vec<ContractInfo>,
    ts: u64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Kline {
    pub id: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    // Transaction Volume(amount)
    // 成交量(张)，买卖双边成交量之和,
    pub vol: f64, 
    // transaction volume(currency), sum(every transaction volume(amount)*every contract value/transaction price for this contract)
    // 成交量(币), 即 sum(每一笔成交量(张)*单张合约面值/该笔成交价)
    pub amount: f64, 
    pub count: u64

}

#[derive(Debug, Serialize, Deserialize)]
pub struct KlineResp {
    pub status: String,
    pub ch: String,
    pub data: Vec<Kline>,
    pub ts: u64,

}

impl Hbdm {
    pub fn new() -> Hbdm {
        Hbdm {
            api_host: HBDM_API_HOST.to_string(),
        }
    }

    pub fn get_contract_info(&self) -> Result<ContractInfoResp, Box<dyn std::error::Error>>{
        let endpoint = "/api/v1/contract_contract_info";
        let mut url = self.api_host.clone();
        url.push_str(endpoint);
        let res: ContractInfoResp = reqwest::blocking::get(&url)?.json()?;
        Ok(res)
    }

    pub fn get_kline(&self, symbol: &str, period: &str, from: u64, to: u64) -> Result<KlineResp, Box<dyn std::error::Error>> {
        let endpoint = format!("/market/history/kline?symbol={}&period={}&from={}&to={}", symbol, period, from, to);
        let mut url = self.api_host.clone();
        url.push_str(&endpoint);
        let res: KlineResp = reqwest::blocking::get(&url)?.json()?;
        Ok(res)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_hbdm_kline() {
        let hbdm = Hbdm::new();
        let from = "2020-03-22T08:00:00Z".parse::<DateTime<Utc>>().unwrap().timestamp() as u64;
        let to = "2020-03-23T07:00:00Z".parse::<DateTime<Utc>>().unwrap().timestamp() as u64;
        let resp = hbdm.get_kline("BTC_CQ", "60min", from, to).unwrap();
        println!("{:#?}", resp);
    }

    #[test]
    fn test_get_contract_info() {
        let hbdm = Hbdm::new();
        match hbdm.get_contract_info() {
            Ok(res) => {
                println!("{:#?}", res);
            },
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }
}
