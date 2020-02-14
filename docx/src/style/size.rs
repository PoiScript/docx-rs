use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:sz")]
pub struct Size {
    #[xml(attr = "w:val")]
    pub value: usize,
}

impl Size {
    pub fn new(value: usize) -> Self {
        Size { value }
    }
}
