use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:dstrike")]
pub struct Dstrike {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl<T: Into<bool>> From<T> for Dstrike {
    fn from(val: T) -> Self {
        Dstrike { value: val.into() }
    }
}
