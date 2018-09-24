use quick_xml::events::BytesStart;
use std::borrow::{Borrow, Cow};
use std::convert::AsRef;
use std::str::FromStr;

use errors::{Error, Result};
use schema::SCHEMA_MAIN;
use style::{CharStyle, ParaStyle};

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:document")]
#[xml(extend_attrs = "document_extend_attrs")]
pub struct Document<'a> {
  #[xml(child)]
  #[xml(tag = "w:body")]
  pub body: Body<'a>,
}

fn document_extend_attrs(_: &Document, start: &mut BytesStart) {
  start.push_attribute(("xmlns:w", SCHEMA_MAIN));
}

// Specifies a run of content within the paragraph.
#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:r")]
pub struct Run<'a> {
  #[xml(child)]
  #[xml(tag = "w:rPr")]
  prop: CharStyle<'a>,
  #[xml(child)]
  #[xml(tag = "w:t")]
  #[xml(tag = "w:br")]
  content: Vec<RunContent<'a>>,
}

impl<'a> Run<'a> {
  pub fn add_text(&mut self, text: &'a str) -> &mut Self {
    self.content.push(RunContent::Text(TextRun {
      text: Cow::Borrowed(text),
    }));
    self
  }
  pub fn add_break(&mut self) -> &mut Self {
    self.content.push(RunContent::Break(BreakRun { ty: None }));
    self
  }
}

#[derive(Debug, Xml)]
pub enum RunContent<'a> {
  #[xml(event = "Start")]
  #[xml(tag = "w:t")]
  Text(TextRun<'a>),
  #[xml(event = "Empty")]
  #[xml(tag = "w:br")]
  Break(BreakRun),
}

#[derive(Debug, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:t")]
pub struct TextRun<'a> {
  #[xml(text)]
  text: Cow<'a, str>,
}

#[derive(Debug, Xml)]
#[xml(event = "Empty")]
#[xml(tag = "w:br")]
pub struct BreakRun {
  #[xml(attr = "type")]
  ty: Option<BreakType>,
}

#[derive(Debug)]
pub enum BreakType {
  Column,
  Page,
  TextWrapping,
}

string_enum! {
  BreakType {
    Column = "column",
    Page = "page",
    TextWrapping = "textWrapping",
  }
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:p")]
pub struct Para<'a> {
  #[xml(child)]
  #[xml(tag = "w:pPr")]
  prop: ParaStyle<'a>,
  // Each paragraph containes one or more runs.
  #[xml(child)]
  #[xml(tag = "w:r")]
  runs: Vec<Run<'a>>,
}

impl<'a> Para<'a> {
  pub fn new_run(&mut self) -> &mut Run<'a> {
    self.runs.push(Run::default());
    self.runs.last_mut().unwrap()
  }

  pub fn get_style(&mut self) -> &mut ParaStyle<'a> {
    &mut self.prop
  }
}

/// Specifies the contents of the body of the document.
#[derive(Debug, Xml)]
pub enum BodyContent<'a> {
  #[xml(event = "Start")]
  #[xml(tag = "w:p")]
  Para(Para<'a>),
  // Table,
  // SecProp,
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:body")]
pub struct Body<'a> {
  #[xml(child)]
  #[xml(tag = "w:p")]
  content: Vec<BodyContent<'a>>,
}

impl<'a> Body<'a> {
  pub fn create_para(&mut self) -> &mut Para<'a> {
    self.content.push(BodyContent::Para(Para::default()));
    match self.content.last_mut().unwrap() {
      BodyContent::Para(p) => p,
    }
  }
}
