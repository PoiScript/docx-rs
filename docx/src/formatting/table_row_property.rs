use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __setter,
    error::{Error, Result},
    formatting::TableJustification,
};

/// Table Row Property
///
/// ```rust
/// use docx::formatting::{TableRowProperty, TableJustificationVal};
///
/// TableRowProperty::default()
///     .justification(TableJustificationVal::Start);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:trPr")]
pub struct TableRowProperty {
    /// Specifies the alignment of the row with respect to the text margins in the section.
    #[xml(child = "w:jc")]
    pub justification: Option<TableJustification>,
}

impl TableRowProperty {
    __setter!(justification: Option<TableJustification>);
}
