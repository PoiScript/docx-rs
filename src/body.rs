use std::borrow::Cow;

use errors::{Error, Result};
use style::{CharStyle, ParaStyle, ParaStyleName};
use xml::Xml;

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
  content: Vec<TextRun<'a>>,
}

impl<'a> Run<'a> {
  pub fn add_text(&mut self, text: &'a str) -> &mut Self {
    self.content.push(TextRun {
      text: Cow::Borrowed(text),
    });
    self
  }
}

// #[derive(Debug, Xml)]
// pub enum RunContent<'a> {
//   #[xml(event = "Start")]
//   #[xml(tag = "w:t")]
//   Text(TextRun<'a>),
//   #[xml(event = "Empty")]
//   #[xml(tag = "w:br")]
//   Break(BreakRun),
// }

#[derive(Debug, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:t")]
struct TextRun<'a> {
  #[xml(text)]
  text: Cow<'a, str>,
}

#[derive(Debug, Xml)]
#[xml(event = "Empty")]
#[xml(tag = "w:br")]
struct BreakRun;

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

// // Specifies the contents of the body of the document.
// #[derive(Debug, Xml)]
// pub enum BodyContent<'a> {
//   #[xml(event = "Start")]
//   #[xml(tag = "w:p")]
//   Para(Para<'a>),
//   Table,
//   SecProp,
// }

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:body")]
pub struct Body<'a> {
  #[xml(child)]
  #[xml(tag = "w:p")]
  content: Vec<Para<'a>>,
}

impl<'a> Body<'a> {
  pub fn create_para(&mut self) -> &mut Para<'a> {
    self.content.push(Para::default());
    self.content.last_mut().unwrap()
  }
}
