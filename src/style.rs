use quick_xml::events::BytesStart;
use std::borrow::Cow;

use errors::{Error, Result};
use schema::SCHEMA_MAIN;
use Xml;

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
  #[xml(child)]
  #[xml(tag = "w:name")]
  name: StyleName<'a>,
  #[xml(child)]
  #[xml(tag = "w:pPr")]
  para: ParaStyle<'a>,
  #[xml(child)]
  #[xml(tag = "w:rPr")]
  char: CharStyle<'a>,
}

fn style_extend_attrs(s: &Style, start: &mut BytesStart) {
  start.push_attribute(("w:type", "paragraph"));
  start.push_attribute(("w:styleId", &s.name.name as &str));
}

impl<'a> Style<'a> {
  pub fn with_name(&mut self, name: &'a str) -> &mut Self {
    self.name = StyleName {
      name: Cow::Borrowed(name),
    };
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
#[xml(event = "Empty")]
#[xml(tag = "w:name")]
pub struct StyleName<'a> {
  #[xml(attr = "w:val")]
  name: Cow<'a, str>,
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:rPr")]
pub struct CharStyle<'a> {
  #[xml(child)]
  #[xml(tag = "w:name")]
  color: Option<Color<'a>>,
  #[xml(child)]
  #[xml(tag = "w:sz")]
  sz: Option<Size<'a>>,
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Empty")]
#[xml(tag = "w:color")]
pub struct Color<'a> {
  #[xml(attr = "w:val")]
  val: Cow<'a, str>,
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Empty")]
#[xml(tag = "w:sz")]
pub struct Size<'a> {
  #[xml(attr = "w:val")]
  val: Cow<'a, str>,
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:pPr")]
pub struct ParaStyle<'a> {
  #[xml(child)]
  #[xml(tag = "w:pStyle")]
  style: Option<ParaStyleName<'a>>,
  #[xml(child)]
  #[xml(tag = "w:jc")]
  jc: Option<Justification>,
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Empty")]
#[xml(tag = "w:pStyle")]
pub struct ParaStyleName<'a> {
  #[xml(attr = "w:val")]
  val: Cow<'a, str>,
}

#[derive(Debug, Default, Xml)]
#[xml(event = "Empty")]
#[xml(tag = "w:jc")]
pub struct Justification {
  #[xml(attr = "w:val")]
  val: String,
}

// #[derive(Clone, Debug)]
// pub enum JustificationType {
//   Start,
//   End,
//   Center,
//   Both,
//   Distribute,
// }

// impl JustificationType {
//   pub fn as_ref(&self) -> &str {
//     match *self {
//       JustificationType::Start => "start",
//       JustificationType::End => "end",
//       JustificationType::Center => "center",
//       JustificationType::Both => "both",
//       JustificationType::Distribute => "distribute",
//     }
//   }
// }

impl<'a> ParaStyle<'a> {
  pub fn with_jc(&mut self, jc: &'a str) -> &mut Self {
    self.jc = Some(Justification {
      val: jc.to_string(),
    });
    self
  }

  pub fn with_name(&mut self, name: &'a str) -> &mut Self {
    self.style = Some(ParaStyleName {
      val: Cow::Borrowed(name),
    });
    self
  }
}

impl<'a> CharStyle<'a> {
  pub fn with_sz(&mut self, sz: &'a str) -> &mut Self {
    self.sz = Some(Size {
      val: Cow::Borrowed(sz),
    });
    self
  }

  pub fn with_color(&mut self, color: &'a str) -> &mut Self {
    self.color = Some(Color {
      val: Cow::Borrowed(color),
    });
    self
  }
}
