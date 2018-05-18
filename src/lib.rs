#![feature(trace_macros)]
extern crate quick_xml;
extern crate zip;

mod body;
mod content_type;
mod docx;
#[macro_use]
mod event_macro;
mod events_list;
mod schema;
mod xml;

pub use body::Para;
pub use docx::Docx;
