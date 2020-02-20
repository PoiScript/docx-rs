use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
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
