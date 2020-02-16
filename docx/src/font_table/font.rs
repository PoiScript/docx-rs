use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter,
    error::{Error, Result},
    font_table::{Charset, Family, Pitch},
};

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

impl<'a> Font<'a> {
    __setter!(name: Cow<'a, str>);
    __setter!(charset: Option<Charset<'a>>);
    __setter!(family: Option<Family<'a>>);
    __setter!(pitch: Option<Pitch<'a>>);
}
