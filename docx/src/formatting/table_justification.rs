use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __string_enum,
    error::{Error, Result},
};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:tblInd")]
pub struct TableJustification {
    #[xml(attr = "w:val")]
    pub value: Option<TableJustificationVal>,
}

impl From<TableJustificationVal> for TableJustification {
    fn from(val: TableJustificationVal) -> Self {
        TableJustification { value: Some(val) }
    }
}

#[derive(Debug)]
pub enum TableJustificationVal {
    Start,
    End,
    Center,
}

__string_enum! {
    TableJustificationVal {
        Start = "start",
        End = "end",
        Center = "center",
    }
}
