use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::error::{Error, Result};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:pitch")]
pub struct Pitch<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, S: Into<Cow<'a, str>>> From<S> for Pitch<'a> {
    fn from(s: S) -> Self {
        Pitch { value: s.into() }
    }
}
