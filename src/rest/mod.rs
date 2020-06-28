use crate::model::{KData, Balance, OrderResp, OpenInterest, SymbolInfo, Ticker, CancelOrderResult, QueryOrderResult};
use anyhow::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

pub mod binance_futures;
pub mod binance_spot;

#[async_trait]
pub trait PublicAPI {
    async fn ping(&self) -> Result<()>;

    async fn get_symbols(&self) -> Result<Vec<SymbolInfo>>;

    async fn get_ticker(&self, symbol: &str) -> Result<Ticker>;

    async fn get_klines(
        &self,
        symbol: &str,
        interval: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<KData>>;

    // spot api dont need to impl
    async fn get_open_interest(&self, symbol: &str) -> Result<OpenInterest> {
        Ok(OpenInterest::default())
    }
}

#[async_trait]
pub trait PrivateAPI {
    async fn new_order(
        &self,
        symbol: &str,
        qty: f64,
        price: f64,
        type_: &str,
        side: &str,
    ) -> Result<OrderResp>;
    async fn cancel_order(&self, symbol: &str, order_id: u32) -> Result<CancelOrderResult>;
    async fn query_order(&self, symbol: &str, order_id: u32) -> Result<QueryOrderResult>;
    // user balance
    async fn query_balance(&self) -> Result<Balance>;
}
