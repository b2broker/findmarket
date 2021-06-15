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
    }

    pub fn market(&self) -> String {
        format!("{}/{}", self.based, self.quoted)
    }
}

struct Symbols (std::vec::Vec<Symbol>);

