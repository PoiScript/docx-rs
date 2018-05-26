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
  justify: Option<Cow<'a, Justification>>,
}

pub trait StyleExt<'a> {
  fn with_jc<J>(&mut self, justification: J) -> &mut Self
  where
    J: Into<Cow<'a, Justification>>;
  fn with_sz(&mut self, size: usize) -> &mut Self;
  fn with_color<S>(&mut self, color: S) -> &mut Self
  where
    S: Into<Cow<'a, str>>;
}

impl<'a> StyleExt<'a> for Style<'a> {
  fn with_jc<J>(&mut self, justification: J) -> &mut Self
  where
    J: Into<Cow<'a, Justification>>,
  {
    self.justify = Some(justification.into());
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

  pub fn write_p_pr<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    if let Some(ref jc) = self.justify {
      write_events!(writer, (b"w:jc", "w:val", jc.as_str()));
    }
    Ok(())
  }

  pub fn write_r_pr<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    if let Some(ref size) = self.size {
      write_events!(writer, (b"w:jc", "w:val", size.to_string().as_str()));
    }
    if let Some(ref color) = self.color {
      write_events!(writer, (b"w:color", "w:val", color.as_ref()));
    }
    Ok(())
  }
}

impl<'a> Xml<'a> for Style<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_events!(writer, (b"w:style", "w:type", "paragraph", "w:styleId", self.name) {
      (b"w:name", "w:val", self.name)
      b"w:pPr" {[
        self.write_p_pr(writer)?
      ]}
      b"w:rPr" {[
        self.write_r_pr(writer)?
      ]}
    });
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
