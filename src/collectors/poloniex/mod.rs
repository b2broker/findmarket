use crate::collectors::*;
use serde::{Deserialize};


pub struct Poloniex;

impl Poloniex {
    pub fn new() -> Self {
        Poloniex { }
    }
}

#[derive(Deserialize, Debug)]
struct Base {
    id: Option<i32>
}

impl symbol::Collector for Poloniex {
    fn collect(&self) -> Result<symbol::Symbols, error::Error> {
        reqwest::blocking::get("https://poloniex.com/public?command=returnTicker")
            .map_err(|_| error::Error::HTTPError())?
            .json::<std::collections::HashMap<String, Base>>()
            .map_err(|_| error::Error::ParseError())
            .and_then(|x| {
                let a = x
                    .into_iter()
                    .fold(std::vec::Vec::<symbol::Symbol>::new(),
                          |mut symbols, poloniex_symbol| {
                              let parts = poloniex_symbol.0.split("_").collect::<Vec<_>>();

                              if poloniex_symbol.1.id.is_some() {
                                  symbols.push(
                                      symbol::Symbol::new(
                                          poloniex_symbol.1.id.unwrap().to_string(),
                                          parts[1].to_string(),
                                          parts[0].to_string(),
                                          poloniex_symbol.0)
                                  );
                              }

                              symbols
                          });

                Ok(symbol::Symbols::new(a))
            })
    }
}
