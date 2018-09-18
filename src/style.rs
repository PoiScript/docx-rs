use quick_xml::events::*;
use quick_xml::Writer;
use std::borrow::Cow;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;
use xml::Xml;

#[derive(Debug, Default)]
pub struct Style<'a> {
  name: &'a str,
  size: Option<usize>,
  color: Option<Cow<'a, str>>,
  justify: Option<&'a Justification>,
}

pub trait StyleExt<'a> {
  fn with_jc(&mut self, justification: &'a Justification) -> &mut Self;
  fn with_sz(&mut self, size: usize) -> &mut Self;
  fn with_color<S>(&mut self, color: S) -> &mut Self
  where
    S: Into<Cow<'a, str>>;
}

impl<'a> StyleExt<'a> for Style<'a> {
  fn with_jc(&mut self, justification: &'a Justification) -> &mut Self {
    self.justify = Some(justification);
    self
  }

  fn with_sz(&mut self, size: usize) -> &mut Self {
    self.size = Some(size);
    self
  }

  fn with_color<S>(&mut self, color: S) -> &mut Self
  where
    S: Into<Cow<'a, str>>,
  {
    self.color = Some(color.into());
    self
  }
}

impl<'a> Style<'a> {
  pub fn with_name(&mut self, name: &'a str) -> &mut Self {
    self.name = name;
    self
  }

  pub fn write_p_pr<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    if let Some(ref jc) = self.justify {
      tag!(w, b"w:jc"["w:val", jc.as_str()]);
    }
    Ok(())
  }

  pub fn write_r_pr<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    if let Some(ref size) = self.size {
      tag!(w, b"w:jc"["w:val", size.to_string().as_str()]);
    }
    if let Some(ref color) = self.color {
      tag!(w, b"w:color"["w:val", color.as_ref()]);
    }
    Ok(())
  }
}

impl<'a> Xml for Style<'a> {
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    tag!(w, b"w:style"["w:type", "paragraph", "w:styleId", self.name] {{
      tag!(w, b"w:name"["w:val", self.name]);
      tag!(w, b"w:pPr" {{
        self.write_p_pr(w)?;
      }});
      tag!(w, b"w:rPr" {{
        self.write_r_pr(w)?;
      }});
    }});
    Ok(())
  }
}

#[derive(Clone, Debug)]
pub enum Justification {
  Start,
  End,
  Center,
  Both,
  Distribute,
}

impl Justification {
  pub fn as_str(&self) -> &str {
    match *self {
      Justification::Start => "start",
      Justification::End => "end",
      Justification::Center => "center",
      Justification::Both => "both",
      Justification::Distribute => "distribute",
    }
  }
}
