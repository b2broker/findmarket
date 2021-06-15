use crate::collectors::error::Error;

pub trait Collector {
    fn collect(&self) -> Result<Symbols, Error>;
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub alias: String,
    pub name: String,
    pub based: String,
    pub quoted: String,
}

impl Symbol {
    pub fn new(name: String, based: String, quoted: String, alias: String) -> Self {
        Symbol { name, based, quoted, alias }
    }

    pub fn market(&self) -> String {
        format!("{}/{}", self.based, self.quoted)
    }
}

#[derive(Debug)]
pub struct Symbols (std::vec::Vec<Symbol>);

impl Symbols {
    pub fn new(data: Vec<Symbol>) -> Self {
        Symbols(data)
    }

    pub fn data(&self) -> &std::vec::Vec<Symbol> {
        &self.0
    }
}
