use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::error::{Error, Result};

use super::{charset::Charset, family::Family, pitch::Pitch};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:font")]
pub struct Font<'a> {
    #[xml(attr = "w:name")]
    pub name: Cow<'a, str>,
    #[xml(child = "w:val")]
    pub charset: Option<Charset<'a>>,
    #[xml(child = "w:family")]
    pub family: Option<Family<'a>>,
    #[xml(child = "w:pitch")]
    pub pitch: Option<Pitch<'a>>,
}
