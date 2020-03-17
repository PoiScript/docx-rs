use strong_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tcPr")]
pub struct TableCellProperty {}

impl TableCellProperty {}

__xml_test_suites!(
    TableCellProperty,
    TableCellProperty::default(),
    r#"<w:tcPr/>"#,
);
