use std::{
    io::Error as IOError,
    num::ParseIntError,
    str::{ParseBoolError, Utf8Error},
    string::FromUtf8Error,
};

use xmlparser::Error as XmlError;
use zip::result::ZipError;

/// Error type of docx-rs
#[derive(Debug)]
pub enum Error {
    IO(IOError),
    Xml(XmlError),
    Zip(ZipError),
    Utf8(Utf8Error),
    ParseInt(ParseIntError),
    ParseBool(ParseBoolError),
    UnexpectedEof,
    UnexpectedToken { token: String },
    TagMismatch { expected: String, found: String },
    MissingField { name: String, field: String },
    UnknownValue { expected: String, found: String },
    UnterminatedEntity { entity: String },
    UnrecognizedSymbol { symbol: String },
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Error::IO(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}

impl From<ParseBoolError> for Error {
    fn from(err: ParseBoolError) -> Self {
        Error::ParseBool(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::Utf8(err.utf8_error())
    }
}

impl From<XmlError> for Error {
    fn from(err: XmlError) -> Self {
        Error::Xml(err)
    }
}

impl From<ZipError> for Error {
    fn from(err: ZipError) -> Self {
        Error::Zip(err)
    }
}

/// Specialized `Result` which the error value is `Error`.
pub type Result<T> = ::std::result::Result<T, Error>;
