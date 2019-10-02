//! Main Document part
//!
//! The corresponding ZIP item is `/word/document.xml`.

use docx_codegen::Xml;
use quick_xml::events::BytesStart;

use crate::{
    __string_enum,
    error::{Error, Result},
    schema::SCHEMA_MAIN,
    style::{CharStyle, ParaStyle},
};

/// The root element of the main document part.
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:document")]
#[xml(extend_attrs = "document_extend_attrs")]
pub struct Document {
    /// Specifies the body of the docment.
    #[xml(child = "w:body")]
    pub body: Body,
}

#[inline]
fn document_extend_attrs(_: &Document, start: &mut BytesStart) {
    start.push_attribute(("xmlns:w", SCHEMA_MAIN));
}

/// The root element of the body of the document.
///
/// This is the main document editing surface.
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:body")]
pub struct Body {
    /// Specifies the contents of the body of the document.
    #[xml(child = "w:p")]
    pub content: Vec<BodyContent>,
}

/// A set of elements that can be contained in the body
#[derive(Debug, Xml)]
pub enum BodyContent {
    #[xml(tag = "w:p")]
    Para(Para),
    // Table,
    // SecProp,
}

/// The root element of a paragraph
///
/// Paragraph is the main block-level container for content.
/// Paragraph begins with a new line.
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:p")]
pub struct Para {
    /// Specifies the properties of a paragraph
    ///
    /// This information is applied to all the contents of the paragraph.
    #[xml(child = "w:pPr")]
    pub prop: Option<ParaStyle>,
    /// Specifes the run contents of a paragraph
    ///
    /// Run is a region of text with properties. Each paragraph containes one or more runs.
    #[xml(child = "w:r", child = "w:hyperlink")]
    pub content: Vec<ParaContent>,
}

impl Para {
    /// Appends a text to the back of this paragraph.
    ///
    /// Similarly to [`Run.text`], but it will create a new run without
    /// any formatting. If you want to insert a styled text, use method
    /// [`run`] instead.
    ///
    /// ```rust
    /// use docx::document::{Para, Text, TextSpace};
    ///
    /// let mut para = Para::default();
    ///
    /// para.text("Hello,");
    /// para.text(Text::new(" world", Some(TextSpace::Preserve)));
    /// ```
    ///
    /// [`Run.text`]: struct.Run.html#method.text
    /// [`run`]: #method.run
    #[inline]
    pub fn text<T: Into<Text>>(&mut self, t: T) -> &mut Self {
        self.content.push(ParaContent::Run(Run {
            prop: None,
            content: vec![RunContent::Text(t.into())],
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
    pub fn run(&mut self, r: Run) -> &mut Self {
        self.content.push(ParaContent::Run(r));
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
    pub fn prop(&mut self) -> &mut ParaStyle {
        self.prop.get_or_insert(ParaStyle::default())
    }
}

/// A set of elements that can be contained as the content of a paragraph.
#[derive(Debug, Xml)]
pub enum ParaContent {
    #[xml(tag = "w:r")]
    Run(Run),
    #[xml(tag = "w:hyperlink")]
    Link(Hyperlink),
    #[xml(tag = "w:bookmarkStart")]
    BookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    BookmarkEnd(BookmarkEnd),
}

/// The empty element that defines the beginning of a bookmark
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:bookmarkStart")]
#[xml(leaf)]
pub struct BookmarkStart {
    /// Specifies a unique identifier for the bookmark.
    #[xml(attr = "w:id")]
    pub id: Option<String>,
    /// Specifies the bookmark name.
    #[xml(attr = "w:name")]
    pub name: Option<String>,
}

/// The empty element that defines the end of a bookmark
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:bookmarkEnd")]
#[xml(leaf)]
pub struct BookmarkEnd {
    /// Specifies a unique identifier for the bookmark.
    #[xml(attr = "w:id")]
    pub id: Option<String>,
}

/// The root element of a hyperlink within the paragraph
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:hyperlink")]
pub struct Hyperlink {
    /// Specifies the ID of the relationship in the relationships part for an external link.
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// Specifies the name of a bookmark within the document.
    #[xml(attr = "w:anchor")]
    pub anchor: Option<String>,
    #[xml(child = "w:r")]
    pub content: Run,
}

/// The root element of a run within the paragraph
///
/// Run is a non-block region of text with properties.
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:r")]
pub struct Run {
    /// Specifies the properties of a run
    ///
    /// Just as paragraph, a run's properties is applied to all the contents of the run.
    #[xml(child = "w:rPr")]
    pub prop: Option<CharStyle>,
    #[xml(child = "w:t", child = "w:br")]
    /// Specifies the content of a run
    pub content: Vec<RunContent>,
}

impl Run {
    /// Creates a new run containing the given text
    pub fn text<T: Into<Text>>(t: T) -> Self {
        Run {
            prop: None,
            content: vec![RunContent::Text(t.into())],
        }
    }

    /// Returns the properties of this run.
    pub fn prop(&mut self) -> &mut CharStyle {
        self.prop.get_or_insert(CharStyle::default())
    }

    /// Appends a break to the back of this run.
    pub fn text_break(&mut self) -> &mut Self {
        self.content.push(RunContent::Break(Break { ty: None }));
        self
    }
}

/// A set of elements that can be contained as the content of a run.
#[derive(Debug, Xml)]
pub enum RunContent {
    #[xml(tag = "w:t")]
    Text(Text),
    #[xml(tag = "w:br")]
    Break(Break),
}

/// The root element of a literal text that shall be displayed in the document
#[derive(Debug, Xml)]
#[xml(tag = "w:t")]
pub struct Text {
    /// Specifies how to handle whitespace
    #[xml(attr = "xml:space")]
    pub space: Option<TextSpace>,
    /// Specifies a literal text
    #[xml(text)]
    pub text: String,
}

impl Text {
    pub fn new<S: Into<String>>(text: S, space: Option<TextSpace>) -> Self {
        Text {
            text: text.into(),
            space,
        }
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Text {
            space: None,
            text: s.to_owned(),
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

__string_enum! {
    TextSpace {
        Default = "default",
        Preserve = "preserve",
    }
}

/// The root element of a break
#[derive(Debug, Xml)]
#[xml(tag = "w:br")]
#[xml(leaf)]
pub struct Break {
    /// Specifies the break type of this break.
    #[xml(attr = "type")]
    ty: Option<BreakType>,
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
