use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:top")]
pub struct TopBorder<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>,
    #[xml(attr = "w:val")]
    pub style: Option<BorderStyle>,
}

impl<'a> TopBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: Option<BorderStyle>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        TopBorder,
        TopBorder::default(),
        r#"<w:top/>"#,
        TopBorder::default().color("000000"),
        r#"<w:top w:color="000000"/>"#,
        TopBorder::default().shadow(false),
        r#"<w:top w:shadow="false"/>"#,
        TopBorder::default().space(40usize),
        r#"<w:top w:space="40"/>"#,
        TopBorder::default().size(20usize),
        r#"<w:top w:sz="20"/>"#,
        TopBorder::default().style(BorderStyle::Dotted),
        r#"<w:top w:val="dotted"/>"#,
    );
}
