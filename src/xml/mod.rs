mod app;
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

use quick_xml::events::BytesStart;
use quick_xml::{Reader, Writer};
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;

pub trait Xml<'a>: Default {
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()>;
}

pub trait XmlStruct: Default {
  fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
  where
    W: Write + Seek;
  fn read_with_bytes_start(bs: &BytesStart, r: &mut Reader<&[u8]>) -> Result<Self>
  where
    Self: Sized;
  fn read(r: &mut Reader<&[u8]>) -> Result<Self>
  where
    Self: Sized;
}

pub trait XmlEnum: Default {
  fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
  where
    W: Write + Seek;
  fn read_with_bytes_start(bs: &BytesStart, r: &mut Reader<&[u8]>) -> Result<Self>
  where
    Self: Sized;
  fn read(r: &mut Reader<&[u8]>) -> Result<Self>
  where
    Self: Sized;
}
