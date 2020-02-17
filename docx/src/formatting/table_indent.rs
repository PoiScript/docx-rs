use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __string_enum,
    error::{Error, Result},
};

/// Table Indent
///
/// ```rust
/// use docx::formatting::*;
///
/// let ind = TableIndent::from(42);
/// let ind = TableIndent::from(TableIndentUnit::Pct);
/// let ind = TableIndent::from((42, TableIndentUnit::Dxa));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:tblInd")]
pub struct TableIndent {
    #[xml(attr = "w:w")]
    pub value: Option<usize>,
    #[xml(attr = "w:type")]
    pub unit: Option<TableIndentUnit>,
}

impl From<usize> for TableIndent {
    fn from(val: usize) -> Self {
        TableIndent {
            value: Some(val),
            unit: None,
        }
    }
}

impl From<TableIndentUnit> for TableIndent {
    fn from(val: TableIndentUnit) -> Self {
        TableIndent {
            value: None,
            unit: Some(val),
        }
    }
}

impl From<(usize, TableIndentUnit)> for TableIndent {
    fn from(val: (usize, TableIndentUnit)) -> Self {
        TableIndent {
            value: Some(val.0),
            unit: Some(val.1),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TableIndentUnit {
    Auto,
    Dxa,
    Nil,
    Pct,
}

__string_enum! {
    TableIndentUnit {
        Auto = "auto",
        Dxa = "dxa",
        Nil = "nil",
        Pct = "pct",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        TableIndent,
        TableIndent::default(),
        "<w:tblInd/>",
        TableIndent::from(42),
        r#"<w:tblInd w:w="42"/>"#,
        TableIndent::from(TableIndentUnit::Pct),
        r#"<w:tblInd w:type="pct"/>"#,
        TableIndent::from((42, TableIndentUnit::Dxa)),
        r#"<w:tblInd w:w="42" w:type="dxa"/>"#,
    );
}
