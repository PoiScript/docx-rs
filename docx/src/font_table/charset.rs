use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::error::{Error, Result};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:val")]
pub struct Charset<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a> Charset<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(value: S) -> Self {
        Charset {
            value: value.into(),
        }
    }
}
