use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::error::{Error, Result};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:color")]
pub struct Color<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a> Color<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(value: S) -> Self {
        Color {
            value: value.into(),
        }
    }
}
