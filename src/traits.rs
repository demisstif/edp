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
    );
    async fn cancel_order();
    async fn query_order();
    async fn get_ticker();
    async fn get_order_book();
    async fn get_klines();
}

#[async_trait]
pub trait SpotAPI: ExchangeAPI {}

#[async_trait]
pub trait PerpetualAPI: ExchangeAPI {}

#[async_trait]
pub trait DeliveryAPI {}
