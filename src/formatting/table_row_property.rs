use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites, formatting::TableJustification};

/// Table Row Property
///
/// ```rust
/// use docx::formatting::{TableRowProperty, TableJustificationVal};
///
/// let prop = TableRowProperty::default()
///     .justification(TableJustificationVal::Start);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
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

__xml_test_suites!(
    TableRowProperty,
    TableRowProperty::default(),
    r#"<w:trPr/>"#,
    TableRowProperty::default().justification(crate::formatting::TableJustificationVal::Start),
    r#"<w:trPr><w:jc w:val="start"/></w:trPr>"#,
);
