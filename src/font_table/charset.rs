use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:charset")]
pub struct Charset<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, S: Into<Cow<'a, str>>> From<S> for Charset<'a> {
    fn from(s: S) -> Self {
        Charset { value: s.into() }
    }
}
