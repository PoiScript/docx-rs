use quick_xml::events::*;
use quick_xml::Writer;
use std::borrow::Cow;
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
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    match *self {
      Run::Text(text) => tag!(w, b"w:t"{text}),
      Run::Break => tag!(w, b"w:br"),
    }
    Ok(())
  }
}

#[derive(Debug, Default)]
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
  fn with_jc(&mut self, justification: &'a Justification) -> &mut Self {
    self.get_style().with_jc(justification);
    self
  }

  fn with_sz(&mut self, size: usize) -> &mut Self {
    self.get_style().with_sz(size);
    self
  }

  fn with_color<S>(&mut self, color: S) -> &mut Self
  where
    S: Into<Cow<'a, str>>,
  {
    self.get_style().with_color(color);
    self
  }
}

impl<'a> Xml<'a> for Para<'a> {
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    tag!(w, b"w:p" {{
      tag!(w, b"w:pPr" {{
        if let Some(style_name) = self.style_name {
          tag!(w, b"w:pStyle"["w:val", style_name]);
        }
        if let Some(ref style) = self.style {
          style.write_p_pr(w)?;
        }
      }});
      tag!(w, b"w:r" {{
        for run in &self.runs {
          run.write(w)?;
        }
      }});
    }});
    Ok(())
  }
}

// Specifies the contents of the body of the document.
pub enum _Content<'a> {
  Para(Para<'a>),
  Table,
  SecProp,
}
