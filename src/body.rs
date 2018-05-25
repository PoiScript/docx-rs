use quick_xml::events::*;
use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;
use style::{Justification, Style, StyleExt};
use xml::Xml;

// Specifies a run of content within the paragraph.
#[derive(Debug)]
pub enum Run<'a> {
  Text(&'a str),
  Break,
}

impl<'a> Default for Run<'a> {
  fn default() -> Run<'a> {
    Run::Break
  }
}

impl<'a> Xml<'a> for Run<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    match *self {
      Run::Text(text) => write_events!(writer, b"w:t"{text}),
      Run::Break => write_empty_event!(writer, b"w:br"),
    }
    Ok(())
  }
}

#[derive(Debug)]
pub struct Para<'a> {
  runs: Vec<Run<'a>>,
  style: Option<Style<'a>>,
  style_name: Option<&'a str>,
}

impl<'a> Para<'a> {
  pub fn with_style_name(&mut self, name: &'a str) -> &mut Self {
    self.style_name = Some(name);
    self
  }

  pub fn add_break(&mut self) -> &mut Self {
    self.runs.push(Run::Break);
    self
  }

  pub fn add_text(&mut self, text: &'a str) -> &mut Self {
    self.runs.push(Run::Text(text));
    self
  }

  pub fn get_style(&mut self) -> &mut Style<'a> {
    self.style.get_or_insert(Style::default())
  }
}

impl<'a> StyleExt<'a> for Para<'a> {
  fn with_jc(&mut self, justification: &Justification) -> &mut Self {
    self.get_style().with_jc(justification);
    self
  }

  fn with_sz(&mut self, size: usize) -> &mut Self {
    self.get_style().with_sz(size);
    self
  }

  fn with_color(&mut self, color: &'a str) -> &mut Self {
    self.get_style().with_color(color);
    self
  }
}

impl<'a> Default for Para<'a> {
  fn default() -> Para<'a> {
    Para {
      runs: Vec::new(),
      style: None,
      style_name: None,
    }
  }
}

impl<'a> Xml<'a> for Para<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_events!(writer, b"w:p"{
      b"w:pPr"{[
        if let Some(style_name) = self.style_name {
          write_empty_event!(writer, b"w:pStyle", "w:val", style_name);
        }
      ] [
        if let Some(ref style) = self.style {
          style.write_p_pr(writer)?;
        }
      ]}
      b"w:r"{[
        for run in &self.runs {
          run.write(writer)?;
        }
      ]}
    });
    Ok(())
  }
}

// Specifies the contents of the body of the document.
pub enum _Content<'a> {
  Para(Para<'a>),
  Table,
  SecProp,
}
