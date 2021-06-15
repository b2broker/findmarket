#[macro_use] extern crate prettytable;

use crate::collectors::symbol::{Collector, Symbols, Symbol};
use crate::collectors::error::Error;
use prettytable::{Table, Row, Cell};
use clap::{Arg, App};

mod collectors;


pub fn search(symbol: String, data: &Symbols) -> Symbols {
    if symbol.contains("/") {
        let parts = symbol.split("/").collect::<std::vec::Vec::<&str>>();
        let new_symbol = Symbol::new(
            "".to_string(),
            parts[0].to_string(),
            parts[1].to_string(),
            "".to_string());

        let output = data.data().into_iter().fold(std::vec::Vec::<Symbol>::new(),
        |mut symbols, input| {
            if new_symbol.market().eq(input.market().as_str()) {
                symbols.push(input.clone());
            }

            symbols
        });

        return Symbols::new(output);
    } else {
        let output = data.data().into_iter().fold(std::vec::Vec::<Symbol>::new(),
        |mut symbols, input| {
            if input.based.eq(symbol.as_str()) || input.quoted.eq(symbol.as_str()) {
                symbols.push(input.clone());
            }

            symbols
        });

        return Symbols::new(output);
    }
}

fn main() {
    let app = App::new("Findmarket")
        .version("0.1-beta")
        // .author("Yury K. <crxfoz@gmail.com>")
        // .about("its magic")
        .arg(Arg::new("markets")
            .short('m')
            .long("markets")
            .about("List of markets to find symbols. Supported values: poloniex, binance, binanceus, coinbase, bitstamp, kraken")
            // .required(true)
            // .min_values(1)
            .multiple(true)
            .use_delimiter(true))
        .arg(Arg::new("symbols")
            .short('s')
            .long("symbols")
            .about("List of symbols to find")
            .required(true)
            .min_values(1)
            .multiple(true)
            .use_delimiter(true)
        ).get_matches();



    let mut providers = std::vec::Vec::<(String, Result<collectors::symbol::Symbols, collectors::error::Error>)>::new();

    let markets = match app.values_of("markets") {
        None => {
            vec!("binance", "coinbase", "bitstamp", "kraken", "poloniex", "binanceus")
        },
        Some(list) => list.collect()
    };


    let symbols: Vec<_> = app.values_of("symbols")
        .expect("invalide args")
        .collect();

    for market in markets {
        match market {
            "binance" => providers.push(
                ("binance".to_string(),
                 collectors::binance::Binance::new().collect())),
            "coinbase" => providers.push(
                ("coinbase".to_string(),
                 collectors::coinbase::Coinbase::new().collect())),
            "bitstamp" => providers.push(
                ("bitstamp".to_string(),
                 collectors::bitstamp::Bitstamp::new().collect())),
            "kraken" => providers.push(
                ("kraken".to_string(),
                 collectors::kraken::Kraken::new().collect())),
            "poloniex" => providers.push(
                ("poloniex".to_string(),
                 collectors::poloniex::Poloniex::new().collect())),
            "binanceus" => providers.push(
                ("binanceus".to_string(),
                 collectors::binance::BinanceUS::new().collect())),
            _ => panic!("non supported market")
        }
    }


    let mut table = Table::new();
    table.add_row(row!["Source", "Symbol", "Alias", "Market", "Query"]);

    for provider in providers {
        let provider_name = provider.0.as_str();

        // check errors happen while parsing and continue
        let pprov = match &provider.1 {
            Ok(x) => x,
            Err(err) => {
                match err {
                    Error::ParseError() => println!("{}", format!("[{}] [warning] Parsing error", &provider_name)),
                    Error::HTTPError() => println!("{}", format!("[{}] [warning] HTTP Error", &provider_name)),
                }

                continue
            }
        };

        // build table
        for symbol in &symbols {
            let output = search(symbol.to_string(), pprov);
            if output.data().len() == 0 {
                println!("{}", format!("[{}] [{}] Not found", provider_name, symbol));
                continue
            }

            for item in output.data().iter() {
                table.add_row(Row::new(vec![
                    Cell::new(provider_name),
                    Cell::new(item.name.as_str()),
                    Cell::new(item.alias.as_str()),
                    Cell::new(item.market().as_str()),
                    Cell::new(symbol),
                ]));
            }
        }
    }

    table.printstd();
}
