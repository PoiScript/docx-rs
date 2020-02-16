use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter,
    error::{Error, Result},
    formatting::{Borders, Justification, NumberingProperty},
};

/// Paragraph Property
///
/// ```rust
/// use docx::formatting::{ParagraphProperty, JustificationVal};
///
/// ParagraphProperty::default()
///     .style_id("foo")
///     .justification(JustificationVal::Start)
///     .numbering((10usize, 20usize));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:pPr")]
pub struct ParagraphProperty<'a> {
    /// Specifies the style ID of the paragraph style.
    #[xml(child = "w:pStyle")]
    pub style_id: Option<ParagraphStyleId<'a>>,
    /// Specifies the paragraph alignment.
    #[xml(child = "w:jc")]
    pub justification: Option<Justification>,
    /// Specifies borders for the paragraph.
    #[xml(child = "w:pBdr")]
    pub border: Option<Borders<'a>>,
    /// Specifies that the paragraph should be numbered.
    #[xml(child = "w:numPr")]
    pub numbering: Option<NumberingProperty>,
}

impl<'a> ParagraphProperty<'a> {
    __setter!(style_id: Option<ParagraphStyleId<'a>>);
    __setter!(justification: Option<Justification>);
    __setter!(border: Option<Borders<'a>>);
    __setter!(numbering: Option<NumberingProperty>);
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:pStyle")]
pub struct ParagraphStyleId<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for ParagraphStyleId<'a> {
    fn from(val: T) -> Self {
        ParagraphStyleId { value: val.into() }
    }
}
