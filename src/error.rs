use std::fmt::{Display, Formatter};
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    EscSeqParseError(String),
    Utf8ParseError(Vec<u8>),
    BadKeyError(u8)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            IoError(err) => write!(f, "Input/output error: {err}"),
            EscSeqParseError(seq) => write!(f, "Error parsing escape sequence: {seq}"),
            Utf8ParseError(bytes) => write!(f, "Byte sequence is not UTF-8: {bytes:?}"),
            BadKeyError(key) => write!(f, "Unexpected or invalid key: {key}")
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::Utf8ParseError(value.into_bytes())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
