use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter,
    error::{Error, Result},
    formatting::{TableBorders, TableIndent, TableJustification, TableWidth},
};

/// Table Property
///
/// ```rust
/// use docx::formatting::*;
///
/// TableProperty::default()
///     .style_id("foo")
///     .justification(TableJustificationVal::Start)
///     .indent((50, TableIndentUnit::Pct))
///     .width((50, TableWidthUnit::Pct));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:tblPr")]
pub struct TableProperty<'a> {
    #[xml(child = "w:tblStyle")]
    pub style_id: Option<TableStyleId<'a>>,
    #[xml(child = "w:jc")]
    pub justification: Option<TableJustification>,
    #[xml(child = "w:tblBorders")]
    pub borders: Option<TableBorders<'a>>,
    #[xml(child = "w:tblInd")]
    pub indent: Option<TableIndent>,
    #[xml(child = "w:tblW")]
    pub width: Option<TableWidth>,
}

impl<'a> TableProperty<'a> {
    __setter!(style_id: Option<TableStyleId<'a>>);
    __setter!(justification: Option<TableJustification>);
    __setter!(borders: Option<TableBorders<'a>>);
    __setter!(indent: Option<TableIndent>);
    __setter!(width: Option<TableWidth>);
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:tblStyle")]
pub struct TableStyleId<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for TableStyleId<'a> {
    fn from(val: T) -> Self {
        TableStyleId { value: val.into() }
    }
}
