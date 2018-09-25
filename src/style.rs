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
  #[xml(tag = "w:docDefaults")]
  default: Option<DefaultStyle<'a>>,
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
#[xml(tag = "w:docDefaults")]
pub struct DefaultStyle<'a> {
  #[xml(child)]
  #[xml(tag = "w:rPrDefault")]
  char: CharStyle<'a>,
  #[xml(child)]
  #[xml(tag = "w:pPrDefault")]
  para: ParaStyle<'a>,
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:rPrDefault")]
pub struct DefaultCharStyle<'a> {
  #[xml(child)]
  #[xml(tag = "w:rPr")]
  inner: CharStyle<'a>,
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:pPrDefault")]
pub struct DefaultParaStyle<'a> {
  #[xml(child)]
  #[xml(tag = "w:pPr")]
  inner: ParaStyle<'a>,
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
  pub para: ParaStyle<'a>,
  #[xml(child)]
  #[xml(tag = "w:rPr")]
  pub char: CharStyle<'a>,
}

fn style_extend_attrs(s: &Style, start: &mut BytesStart) {
  start.push_attribute(("w:type", "paragraph"));
  if let Some(ref name) = s.name {
    start.push_attribute(("w:styleId", name.borrow()));
  }
}

impl<'a> Style<'a> {
  pub fn name(&mut self, name: &'a str) -> &mut Self {
    self.name = Some(Cow::Borrowed(name));
    self
  }

  pub fn reset_name(&mut self) -> &mut Self {
    self.name = None;
    self
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
  #[xml(flatten_empty)]
  #[xml(tag = "w:b")]
  #[xml(attr = "w:val")]
  bold: Option<bool>,
  #[xml(flatten_empty)]
  #[xml(tag = "w:i")]
  #[xml(attr = "w:val")]
  italics: Option<bool>,
  #[xml(flatten_empty)]
  #[xml(tag = "w:strike")]
  #[xml(attr = "w:val")]
  strike: Option<bool>,
  #[xml(flatten_empty)]
  #[xml(tag = "w:dstrike")]
  #[xml(attr = "w:val")]
  dstrike: Option<bool>,
  #[xml(flatten_empty)]
  #[xml(tag = "w:outline")]
  #[xml(attr = "w:val")]
  outline: Option<bool>,
  #[xml(child)]
  #[xml(tag = "w:u")]
  underline: Option<Underline<'a>>,
}

impl<'a> CharStyle<'a> {
  pub fn sz(&mut self, sz: usize) -> &mut Self {
    self.sz = Some(sz);
    self
  }

  pub fn reset_sz(&mut self) -> &mut Self {
    self.sz = None;
    self
  }

  pub fn color(&mut self, color: &'a str) -> &mut Self {
    self.color = Some(Cow::Borrowed(color));
    self
  }

  pub fn reset_color(&mut self) -> &mut Self {
    self.color = None;
    self
  }

  pub fn bold(&mut self, val: bool) -> &mut Self {
    self.bold = Some(val);
    self
  }

  pub fn reset_bold(&mut self) -> &mut Self {
    self.bold = None;
    self
  }

  pub fn italics(&mut self, val: bool) -> &mut Self {
    self.italics = Some(val);
    self
  }

  pub fn reset_italics(&mut self) -> &mut Self {
    self.italics = None;
    self
  }

  pub fn strike(&mut self, val: bool) -> &mut Self {
    self.strike = Some(val);
    if let Some(true) = self.dstrike {
      self.dstrike = None;
    }
    self
  }

  pub fn reset_strike(&mut self) -> &mut Self {
    self.strike = None;
    self
  }

  pub fn dstrike(&mut self, val: bool) -> &mut Self {
    self.dstrike = Some(val);
    if let Some(true) = self.strike {
      self.strike = None;
    }
    self
  }

  pub fn reset_dstrike(&mut self) -> &mut Self {
    self.dstrike = None;
    self
  }

  pub fn outline(&mut self, val: bool) -> &mut Self {
    self.outline = Some(val);
    self
  }

  pub fn reset_outline(&mut self) -> &mut Self {
    self.outline = None;
    self
  }

  pub fn underline(&mut self, color: Option<&'a str>, ty: Option<UnderlineType>) -> &mut Self {
    self.underline = Some(Underline {
      color: color.map(|s| Cow::Borrowed(s)),
      val: ty,
    });
    self
  }

  pub fn reset_underline(&mut self) -> &mut Self {
    self.underline = None;
    self
  }
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Empty")]
#[xml(tag = "w:u")]
pub struct Underline<'a> {
  #[xml(attr = "w:color")]
  color: Option<Cow<'a, str>>,
  #[xml(attr = "w:val")]
  val: Option<UnderlineType>,
}

#[derive(Debug)]
pub enum UnderlineType {
  Dash,
  DashDotDotHeavy,
  DashDotHeavy,
  DashedHeavy,
  DashLong,
  DashLongHeavy,
  DotDash,
  DotDotDash,
  Dotted,
  DottedHeavy,
  Double,
  None,
  Single,
  Thick,
  Wave,
  WavyDouble,
  WavyHeavy,
  Words,
}

string_enum! {
  UnderlineType {
    Dash = "dash",
    DashDotDotHeavy = "dashDotDotHeavy",
    DashDotHeavy = "dashDotHeavy",
    DashedHeavy = "dashedHeavy",
    DashLong = "dashLong",
    DashLongHeavy = "dashLongHeavy",
    DotDash = "dotDash",
    DotDotDash = "dotDotDash",
    Dotted = "dotted",
    DottedHeavy = "dottedHeavy",
    Double = "double",
    None = "none",
    Single = "single",
    Thick = "thick",
    Wave = "wave",
    WavyDouble = "wavyDouble",
    WavyHeavy = "wavyHeavy",
    Words = "words",
  }
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

impl<'a> ParaStyle<'a> {
  pub fn jc(&mut self, jc: Justification) -> &mut Self {
    self.jc = Some(jc);
    self
  }

  pub fn reset_jc(&mut self) -> &mut Self {
    self.jc = None;
    self
  }

  pub fn name(&mut self, name: &'a str) -> &mut Self {
    self.name = Some(Cow::Borrowed(name));
    self
  }

  pub fn reset_name(&mut self) -> &mut Self {
    self.name = None;
    self
  }
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
