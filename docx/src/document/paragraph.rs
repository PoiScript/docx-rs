use derive_more::From;
use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter,
    document::{BookmarkEnd, BookmarkStart, Hyperlink, Run, RunContent, Text},
    error::{Error, Result},
    formatting::ParagraphProperty,
};

/// Paragraph
///
/// Paragraph is the main block-level container for content.
/// Paragraph begins with a new line.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:p")]
pub struct Paragraph<'a> {
    /// Specifies the properties of a paragraph
    ///
    /// This information is applied to all the contents of the paragraph.
    #[xml(child = "w:pPr")]
    pub prop: Option<ParagraphProperty<'a>>,
    /// Specifes the run contents of a paragraph
    ///
    /// Run is a region of text with properties. Each paragraph containes one or more runs.
    #[xml(child = "w:r", child = "w:hyperlink")]
    pub content: Vec<ParagraphContent<'a>>,
}

impl<'a> Paragraph<'a> {
    __setter!(prop: Option<ParagraphProperty<'a>>);

    #[inline(always)]
    pub fn push<T: Into<ParagraphContent<'a>>>(mut self, content: T) -> Self {
        self.content.push(content.into());
        self
    }

    #[inline(always)]
    pub fn push_text<T: Into<Text<'a>>>(mut self, content: T) -> Self {
        self.content.push(ParagraphContent::Run(Run {
            content: vec![RunContent::Text(content.into())],
            ..Default::default()
        }));
        self
    }

    pub fn iter_text(&self) -> impl Iterator<Item = &Cow<'a, str>> {
        self.content
            .iter()
            .filter_map(|content| match content {
                ParagraphContent::Run(run) => Some(run.iter_text()),
                ParagraphContent::Link(link) => Some(link.content.iter_text()),
                _ => None,
            })
            .flatten()
    }

    pub fn iter_text_mut(&mut self) -> impl Iterator<Item = &mut Cow<'a, str>> {
        self.content
            .iter_mut()
            .filter_map(|content| match content {
                ParagraphContent::Run(run) => Some(run.iter_text_mut()),
                ParagraphContent::Link(link) => Some(link.content.iter_text_mut()),
                _ => None,
            })
            .flatten()
    }
}

/// A set of elements that can be contained as the content of a paragraph.
#[derive(Debug, From, XmlRead, XmlWrite, IntoOwned)]
pub enum ParagraphContent<'a> {
    #[xml(tag = "w:r")]
    Run(Run<'a>),
    #[xml(tag = "w:hyperlink")]
    Link(Hyperlink<'a>),
    #[xml(tag = "w:bookmarkStart")]
    BookmarkStart(BookmarkStart<'a>),
    #[xml(tag = "w:bookmarkEnd")]
    BookmarkEnd(BookmarkEnd<'a>),
}
