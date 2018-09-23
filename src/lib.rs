#[macro_use]
extern crate docx_codegen;
#[macro_use]
extern crate log;
extern crate quick_xml;
extern crate zip;

#[macro_use]
mod macros;

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
