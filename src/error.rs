use std::io::Error as IOError;

use strong_xml::XmlError;
use zip::result::ZipError;

/// Error type of docx-rs
#[derive(Debug)]
pub enum DocxError {
    IO(IOError),
    Xml(XmlError),
    Zip(ZipError),
}

impl From<IOError> for DocxError {
    fn from(err: IOError) -> Self {
        DocxError::IO(err)
    }
}

impl From<XmlError> for DocxError {
    fn from(err: XmlError) -> Self {
        DocxError::Xml(err)
    }
}

impl From<ZipError> for DocxError {
    fn from(err: ZipError) -> Self {
        DocxError::Zip(err)
    }
}

/// Specialized `Result` which the error value is `DocxError`.
pub type DocxResult<T> = Result<T, DocxError>;
