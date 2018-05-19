mod app;
mod content_types;
mod core;
mod document;
mod font_table;
mod rels;

pub use self::app::AppXml;
pub use self::content_types::ContentTypesXml;
pub use self::core::CoreXml;
pub use self::document::DocumentXml;
pub use self::font_table::FontTableXml;
pub use self::rels::RelsXml;

use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;

pub trait Xml<'a>: Default {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()>;
}
