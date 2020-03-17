use strong_xml::{XmlRead, XmlWrite};

use crate::{__string_enum, __xml_test_suites};

/// Table Width
///
/// ```rust
/// use docx::formatting::*;
///
/// let width = TableWidth::from(42usize);
/// let width = TableWidth::from(TableWidthUnit::Pct);
/// let width = TableWidth::from((42, TableWidthUnit::Dxa));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tblW")]
pub struct TableWidth {
    #[xml(attr = "w:w")]
    pub value: Option<usize>,
    #[xml(attr = "w:type")]
    pub unit: Option<TableWidthUnit>,
}

impl From<usize> for TableWidth {
    fn from(val: usize) -> Self {
        TableWidth {
            value: Some(val),
            unit: None,
        }
    }
}

impl From<TableWidthUnit> for TableWidth {
    fn from(val: TableWidthUnit) -> Self {
        TableWidth {
            value: None,
            unit: Some(val),
        }
    }
}

impl From<(usize, TableWidthUnit)> for TableWidth {
    fn from(val: (usize, TableWidthUnit)) -> Self {
        TableWidth {
            value: Some(val.0),
            unit: Some(val.1),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TableWidthUnit {
    Auto,
    Dxa,
    Nil,
    Pct,
}

__string_enum! {
    TableWidthUnit {
        Auto = "auto",
        Dxa = "dxa",
        Nil = "nil",
        Pct = "pct",
    }
}

__xml_test_suites!(
    TableWidth,
    TableWidth::default(),
    "<w:tblW/>",
    TableWidth::from(42),
    r#"<w:tblW w:w="42"/>"#,
    TableWidth::from(TableWidthUnit::Pct),
    r#"<w:tblW w:type="pct"/>"#,
    TableWidth::from((42, TableWidthUnit::Dxa)),
    r#"<w:tblW w:w="42" w:type="dxa"/>"#,
);
