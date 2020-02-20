use derive_more::From;
use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter,
    document::{BookmarkEnd, BookmarkStart, Hyperlink, Run, RunContent, Text},
    formatting::ParagraphProperty,
};

/// Paragraph
///
/// Paragraph is the main block-level container for content.
///
/// ```rust
/// use docx::document::*;
/// use docx::formatting::*;
///
/// let par = Paragraph::default()
///     .prop(ParagraphProperty::default())
///     .push_text("hello,")
///     .push_text((" world.", TextSpace::Preserve))
///     .push(Run::default())
///     .push(BookmarkStart::default())
///     .push(BookmarkEnd::default());
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
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
    #[xml(
        child = "w:r",
        child = "w:hyperlink",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd"
    )]
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
#[derive(Debug, From, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Paragraph,
        Paragraph::default(),
        r#"<w:p></w:p>"#,
        Paragraph::default().prop(ParagraphProperty::default()),
        r#"<w:p><w:pPr></w:pPr></w:p>"#,
        Paragraph::default().push(Run::default()),
        r#"<w:p><w:r></w:r></w:p>"#,
        Paragraph::default().push(Hyperlink::default()),
        r#"<w:p><w:hyperlink><w:r></w:r></w:hyperlink></w:p>"#,
        Paragraph::default().push(BookmarkStart::default()),
        r#"<w:p><w:bookmarkStart/></w:p>"#,
        Paragraph::default().push(BookmarkEnd::default()),
        r#"<w:p><w:bookmarkEnd/></w:p>"#,
    );
}
