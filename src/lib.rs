extern crate quick_xml;
extern crate zip;

mod body;
mod content_type;
mod docx;
mod events_list;
mod schema;
mod xml;

pub use docx::Docx;
