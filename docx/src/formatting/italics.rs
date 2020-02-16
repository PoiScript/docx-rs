use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:i")]
pub struct Italics {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl<T: Into<bool>> From<T> for Italics {
    fn from(value: T) -> Self {
        Italics {
            value: value.into(),
        }
    }
}
