use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:between")]
pub struct BetweenBorder<'a> {
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

impl<'a> BetweenBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: Option<BorderStyle>);
}

__xml_test_suites!(
    BetweenBorder,
    BetweenBorder::default(),
    r#"<w:between/>"#,
    BetweenBorder::default().color("000000"),
    r#"<w:between w:color="000000"/>"#,
    BetweenBorder::default().shadow(false),
    r#"<w:between w:shadow="false"/>"#,
    BetweenBorder::default().space(40usize),
    r#"<w:between w:space="40"/>"#,
    BetweenBorder::default().size(20usize),
    r#"<w:between w:sz="20"/>"#,
    BetweenBorder::default().style(BorderStyle::Dotted),
    r#"<w:between w:val="dotted"/>"#,
);
