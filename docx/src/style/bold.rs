use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:bold")]
pub struct Bold {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl Bold {
    pub fn new(value: bool) -> Self {
        Bold { value }
    }
}
