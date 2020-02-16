use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __string_enum,
    error::{Error, Result},
};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:sz")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TableWidth {
                value: Some(42),
                unit: None
            },
            42.into(),
        );
        assert_eq!(
            TableWidth {
                value: None,
                unit: Some(TableWidthUnit::Pct)
            },
            TableWidthUnit::Pct.into(),
        );
        assert_eq!(
            TableWidth {
                value: Some(42),
                unit: Some(TableWidthUnit::Dxa)
            },
            (42, TableWidthUnit::Dxa).into(),
        );
    }
}
