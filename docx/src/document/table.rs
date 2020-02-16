use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __setter,
    document::{TableGrid, TableRow},
    error::{Error, Result},
    formatting::TableProperty,
};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:tbl")]
pub struct Table<'a> {
    #[xml(child = "w:tblPr")]
    pub prop: Option<TableProperty<'a>>,
    #[xml(child = "w:tblGrid")]
    pub grids: Vec<TableGrid>,
    #[xml(child = "w:tr")]
    pub rows: Vec<TableRow<'a>>,
}

impl<'a> Table<'a> {
    __setter!(prop: Option<TableProperty<'a>>);
    __setter!(grids: Vec<TableGrid>);
    __setter!(rows: Vec<TableRow<'a>>);
}
