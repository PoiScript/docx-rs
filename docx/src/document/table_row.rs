use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __setter,
    document::TableCell,
    error::{Error, Result},
    formatting::TableRowProperty,
};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:tr")]
pub struct TableRow<'a> {
    #[xml(child = "w:trPr")]
    pub prop: Option<TableRowProperty>,
    #[xml(child = "w:tc")]
    pub cells: Vec<TableCell<'a>>,
}

impl<'a> TableRow<'a> {
    __setter!(prop: Option<TableRowProperty>);
}
