//! Main Document part
//!
//! The corresponding ZIP item is `/word/document.xml`.

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;
use std::io::Write;

use crate::{
    __string_enum,
    error::{Error, Result},
    schema::SCHEMA_MAIN,
    style::{CharStyle, ParaStyle},
};

/// The root element of the main document part.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:document")]
#[xml(extend_attrs = "document_extend_attrs")]
pub struct Document<'a> {
    /// Specifies the body of the docment.
    #[xml(child = "w:body")]
    pub body: Body<'a>,
}

#[inline]
fn document_extend_attrs<W: Write>(_: &Document, mut w: W) -> Result<()> {
    write!(w, " xmlns:w=\"{}\"", SCHEMA_MAIN)?;
    Ok(())
}

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
    Para(Para<'a>),
    // Table,
    // SecProp,
}

/// The root element of a paragraph
///
/// Paragraph is the main block-level container for content.
/// Paragraph begins with a new line.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:p")]
pub struct Para<'a> {
    /// Specifies the properties of a paragraph
    ///
    /// This information is applied to all the contents of the paragraph.
    #[xml(child = "w:pPr")]
    pub prop: Option<ParaStyle<'a>>,
    /// Specifes the run contents of a paragraph
    ///
    /// Run is a region of text with properties. Each paragraph containes one or more runs.
    #[xml(child = "w:r", child = "w:hyperlink")]
    pub content: Vec<ParaContent<'a>>,
}

impl<'a> Para<'a> {
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
    /// use docx::document::{Para, Text, TextSpace};
    ///
    /// let mut para = Para::default();
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
    /// use docx::document::{Para, Run};
    ///
    /// let mut para = Para::default();
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
    pub fn prop(&mut self) -> &mut ParaStyle<'a> {
        self.prop.get_or_insert(ParaStyle::default())
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

/// The empty element that defines the beginning of a bookmark
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:bookmarkStart")]
pub struct BookmarkStart<'a> {
    /// Specifies a unique identifier for the bookmark.
    #[xml(attr = "w:id")]
    pub id: Option<Cow<'a, str>>,
    /// Specifies the bookmark name.
    #[xml(attr = "w:name")]
    pub name: Option<Cow<'a, str>>,
}

/// The empty element that defines the end of a bookmark
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:bookmarkEnd")]
pub struct BookmarkEnd<'a> {
    /// Specifies a unique identifier for the bookmark.
    #[xml(attr = "w:id")]
    pub id: Option<Cow<'a, str>>,
}

/// The root element of a hyperlink within the paragraph
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:hyperlink")]
pub struct Hyperlink<'a> {
    /// Specifies the ID of the relationship in the relationships part for an external link.
    #[xml(attr = "r:id")]
    pub id: Option<Cow<'a, str>>,
    /// Specifies the name of a bookmark within the document.
    #[xml(attr = "w:anchor")]
    pub anchor: Option<Cow<'a, str>>,
    #[xml(child = "w:r")]
    pub content: Run<'a>,
}

/// The root element of a run within the paragraph
///
/// Run is a non-block region of text with properties.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:r")]
pub struct Run<'a> {
    /// Specifies the properties of a run
    ///
    /// Just as paragraph, a run's properties is applied to all the contents of the run.
    #[xml(child = "w:rPr")]
    pub prop: Option<CharStyle<'a>>,
    #[xml(child = "w:t", child = "w:br")]
    /// Specifies the content of a run
    pub content: Vec<RunContent<'a>>,
}

impl<'a> Run<'a> {
    /// Creates a new run containing the given text
    pub fn text<T: Into<Text<'a>>>(t: T) -> Self {
        Run {
            prop: None,
            content: vec![RunContent::Text(t.into())],
        }
    }

    /// Returns the properties of this run.
    pub fn prop(&mut self) -> &mut CharStyle<'a> {
        self.prop.get_or_insert(CharStyle::default())
    }

    /// Appends a break to the back of this run.
    pub fn text_break(&mut self) -> &mut Self {
        self.content.push(RunContent::Break(Break { ty: None }));
        self
    }
}

/// A set of elements that can be contained as the content of a run.
#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
pub enum RunContent<'a> {
    #[xml(tag = "w:t")]
    Text(Text<'a>),
    #[xml(tag = "w:br")]
    Break(Break),
}

/// The root element of a literal text that shall be displayed in the document
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:t")]
pub struct Text<'a> {
    /// Specifies how to handle whitespace
    #[xml(attr = "xml:space")]
    pub space: Option<TextSpace>,
    /// Specifies a literal text
    #[xml(text)]
    pub text: Cow<'a, str>,
}

impl<'a> Text<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(text: S, space: Option<TextSpace>) -> Self {
        Text {
            text: text.into(),
            space,
        }
    }
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(text: &'a str) -> Self {
        Text {
            space: None,
            text: text.into(),
        }
    }
}

/// Specifies how whitespace should be handled
#[derive(Debug)]
pub enum TextSpace {
    Default,
    /// Using the W3C space preservation rules
    Preserve,
}

impl Default for TextSpace {
    fn default() -> Self {
        TextSpace::Default
    }
}

__string_enum! {
    TextSpace {
        Default = "default",
        Preserve = "preserve",
    }
}

/// The root element of a break
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:br")]
pub struct Break {
    /// Specifies the break type of this break.
    #[xml(attr = "type")]
    pub ty: Option<BreakType>,
}

/// Specifies the break type of a break
///
/// The default value is TextWrapping.
#[derive(Debug)]
pub enum BreakType {
    /// Text restarts on the next column.
    Column,
    /// Text restarts on the next page.
    Page,
    /// Text restarts on the next line.
    TextWrapping,
}

__string_enum! {
    BreakType {
        Column = "column",
        Page = "page",
        TextWrapping = "textWrapping",
    }
}
