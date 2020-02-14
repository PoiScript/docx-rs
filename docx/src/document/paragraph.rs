use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use super::{
    bookmark::{BookmarkEnd, BookmarkStart},
    hyperlink::Hyperlink,
    r#break::Break,
    run::{Run, RunContent},
    text::Text,
};

use crate::{
    error::{Error, Result},
    style::ParagraphStyle,
};

/// The root element of a paragraph
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
    pub prop: Option<ParagraphStyle<'a>>,
    /// Specifes the run contents of a paragraph
    ///
    /// Run is a region of text with properties. Each paragraph containes one or more runs.
    #[xml(child = "w:r", child = "w:hyperlink")]
    pub content: Vec<ParaContent<'a>>,
}

impl<'a> Paragraph<'a> {
    /// Appends a text to the back of this paragraph.
    ///
    /// Similarly to [`Run.text`], but it will create a new run without
    /// any formatting. If you want to insert a styled text, use method
    /// [`run`] instead.
    ///
    /// [`Run.text`]: struct.Run.html#method.text
    /// [`run`]: #method.run
    ///
    /// ```rust
    /// use docx::document::{Paragraph, Text, TextSpace};
    ///
    /// let mut para = Paragraph::default();
    ///
    /// para.text("Hello,");
    /// para.text(Text::new(" world", Some(TextSpace::Preserve)));
    /// ```
    #[inline]
    pub fn text<T: Into<Text<'a>>>(&mut self, text: T) -> &mut Self {
        self.content.push(ParaContent::Run(Run {
            prop: None,
            content: vec![RunContent::Text(text.into())],
        }));
        self
    }

    /// Appends a run to the back of this paragraph
    ///
    /// ```rust
    /// use docx::document::{Paragraph, Run};
    ///
    /// let mut para = Paragraph::default();
    /// let mut run = Run::text("Hello");
    /// run.prop().bold(true);
    ///
    /// para.run(run);
    /// ```
    #[inline]
    pub fn run(&mut self, run: Run<'a>) -> &mut Self {
        self.content.push(ParaContent::Run(run));
        self
    }

    #[inline]
    pub fn text_break(&mut self) -> &mut Self {
        self.content.push(ParaContent::Run(Run {
            prop: None,
            content: vec![RunContent::Break(Break { ty: None })],
        }));
        self
    }

    /// Returns the properties of this paragraph.
    pub fn prop(&mut self) -> &mut ParagraphStyle<'a> {
        self.prop.get_or_insert(ParagraphStyle::default())
    }
}

/// A set of elements that can be contained as the content of a paragraph.
#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
pub enum ParaContent<'a> {
    #[xml(tag = "w:r")]
    Run(Run<'a>),
    #[xml(tag = "w:hyperlink")]
    Link(Hyperlink<'a>),
    #[xml(tag = "w:bookmarkStart")]
    BookmarkStart(BookmarkStart<'a>),
    #[xml(tag = "w:bookmarkEnd")]
    BookmarkEnd(BookmarkEnd<'a>),
}
