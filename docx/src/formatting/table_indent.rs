use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __string_enum,
    error::{Error, Result},
};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
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

    #[test]
    fn test_convert() {
        assert_eq!(
            TableIndent {
                value: Some(42),
                unit: None
            },
            42.into(),
        );
        assert_eq!(
            TableIndent {
                value: None,
                unit: Some(TableIndentUnit::Pct)
            },
            TableIndentUnit::Pct.into(),
        );
        assert_eq!(
            TableIndent {
                value: Some(42),
                unit: Some(TableIndentUnit::Dxa)
            },
            (42, TableIndentUnit::Dxa).into(),
        );
    }
}
