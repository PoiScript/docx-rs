mod app;
mod content_types;
mod core;
mod document;
mod font_table;
mod rels;
mod styles;

pub use self::app::AppXml;
pub use self::content_types::ContentTypes;
pub use self::core::CoreXml;
pub use self::document::DocumentXml;
pub use self::font_table::FontTableXml;
pub use self::rels::RelsXml;
pub use self::styles::StylesXml;

use quick_xml::events::BytesStart;
use quick_xml::{Reader, Writer};
use std::io::Write;

use errors::Result;

pub trait Xml {
  fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
  where
    W: Write;

  fn read(r: &mut Reader<&[u8]>, bs: Option<&BytesStart>) -> Result<Self>
  where
    Self: Sized;
}
