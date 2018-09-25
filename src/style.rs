use quick_xml::events::BytesStart;
use std::borrow::{Borrow, Cow};
use std::convert::AsRef;
use std::str::FromStr;

use errors::{Error, Result};
use schema::SCHEMA_MAIN;

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:styles")]
#[xml(extend_attrs = "styles_extend_attrs")]
pub struct Styles<'a> {
  #[xml(child)]
  #[xml(tag = "w:style")]
  styles: Vec<Style<'a>>,
}

fn styles_extend_attrs(_: &Styles, start: &mut BytesStart) {
  start.push_attribute(("xmlns:w", SCHEMA_MAIN));
}

impl<'a> Styles<'a> {
  pub fn create_style(&mut self) -> &mut Style<'a> {
    self.styles.push(Style::default());
    self.styles.last_mut().unwrap()
  }
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:style")]
#[xml(extend_attrs = "style_extend_attrs")]
pub struct Style<'a> {
  #[xml(flatten_empty)]
  #[xml(tag = "w:name")]
  #[xml(attr = "w:val")]
  name: Option<Cow<'a, str>>,
  #[xml(child)]
  #[xml(tag = "w:pPr")]
  para: ParaStyle<'a>,
  #[xml(child)]
  #[xml(tag = "w:rPr")]
  char: CharStyle<'a>,
}

fn style_extend_attrs(s: &Style, start: &mut BytesStart) {
  start.push_attribute(("w:type", "paragraph"));
  if let Some(ref name) = s.name {
    start.push_attribute(("w:styleId", name.borrow()));
  }
}

impl<'a> Style<'a> {
  pub fn with_name(&mut self, name: &'a str) -> &mut Self {
    self.name = Some(Cow::Borrowed(name));
    self
  }

  pub fn para_style(&mut self) -> &mut ParaStyle<'a> {
    &mut self.para
  }

  pub fn char_style(&mut self) -> &mut CharStyle<'a> {
    &mut self.char
  }
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:rPr")]
pub struct CharStyle<'a> {
  #[xml(flatten_empty)]
  #[xml(tag = "w:color")]
  #[xml(attr = "w:val")]
  color: Option<Cow<'a, str>>,
  #[xml(flatten_empty)]
  #[xml(tag = "w:sz")]
  #[xml(attr = "w:val")]
  sz: Option<usize>,
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:pPr")]
pub struct ParaStyle<'a> {
  #[xml(flatten_empty)]
  #[xml(tag = "w:pStyle")]
  #[xml(attr = "w:val")]
  name: Option<Cow<'a, str>>,
  #[xml(flatten_empty)]
  #[xml(tag = "w:jc")]
  #[xml(attr = "w:val")]
  jc: Option<Justification>,
}

#[derive(Debug)]
pub enum Justification {
  Start,
  End,
  Center,
  Both,
  Distribute,
}

string_enum!{
  Justification {
    Start = "start",
    End = "end",
    Center = "center",
    Both = "both",
    Distribute = "distribute",
  }
}

impl<'a> ParaStyle<'a> {
  pub fn with_jc(&mut self, jc: Justification) -> &mut Self {
    self.jc = Some(jc);
    self
  }

  pub fn with_name(&mut self, name: &'a str) -> &mut Self {
    self.name = Some(Cow::Borrowed(name));
    self
  }
}

impl<'a> CharStyle<'a> {
  pub fn with_sz(&mut self, sz: usize) -> &mut Self {
    self.sz = Some(sz);
    self
  }

  pub fn with_color(&mut self, color: &'a str) -> &mut Self {
    self.color = Some(Cow::Borrowed(color));
    self
  }
}
