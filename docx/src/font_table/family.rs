use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::error::{Error, Result};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:family")]
pub struct Family<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a> Family<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(value: S) -> Self {
        Family {
            value: value.into(),
        }
    }
}
