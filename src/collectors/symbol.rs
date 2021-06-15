<<<<<<< HEAD
use crate::collectors::Error;

trait Collector {
    fn collect(&self) -> Result<Symbols, Error>;
}


struct Symbol {
    name: String,
    based: String,
    quoted: String,
}

impl Symbol {
    pub fn new(name: String, based: String, quoted: String) -> Self {
        Symbol { name, based, quoted }
=======
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
>>>>>>> pew
    }

    pub fn market(&self) -> String {
        format!("{}/{}", self.based, self.quoted)
    }
}

<<<<<<< HEAD
struct Symbols (std::vec::Vec<Symbol>);

=======
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
>>>>>>> pew
