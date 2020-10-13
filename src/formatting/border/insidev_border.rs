use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:insideV")]
pub struct InsideVBorder<'a> {
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

impl<'a> InsideVBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: Option<BorderStyle>);
}

__xml_test_suites!(
    InsideVBorder,
    InsideVBorder::default(),
    r#"<w:insideV/>"#,
    InsideVBorder::default().color("000000"),
    r#"<w:insideV w:color="000000"/>"#,
    InsideVBorder::default().shadow(false),
    r#"<w:insideV w:shadow="false"/>"#,
    InsideVBorder::default().space(40usize),
    r#"<w:insideV w:space="40"/>"#,
    InsideVBorder::default().size(20usize),
    r#"<w:insideV w:sz="20"/>"#,
    InsideVBorder::default().style(BorderStyle::Dotted),
    r#"<w:insideV w:val="dotted"/>"#,
);
