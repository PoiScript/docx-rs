use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:outline")]
pub struct Outline {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl Outline {
    pub fn new(value: bool) -> Self {
        Outline { value }
    }
}
