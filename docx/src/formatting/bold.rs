use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:bold")]
pub struct Bold {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl From<bool> for Bold {
    fn from(value: bool) -> Self {
        Bold { value }
    }
}
