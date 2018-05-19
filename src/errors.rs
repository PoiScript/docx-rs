use std::error;
use std::fmt;

use quick_xml::Error as XmlError;
use zip::result::ZipError;

#[derive(Debug)]
pub enum Error {
  Xml(XmlError),
  Zip(ZipError),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
    match *self {
      Error::Xml(ref err) => write!(f, "{}", err),
      Error::Zip(ref err) => write!(f, "{}", err),
    }
  }
}

impl error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::Xml(_) => "Xml Error",
      Error::Zip(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      Error::Xml(ref err) => match err {
        XmlError::Io(ref err) => Some(err as &error::Error),
        XmlError::Utf8(ref err) => Some(err as &error::Error),
        _ => None,
      },
      Error::Zip(ref err) => Some(err),
    }
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
