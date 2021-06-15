use crate::collectors::*;
use serde::{Deserialize};


pub struct Kraken;

impl Kraken {
    pub fn new() -> Self {
        Kraken { }
    }

    fn replaces(&self) -> std::collections::HashMap<&str, &str> {
        [
            ("XBT", "BTC"),
            ("XDG", "DOGE"),
        ].iter().cloned().collect()
    }


    fn apply_replace(&self, name: String) -> String {
        for (k, v) in self.replaces().iter() {
            if name.eq(k) {
                return v.to_string()
            }
        }

        return name
    }
}

#[derive(Deserialize, Debug)]
struct Symbol {
    altname: Option<String>,
    wsname: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Base {
    result: std::collections::HashMap<String, Symbol>
}

impl symbol::Collector for Kraken {
    fn collect(&self) -> Result<symbol::Symbols, error::Error> {
        reqwest::blocking::get("https://api.kraken.com/0/public/AssetPairs")
            .map_err(|_| error::Error::HTTPError())?
            .json::<Base>()
            .map_err(|_| error::Error::ParseError())
            .and_then(|x| {
                let a = x.result
                    .into_iter()
                    .fold(std::vec::Vec::<symbol::Symbol>::new(),
                    |mut symbols, kraken_symbol| {


                                // symbols.push(
                                //     symbol::Symbol::new(
                                //         kraken_symbol.1.altname.unwrap(),
                                //         pew.based.clone(),
                                //         pew.quoted.clone(),
                                //         "".to_string())
                                // );
                        symbols
                    });

                Ok(symbol::Symbols::new(a))
            })
    }
}
