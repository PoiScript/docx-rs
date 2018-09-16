use std::error;
use std::fmt;

use quick_xml::Error as XmlError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use zip::result::ZipError;

#[derive(Debug)]
pub enum Error {
  Xml(XmlError),
  Zip(ZipError),
  Utf8(Utf8Error),
  UnexpectedEof,
  UnexpectedTag { expected: String, found: String },
  UnexpectedEvent { expected: String, found: String },
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
    match *self {
      Error::Xml(ref err) => write!(f, "{}", err),
      Error::Zip(ref err) => write!(f, "{}", err),
      Error::Utf8(ref err) => write!(f, "{}", err),
      Error::UnexpectedEof => write!(f, "Unexpected Eof"),
      Error::UnexpectedTag {
        ref expected,
        ref found,
      } => write!(f, "Expecting </{}> found </{}>", expected, found),
      Error::UnexpectedEvent {
        ref expected,
        ref found,
      } => write!(f, "Expecting {} event found {} event", expected, found),
    }
  }
}

impl error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::Xml(_) => "Xml Error",
      Error::Zip(ref err) => err.description(),
      Error::Utf8(ref err) => err.description(),
      Error::UnexpectedEof => "Unexpected Eof",
      Error::UnexpectedTag { .. } => "Unexpted Tag",
      Error::UnexpectedEvent { .. } => "Unexpted Event",
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      Error::Xml(ref err) => match err {
        XmlError::Io(ref err) => Some(err as &error::Error),
        XmlError::Utf8(ref err) => Some(err as &error::Error),
        _ => None,
      },
      Error::Utf8(ref err) => Some(err as &error::Error),
      Error::Zip(ref err) => Some(err),
      Error::UnexpectedEof => None,
      Error::UnexpectedTag { .. } => None,
      Error::UnexpectedEvent { .. } => None,
    }
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

pub type Result<T> = ::std::result::Result<T, Error>;
