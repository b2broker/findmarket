use crate::collectors::*;
use serde::{Deserialize};

pub struct Coinbase;

impl Coinbase {
    pub fn new() -> Self {
        Coinbase { }
    }
}

#[derive(Deserialize, Debug)]
struct Base {
    id: Option<String>,
    base_currency: Option<String>,
    quote_currency: Option<String>,
}

impl symbol::Collector for Coinbase {
    fn collect(&self) -> Result<symbol::Symbols, error::Error> {
        reqwest::blocking::Client::new().get("https://api.pro.coinbase.com/products").header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.169 Safari/537.36").send()
            .map_err(|_| error::Error::HTTPError())?
            .json::<std::vec::Vec<Base>>()
            .map_err(|_| error::Error::ParseError())
            .and_then(|x| {
                let a = x.
                    into_iter().
                    fold(std::vec::Vec::<symbol::Symbol>::new(),
                         |mut symbols, coinbase_symbol| {
                             if let(Some(id), Some(based), Some(quoted)) = (coinbase_symbol.id, coinbase_symbol.base_currency, coinbase_symbol.quote_currency) {
                                 symbols.push(
                                     symbol::Symbol::new(
                                         id,
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
}