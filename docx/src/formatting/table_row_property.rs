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
/// let prop = TableRowProperty::default()
///     .justification(TableJustificationVal::Start);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:trPr")]
pub struct TableRowProperty {
    /// Specifies the alignment of the row with respect to the text margins in the section.
    #[xml(child = "w:jc")]
    pub justification: Option<TableJustification>,
}

impl TableRowProperty {
    __setter!(justification: Option<TableJustification>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;
    use crate::formatting::TableJustificationVal;

    __test_read_write!(
        TableRowProperty,
        TableRowProperty::default(),
        r#"<w:trPr></w:trPr>"#,
        TableRowProperty::default().justification(TableJustificationVal::Start),
        r#"<w:trPr><w:jc w:val="start"/></w:trPr>"#,
    );
}
