use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

use super::paragraph::Paragraph;

/// The root element of the body of the document.
///
/// This is the main document editing surface.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:body")]
pub struct Body<'a> {
    /// Specifies the contents of the body of the document.
    #[xml(child = "w:p")]
    pub content: Vec<BodyContent<'a>>,
}

/// A set of elements that can be contained in the body
#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
pub enum BodyContent<'a> {
    #[xml(tag = "w:p")]
    Paragraph(Paragraph<'a>),
    // Table,
    // SecProp,
}
