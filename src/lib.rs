extern crate quick_xml;
extern crate zip;

mod app_xml;
mod body;
mod content_types_xml;
mod core_xml;
mod document_xml;
mod docx;
mod element;
mod events_list;
mod rels;

pub use docx::Docx;
