use crate::collectors::*;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Symbols {
    symbol: String,
    baseAsset: Option<String>,
    quoteAsset: Option<String>,
}

#[derive(Deserialize)]
pub struct Base {
    symbols: std::vec::Vec<Symbols>
}

pub struct BinanceUS;

impl BinanceUS {
    pub fn new() -> Self {
        BinanceUS {}
    }
}

pub struct Binance;

impl Binance {
    pub fn new() -> Self {
        Binance {}
    }
}

// https://api.binance.com/api/v3/exchangeInfo
// https://api.binance.us/api/v3/exchangeInfo
fn collect(url: &str) -> Result<symbol::Symbols, error::Error> {
    reqwest::blocking::get(url).
        map_err(|_| error::Error::HTTPError())?.
        json::<Base>().
        map_err(|_| error::Error::ParseError()).
        and_then(|x| {
            let a = x.symbols.
                into_iter().
                fold(std::vec::Vec::<symbol::Symbol>::new(),
                     |mut symbols, binance_symbol| {
                         if let (Some(based), Some(quoted)) = (binance_symbol.baseAsset, binance_symbol.quoteAsset) {
                             symbols.push(
                                 symbol::Symbol::new(
                                     binance_symbol.symbol,
                                     based,
                                     quoted,
                                     "".to_string())
                             );
                         }

                         symbols
                     });

            Ok(symbol::Symbols::new(a))
        })
}

impl symbol::Collector for Binance {
    fn collect(&self) -> Result<symbol::Symbols, error::Error> {
        collect("https://api.binance.com/api/v3/exchangeInfo")
    }
}

impl symbol::Collector for BinanceUS {
    fn collect(&self) -> Result<symbol::Symbols, error::Error> {
        collect("https://api.binance.us/api/v3/exchangeInfo")
    }
}

