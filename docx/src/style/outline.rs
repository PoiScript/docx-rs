use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:outline")]
pub struct Outline {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl<T: Into<bool>> From<T> for Outline {
    fn from(val: T) -> Self {
        Outline { value: val.into() }
    }
}
