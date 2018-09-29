#[macro_use]
extern crate docx_codegen;
#[macro_use]
extern crate log;
extern crate quick_xml;
extern crate zip;

#[macro_use]
mod macros;

pub mod app;
pub mod content_type;
pub mod core;
pub mod document;
pub mod docx;
pub mod errors;
pub mod font_table;
pub mod prelude;
pub mod rels;
mod schema;
pub mod style;
