use quick_xml::events::*;
use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;
use schema::{SCHEMA_MAIN, SCHEMA_RELATIONSHIPS};
use xml::Xml;

#[derive(Debug)]
pub struct Font<'a> {
  name: &'a str,
  charset: &'a str,
  family: &'a str,
  pitch: &'a str,
}

impl<'a> Default for Font<'a> {
  fn default() -> Font<'a> {
    Font {
      name: "Times New Roman",
      charset: "00",
      family: "roman",
      pitch: "variable",
    }
  }
}

impl<'a> Xml<'a> for Font<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_events!(writer, (b"w:font", "w:name", self.name) {
      (b"w:charset", "w:val", self.charset)
      (b"w:family", "w:val", self.family)
      (b"w:pitch", "w:val", self.pitch)
    });
    Ok(())
  }
}

#[derive(Debug)]
pub struct FontTableXml<'a> {
  fonts: Vec<Font<'a>>,
}

impl<'a> Default for FontTableXml<'a> {
  fn default() -> FontTableXml<'a> {
    FontTableXml {
      fonts: vec![Font::default()],
    }
  }
}

impl<'a> Xml<'a> for FontTableXml<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_events!(writer, (b"w:fonts", "xmlns:w", SCHEMA_MAIN, "xmlns:r", SCHEMA_RELATIONSHIPS) {[
      for font in &self.fonts {
        font.write(writer)?;
      }
    ]});
    Ok(())
  }
}
