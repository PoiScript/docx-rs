use quick_xml::events::*;
use quick_xml::Writer;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;
use schema::{SCHEMA_MAIN, SCHEMA_RELATIONSHIPS};
use xml::Xml;

#[derive(Debug, Default)]
pub struct Font<'a> {
  name: &'a str,
  charset: &'a str,
  family: &'a str,
  pitch: &'a str,
}

impl<'a> Xml<'a> for Font<'a> {
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    tag!(w, b"w:font"["w:name", self.name] {{
      tag!(w, b"w:charset"["w:val", self.charset]);
      tag!(w, b"w:family"["w:val", self.family]);
      tag!(w, b"w:pitch"["w:val", self.pitch]);
    }});
    Ok(())
  }
}

#[derive(Debug, Default)]
pub struct FontTableXml<'a> {
  fonts: Vec<Font<'a>>,
}

impl<'a> Xml<'a> for FontTableXml<'a> {
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    tag!(w, b"w:fonts"["xmlns:w", SCHEMA_MAIN, "xmlns:r", SCHEMA_RELATIONSHIPS] {{
      for font in &self.fonts {
        font.write(w)?;
      }
    }});
    Ok(())
  }
}
