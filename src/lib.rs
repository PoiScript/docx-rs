extern crate quick_xml;
extern crate zip;

#[macro_use]
mod event_macro;

mod body;
mod content_type;
mod docx;
mod errors;
mod schema;
mod style;
mod xml;

pub use body::Para;
pub use docx::Docx;
pub use style::{Justification, Style};
