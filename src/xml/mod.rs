mod app;
mod comments;
mod content_types;
mod core;
mod document;
mod font_table;
mod rels;
mod styles;

pub use self::app::AppXml;
pub use self::content_types::ContentTypesXml;
pub use self::core::CoreXml;
pub use self::document::DocumentXml;
pub use self::font_table::FontTableXml;
pub use self::rels::RelsXml;
pub use self::styles::StylesXml;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::{Reader, Writer};
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;

pub trait Xml<'a>: Default {
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()>;
}

pub trait XmlStruct {
  fn write<W>(&self, writer: &mut Writer<W>) -> Result<()>
  where
    W: Write + Seek;
  fn read_with_attrs(attrs: Attributes, reader: &mut Reader<&[u8]>) -> Self;
  fn read(reader: &mut Reader<&[u8]>) -> Self;
}

pub trait XmlEnum {
  fn write<W>(&self, writer: &mut Writer<W>) -> Result<()>
  where
    W: Write + Seek;
  fn read_with_attrs(attrs: Attributes, tag: &[u8], reader: &mut Reader<&[u8]>) -> Self;
  fn read(reader: &mut Reader<&[u8]>) -> Self
  where
    Self: Sized,
  {
    let mut buf = Vec::new();
    loop {
      match reader.read_event(&mut buf) {
        Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
          return Self::read_with_attrs(e.attributes(), e.name(), reader);
        }
        _ => break,
      }
    }
    // TODO throws an error
    unreachable!();
  }
}
