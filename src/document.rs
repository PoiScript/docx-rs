//! Main Document part
//!
//! The corresponding ZIP item is `/word/document.xml`.

use quick_xml::events::BytesStart;
use std::borrow::{Borrow, Cow};
use std::convert::AsRef;
use std::str::FromStr;

use errors::{Error, Result};
use schema::SCHEMA_MAIN;
use style::{CharStyle, ParaStyle};

/// The root element of the main document part.
#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:document")]
#[xml(extend_attrs = "document_extend_attrs")]
pub struct Document<'a> {
  /// Specifies the body of the docment.
  #[xml(child)]
  #[xml(tag = "w:body")]
  pub body: Body<'a>,
}

#[inline]
fn document_extend_attrs(_: &Document, start: &mut BytesStart) {
  start.push_attribute(("xmlns:w", SCHEMA_MAIN));
}

/// The root element of the body of the document.
///
/// This is the main document editing surface.
#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:body")]
pub struct Body<'a> {
  /// Specifies the contents of the body of the document.
  #[xml(child)]
  #[xml(tag = "w:p")]
  pub content: Vec<BodyContent<'a>>,
}

impl<'a> Body<'a> {
  /// Create a paragraph in this body, and returns it.
  pub fn create_para(&mut self) -> &mut Para<'a> {
    self.content.push(BodyContent::Para(Para::default()));
    match self.content.last_mut().unwrap() {
      BodyContent::Para(p) => p,
    }
  }
}

/// A set of elements that can be contained in the body
#[derive(Debug, Xml)]
pub enum BodyContent<'a> {
  #[xml(event = "Start")]
  #[xml(tag = "w:p")]
  Para(Para<'a>),
  // Table,
  // SecProp,
}

/// The root element of a paragraph
///
/// Paragraph is the main block-level container for content.
/// Paragraph begins with a new line.
#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:p")]
pub struct Para<'a> {
  /// Specifies the properties of a paragraph
  ///
  /// This information is applied to all the contents of the paragraph.
  #[xml(child)]
  #[xml(tag = "w:pPr")]
  pub prop: Option<ParaStyle<'a>>,
  /// Specifes the run contents of a paragraph
  ///
  /// Run is a region of text with properties. Each paragraph containes one or more runs.
  #[xml(child)]
  #[xml(tag = "w:r")]
  #[xml(tag = "w:hyperlink")]
  pub content: Vec<ParaContent<'a>>,
}

impl<'a> Para<'a> {
  /// Appends a text to the back of this paragraph.
  ///
  /// Similarly to [`Run.text`], but it will create a new run.
  ///
  /// [`Run.text`]: struct.Run.html#method.text
  pub fn text<T: Into<Text<'a>>>(&mut self, t: T) -> &mut Self {
    self.content.push(ParaContent::Run(Run {
      prop: None,
      content: vec![RunContent::Text(t.into())],
    }));
    self
  }

  /// Appends a run to the back of this paragraph, and returns it.
  pub fn new_run(&mut self) -> &mut Run<'a> {
    self.content.push(ParaContent::Run(Run::default()));
    match self.content.last_mut() {
      Some(ParaContent::Run(r)) => r,
      _ => unreachable!("We just insert a run, so the last element must be a run."),
    }
  }

  /// Returns the properties of this paragraph.
  pub fn prop(&mut self) -> &mut ParaStyle<'a> {
    self.prop.get_or_insert(ParaStyle::default())
  }
}

