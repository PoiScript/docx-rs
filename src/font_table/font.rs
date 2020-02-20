use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter,
    font_table::{Charset, Family, Pitch},
};

/// Font
///
/// ```rust
/// use docx::font_table::*;
///
/// let font = Font::new("Arial")
///     .charset("00")
///     .family("swiss")
///     .pitch("variable");
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:font")]
pub struct Font<'a> {
    #[xml(attr = "w:name")]
    pub name: Cow<'a, str>,
    #[xml(child = "w:charset")]
    pub charset: Option<Charset<'a>>,
    #[xml(child = "w:family")]
    pub family: Option<Family<'a>>,
    #[xml(child = "w:pitch")]
    pub pitch: Option<Pitch<'a>>,
}

impl<'a> Font<'a> {
    __setter!(charset: Option<Charset<'a>>);
    __setter!(family: Option<Family<'a>>);
    __setter!(pitch: Option<Pitch<'a>>);

    pub fn new<T: Into<Cow<'a, str>>>(name: T) -> Self {
        Font {
            name: name.into(),
            ..Default::default()
        }
    }
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for Font<'a> {
    fn from(val: T) -> Self {
        Font::new(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Font,
        Font::new("Arial"),
        r#"<w:font w:name="Arial"></w:font>"#,
        Font::new("Arial").charset("00"),
        r#"<w:font w:name="Arial"><w:charset w:val="00"/></w:font>"#,
        Font::new("Arial").family("swiss"),
        r#"<w:font w:name="Arial"><w:family w:val="swiss"/></w:font>"#,
        Font::new("Arial").pitch("variable"),
        r#"<w:font w:name="Arial"><w:pitch w:val="variable"/></w:font>"#,
    );
}
