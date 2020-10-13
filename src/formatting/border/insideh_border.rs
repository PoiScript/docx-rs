use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:insideH")]
pub struct InsideHBorder<'a> {
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

impl<'a> InsideHBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: Option<BorderStyle>);
}

__xml_test_suites!(
    InsideHBorder,
    InsideHBorder::default(),
    r#"<w:insideH/>"#,
    InsideHBorder::default().color("000000"),
    r#"<w:insideH w:color="000000"/>"#,
    InsideHBorder::default().shadow(false),
    r#"<w:insideH w:shadow="false"/>"#,
    InsideHBorder::default().space(40usize),
    r#"<w:insideH w:space="40"/>"#,
    InsideHBorder::default().size(20usize),
    r#"<w:insideH w:sz="20"/>"#,
    InsideHBorder::default().style(BorderStyle::Dotted),
    r#"<w:insideH w:val="dotted"/>"#,
);
