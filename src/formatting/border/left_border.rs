use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:left")]
pub struct LeftBorder<'a> {
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

impl<'a> LeftBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: Option<BorderStyle>);
}

__xml_test_suites!(
    LeftBorder,
    LeftBorder::default(),
    r#"<w:left/>"#,
    LeftBorder::default().color("000000"),
    r#"<w:left w:color="000000"/>"#,
    LeftBorder::default().shadow(false),
    r#"<w:left w:shadow="false"/>"#,
    LeftBorder::default().space(40usize),
    r#"<w:left w:space="40"/>"#,
    LeftBorder::default().size(20usize),
    r#"<w:left w:sz="20"/>"#,
    LeftBorder::default().style(BorderStyle::Dotted),
    r#"<w:left w:val="dotted"/>"#,
);
