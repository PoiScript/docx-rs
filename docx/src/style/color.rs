use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::error::{Error, Result};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:color")]
pub struct Color<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, S: Into<Cow<'a, str>>> From<S> for Color<'a> {
    fn from(val: S) -> Self {
        Color { value: val.into() }
    }
}
