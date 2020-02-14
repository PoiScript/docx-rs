use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:strike")]
pub struct Strike {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl Strike {
    pub fn new(value: bool) -> Self {
        Strike { value }
    }
}
