use derive_more::From;
use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

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

impl<'a> Body<'a> {
    pub fn push<T: Into<BodyContent<'a>>>(&mut self, content: T) -> &mut Self {
        self.content.push(content.into());
        self
    }

    pub fn iter_text(&self) -> impl Iterator<Item = &Cow<'a, str>> {
        self.content
            .iter()
            .filter_map(|content| match content {
                BodyContent::Paragraph(para) => Some(para.iter_text()),
            })
            .flatten()
    }

    pub fn iter_text_mut(&mut self) -> impl Iterator<Item = &mut Cow<'a, str>> {
        self.content
            .iter_mut()
            .filter_map(|content| match content {
                BodyContent::Paragraph(para) => Some(para.iter_text_mut()),
            })
            .flatten()
    }
}

/// A set of elements that can be contained in the body
#[derive(Debug, From, XmlRead, XmlWrite, IntoOwned)]
pub enum BodyContent<'a> {
    #[xml(tag = "w:p")]
    Paragraph(Paragraph<'a>),
    // Table,
    // SecProp,
}
