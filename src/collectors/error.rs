use serde::export::Formatter;


#[derive(Debug)]
pub enum Error {
    HTTPError(),
    ParseError(),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HTTPError() => write!(f, "could not reach url"),
            Error::ParseError() => write!(f, "could not convert body"),
        }
    }
}
