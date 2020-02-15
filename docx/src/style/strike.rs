use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:strike")]
pub struct Strike {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl<T: Into<bool>> From<T> for Strike {
    fn from(val: T) -> Self {
        Strike { value: val.into() }
    }
}
