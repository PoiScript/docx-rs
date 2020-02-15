use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:numPr")]
pub struct Numbers {
    /// Specifies a reference to a numbering definition instance
    #[xml(child = "w:numId")]
    pub id: NumId,
    /// Specifies the numbering level of the numbering definition to use for the paragraph.
    #[xml(child = "w:ilvl")]
    pub level: NumLvl,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:numId")]
pub struct NumId {
    #[xml(attr = "w:val")]
    pub value: usize,
}

impl<T: Into<usize>> From<T> for NumId {
    fn from(val: T) -> Self {
        NumId { value: val.into() }
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:ilvl")]
pub struct NumLvl {
    #[xml(attr = "w:val")]
    pub value: usize,
}

impl<T: Into<usize>> From<T> for NumLvl {
    fn from(val: T) -> Self {
        NumLvl { value: val.into() }
    }
}
