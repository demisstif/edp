#[tokio::main]
async fn main() {
	println!("hello");
	let binance = edp::binance::spot::BinanceSpot {
        base_url: "https://dapi.binance.com",
        credential: None
    };
    // let data = binance.get_klines(symbol, "1m", None, None, Some(10)).await.expect("get price error");
    // println!("{:?}", data)
}