use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:tcPr")]
pub struct TableCellProperty {}

impl TableCellProperty {}