/// A set of elements that can be contained as the content of a paragraph.
#[derive(Debug, Xml)]
pub enum ParaContent<'a> {
  #[xml(event = "Start")]
  #[xml(tag = "w:r")]
  Run(Run<'a>),
  #[xml(event = "Start")]
  #[xml(tag = "w:hyperlink")]
  Link(Hyperlink<'a>),
  #[xml(event = "Empty")]
  #[xml(tag = "w:bookmarkStart")]
  BookmarkStart(BookmarkStart<'a>),
  #[xml(event = "Empty")]
  #[xml(tag = "w:bookmarkEnd")]
  BookmarkEnd(BookmarkEnd<'a>),
}

/// The empty element that defines the beginning of a bookmark
#[derive(Debug, Default, Xml)]
#[xml(event = "Empty")]
#[xml(tag = "w:bookmarkStart")]
pub struct BookmarkStart<'a> {
  /// Specifies a unique identifier for the bookmark.
  #[xml(attr = "w:id")]
  pub id: Option<Cow<'a, str>>,
  /// Specifies the bookmark name.
  #[xml(attr = "w:name")]
  pub name: Option<Cow<'a, str>>,
}

/// The empty element that defines the end of a bookmark
#[derive(Debug, Default, Xml)]
#[xml(event = "Empty")]
#[xml(tag = "w:bookmarkEnd")]
pub struct BookmarkEnd<'a> {
  /// Specifies a unique identifier for the bookmark.
  #[xml(attr = "w:id")]
  pub id: Option<Cow<'a, str>>,
}

/// The root element of a hyperlink within the paragraph
#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:hyperlink")]
pub struct Hyperlink<'a> {
  /// Specifies the ID of the relationship in the relationships part for an external link.
  #[xml(attr = "r:id")]
  pub id: Option<Cow<'a, str>>,
  /// Specifies the name of a bookmark within the document.
  #[xml(attr = "w:anchor")]
  pub anchor: Option<Cow<'a, str>>,
  #[xml(child)]
  #[xml(tag = "w:r")]
  pub content: Run<'a>,
}

/// The root element of a run within the paragraph
///
/// Run is a non-block region of text with properties.
#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:r")]
pub struct Run<'a> {
  /// Specifies the properties of a run
  ///
  /// Just as paragraph, a run's properties is applied to all the contents of the run.
  #[xml(child)]
  #[xml(tag = "w:rPr")]
  pub prop: Option<CharStyle<'a>>,
  #[xml(child)]
  #[xml(tag = "w:t")]
  #[xml(tag = "w:br")]
  /// Specifies the content of a run
  pub content: Vec<RunContent<'a>>,
}

impl<'a> Run<'a> {
  /// Appends a text to the back of this run.
  ///
  /// This function accepts a `&str` or a `Text`.
  ///
  /// ```rust
  /// use docx::document::{Run, Text, TextSpace};
  /// use std::borrow::Cow;
  ///
  /// let mut run = Run::default();
  ///
  /// run.text(" Hello, world  ");
  /// run.text(Text::new(
  ///   Cow::Borrowed("  Hello, world  "),
  ///   Some(TextSpace::Preserve),
  /// ));
  /// ```
  pub fn text<T: Into<Text<'a>>>(&mut self, t: T) -> &mut Self {
    self.content.push(RunContent::Text(t.into()));
    self
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
#[derive(Debug, Xml)]
pub enum RunContent<'a> {
  #[xml(event = "Start")]
  #[xml(tag = "w:t")]
  Text(Text<'a>),
  #[xml(event = "Empty")]
  #[xml(tag = "w:br")]
  Break(Break),
}

/// The root element of a literal text that shall be displayed in the document
#[derive(Debug, Xml)]
#[xml(event = "Start")]
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
  pub fn new(text: Cow<'a, str>, space: Option<TextSpace>) -> Self {
    Text { text, space }
  }
}

impl<'a> From<&'a str> for Text<'a> {
  fn from(s: &'a str) -> Self {
    Text {
      space: None,
      text: Cow::Borrowed(s),
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

string_enum! {
  TextSpace {
    Default = "default",
    Preserve = "preserve",
  }
}

/// The root element of a break
#[derive(Debug, Xml)]
#[xml(event = "Empty")]
#[xml(tag = "w:br")]
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

string_enum! {
  BreakType {
    Column = "column",
    Page = "page",
    TextWrapping = "textWrapping",
  }
}
