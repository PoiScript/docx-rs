use quick_xml::events::*;
use quick_xml::Result;
use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use xml::Xml;

// Specifies a run of content within the paragraph.
#[derive(Debug)]
pub struct Run<'a> {
  text: &'a str,
  props: Vec<RunProp>,
}

impl<'a> Run<'a> {
  fn new(text: &'a str) -> Run<'a> {
    Run {
      text,
      props: Vec::new(),
    }
  }
}

impl<'a> Default for Run<'a> {
  fn default() -> Run<'a> {
    Run {
      text: "",
      props: Vec::new(),
    }
  }
}

impl<'a> Xml<'a> for Run<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_start_event!(writer, b"w:r");
    write_events!(writer, b"w:t"{self.text});
    write_end_event!(writer, b"w:r");
    Ok(())
  }
}

// Specifies properties for the run.
#[derive(Debug)]
pub enum RunProp {}

#[derive(Debug)]
pub struct Para<'a> {
  props: Vec<ParaProp>,
  runs: Vec<Run<'a>>,
}

impl<'a> Para<'a> {
  pub fn new(text: &'a str) -> Para<'a> {
    Para {
      runs: vec![Run::new(text)],
      props: Vec::new(),
    }
  }
}

impl<'a> Default for Para<'a> {
  fn default() -> Para<'a> {
    Para {
      runs: Vec::new(),
      props: Vec::new(),
    }
  }
}

impl<'a> Xml<'a> for Para<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_start_event!(writer, b"w:p");
    for run in &self.runs {
      run.write(writer);
    }
    write_end_event!(writer, b"w:p");
    Ok(())
  }
}

// Specifies a set of properties for the paragraph.
#[derive(Debug)]
pub enum ParaProp {}

// Specifies the contents of the body of the document.
pub enum _Content<'a> {
  Para(Para<'a>),
  Table,
  SecProp,
}
