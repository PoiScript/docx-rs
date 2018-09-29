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

fn document_extend_attrs(_: &Document, start: &mut BytesStart) {
  start.push_attribute(("xmlns:w", SCHEMA_MAIN));
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
  pub fn text(&mut self, text: &'a str) -> &mut Self {
    self.content.push(RunContent::Text(TextRun {
      space: None,
      text: Cow::Borrowed(text),
    }));
    self
  }

  /// Returns the properties of this run.
  pub fn prop(&mut self) -> &mut CharStyle<'a> {
    self.prop.get_or_insert(CharStyle::default())
  }

  /// Appends a break to the back of this run.
  pub fn text_break(&mut self) -> &mut Self {
    self.content.push(RunContent::Break(BreakRun { ty: None }));
    self
  }
}

/// A set of elements that can be contained as the content of a run.
#[derive(Debug, Xml)]
pub enum RunContent<'a> {
  #[xml(event = "Start")]
  #[xml(tag = "w:t")]
  Text(TextRun<'a>),
  #[xml(event = "Empty")]
  #[xml(tag = "w:br")]
  Break(BreakRun),
}

/// The root element of a literal text that shall be displayed in the document
#[derive(Debug, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:t")]
pub struct TextRun<'a> {
  /// Specifies how to handle whitespace
  #[xml(attr = "xml:space")]
  pub space: Option<TextSpace>,
  /// Specifies a literal text
  #[xml(text)]
  pub text: Cow<'a, str>,
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
pub struct BreakRun {
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
  pub runs: Vec<Run<'a>>,
}

impl<'a> Para<'a> {
  /// Appends a run to the back of this paragraph, and returns it.
  pub fn new_run(&mut self) -> &mut Run<'a> {
    self.runs.push(Run::default());
    self.runs.last_mut().unwrap()
  }

  /// Returns the properties of this paragraph.
  pub fn prop(&mut self) -> &mut ParaStyle<'a> {
    self.prop.get_or_insert(ParaStyle::default())
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
