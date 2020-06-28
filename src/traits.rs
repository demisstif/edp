use crate::model::{OrderResp, Ticker, OrderBook};
use anyhow::Result;
use async_trait::async_trait;


#[async_trait]
pub trait ExchangeAPI {
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
    ) -> Result<OrderResp>;

    async fn cancel_order(
        &self,
        symbol: &str,
        order_id: Option<u64>,
        client_order_id: Option<&str>,
    ) -> anyhow::Result<OrderResp>;

    async fn query_order(&self, symbol: &str, order_id: Option<u64>, client_order_id: Option<&str>) -> anyhow::Result<OrderResp>;
    async fn get_ticker(&self, symbol: &str) -> anyhow::Result<Ticker>;
    async fn get_order_book(&self, symbol: &str, limit: Option<u64>) -> anyhow::Result<OrderBook>;
    async fn get_klines();
}

#[async_trait]
pub trait SpotAPI: ExchangeAPI {}

#[async_trait]
pub trait PerpetualAPI: ExchangeAPI {}

#[async_trait]
pub trait DeliveryAPI {}
