//! Error module

use std::error;
use std::fmt;

use quick_xml::Error as XmlError;
use std::io::Error as IOError;
use std::num::ParseIntError;
use std::str::{ParseBoolError, Utf8Error};
use std::string::FromUtf8Error;
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
  UnexpectedTag { expected: String, found: String },
  UnexpectedEvent { expected: String, found: String },
  MissingField { name: String, field: String },
  UnknownValue { expected: String, found: String },
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
    match *self {
      Error::IO(ref err) => write!(f, "{}", err),
      Error::Xml(ref err) => write!(f, "{}", err),
      Error::Zip(ref err) => write!(f, "{}", err),
      Error::Utf8(ref err) => write!(f, "{}", err),
      Error::ParseInt(ref err) => write!(f, "{}", err),
      Error::ParseBool(ref err) => write!(f, "{}", err),
      Error::UnexpectedEof => write!(f, "Unexpected Eof"),
      Error::UnexpectedTag {
        ref expected,
        ref found,
      } => write!(f, "Expecting </{}> found </{}>", expected, found),
      Error::UnexpectedEvent {
        ref expected,
        ref found,
      } => write!(f, "Expecting {} event found {} event", expected, found),
      Error::MissingField {
        ref name,
        ref field,
      } => write!(f, "Missing field '{}' when parsing {}.", field, name),
      Error::UnknownValue {
        ref expected,
        ref found,
      } => write!(f, "Unknown value '{}' when parsing {}.", expected, found),
    }
  }
}

impl error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::IO(ref err) => err.description(),
      Error::Zip(ref err) => err.description(),
      Error::Utf8(ref err) => err.description(),
      Error::ParseInt(ref err) => err.description(),
      Error::ParseBool(ref err) => err.description(),
      Error::Xml(_) => "Xml Error",
      Error::UnexpectedEof => "Unexpected Eof",
      Error::UnexpectedTag { .. } => "Unexpted Tag",
      Error::UnexpectedEvent { .. } => "Unexpted Event",
      Error::MissingField { .. } => "Missing Field",
      Error::UnknownValue { .. } => "Unknown Value",
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      Error::IO(ref err) => Some(err),
      Error::Xml(ref err) => match err {
        XmlError::Io(ref err) => Some(err as &error::Error),
        XmlError::Utf8(ref err) => Some(err as &error::Error),
        _ => None,
      },
      Error::ParseInt(ref err) => Some(err),
      Error::ParseBool(ref err) => Some(err),
      Error::Utf8(ref err) => Some(err as &error::Error),
      Error::Zip(ref err) => Some(err),
      Error::UnexpectedEof => None,
      Error::UnexpectedTag { .. } => None,
      Error::UnexpectedEvent { .. } => None,
      Error::MissingField { .. } => None,
      Error::UnknownValue { .. } => None,
    }
  }
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
