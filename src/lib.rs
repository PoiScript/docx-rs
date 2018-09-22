#[macro_use]
extern crate docx_codegen;
extern crate quick_xml;
extern crate zip;

mod app;
mod content_type;
mod core;
mod document;
mod docx;
pub mod errors;
mod font_table;
pub mod prelude;
mod rels;
mod schema;
mod style;

use quick_xml::events::BytesStart;
use quick_xml::{Reader, Writer};
use std::io::{BufRead, Write};

use errors::Result;

pub trait Xml {
  fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
  where
    W: Write;

  fn read<B>(r: &mut Reader<B>, bs: Option<&BytesStart>) -> Result<Self>
  where
    Self: Sized,
    B: BufRead;
}
