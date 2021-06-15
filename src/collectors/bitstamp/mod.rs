use crate::collectors::*;
use serde::{Deserialize};



pub struct Bitstamp;

impl Bitstamp {
    pub fn new() -> Self {
        Bitstamp { }
    }
}

#[derive(Deserialize, Debug)]
struct Base {
    name: Option<String>,
    url_symbol: Option<String>,
}


impl symbol::Collector for Bitstamp {
    fn collect(&self) -> Result<symbol::Symbols, error::Error> {
        reqwest::blocking::Client::new().get("https://www.bitstamp.net/api/v2/trading-pairs-info/")
            .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.169 Safari/537.36")
            .send()
            .map_err(|_| error::Error::HTTPError())?
            .json::<std::vec::Vec<Base>>()
            .map_err(|_| error::Error::ParseError())
            .and_then(|x| {
                let a = x.
                    into_iter().
                    fold(std::vec::Vec::<symbol::Symbol>::new(),
                         |mut symbols, bitstamp_symbol| {
                             if let (Some(name), Some(url_symbol)) = (bitstamp_symbol.name, bitstamp_symbol.url_symbol) {
                                 let parts = name.split("/").collect::<Vec<_>>();

                                 if parts.len() == 2 {
                                     symbols.push(
                                         symbol::Symbol::new(
                                             url_symbol,
                                             parts[0].to_string(),
                                             parts[1].to_string(),
                                             "".to_string()));
                                 }
                             }

                             symbols
                         });

                Ok(symbol::Symbols::new(a))
            })
    }
}