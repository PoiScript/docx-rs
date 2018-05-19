use quick_xml::events::*;
use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;
use style::Style;
use xml::Xml;

// Specifies a run of content within the paragraph.
#[derive(Debug)]
pub struct Run<'a> {
  text: &'a str,
}

impl<'a> Run<'a> {
  fn new(text: &'a str) -> Run<'a> {
    Run { text }
  }
}

impl<'a> Default for Run<'a> {
  fn default() -> Run<'a> {
    Run { text: "" }
  }
}

impl<'a> Xml<'a> for Run<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_events!(writer, b"w:r"{
      b"w:t"{self.text}
    });
    Ok(())
  }
}

#[derive(Debug)]
pub struct Para<'a> {
  runs: Vec<Run<'a>>,
  style: Option<&'a str>,
  extend: Option<&'a Style<'a>>,
}

impl<'a> Para<'a> {
  pub fn new(text: &'a str) -> Para<'a> {
    Para {
      runs: vec![Run::new(text)],
      style: None,
      extend: None,
    }
  }

  pub fn with_style_name(mut self, style: &'a Style) -> Self {
    self.style = Some(style.name);
    self
  }
}

impl<'a> Default for Para<'a> {
  fn default() -> Para<'a> {
    Para {
      runs: Vec::new(),
      style: None,
      extend: None,
    }
  }
}

impl<'a> Xml<'a> for Para<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_events!(writer, b"w:p"{
      b"w:pPr"{[
        if let Some(style_name) = self.style {
          write_empty_event!(writer, b"w:pStyle", "w:val", style_name);
        }
      ]}
      [
        for run in &self.runs {
          run.write(writer)?;
        }
      ]
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
