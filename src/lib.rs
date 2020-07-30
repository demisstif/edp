#![allow(unused)]

pub mod utils;
pub mod rest;
pub mod ws;
pub mod error;
pub mod model;
pub mod binance;
pub mod traits;
pub(crate) mod indicator;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
