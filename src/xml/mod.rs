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

use quick_xml::events::{BytesDecl, Event};
use quick_xml::Result;
use quick_xml::Writer;
use std::collections::LinkedList;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

pub trait Xml<'a>: Default {
  fn events(&self) -> LinkedList<Event<'a>>;

  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    // write XML declaration
    writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"utf-8"), None)));
    for event in self.events() {
      writer.write_event(event)?;
    }
    Ok(())
  }
}
