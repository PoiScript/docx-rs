use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __setter,
    error::{Error, Result},
    formatting::{BottomBorder, TopBorder},
};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:tblBorders")]
pub struct TableBorders<'a> {
    #[xml(child = "w:top")]
    pub top: Option<TopBorder<'a>>,
    #[xml(child = "w:bottom")]
    pub bottom: Option<BottomBorder<'a>>,
}

impl<'a> TableBorders<'a> {
    __setter!(top: Option<TopBorder<'a>>);
    __setter!(bottom: Option<BottomBorder<'a>>);
}
