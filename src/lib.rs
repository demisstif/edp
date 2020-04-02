pub mod hbdm;
pub mod exchange;
pub mod model;
pub mod error;
pub mod client;
pub mod binance;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
