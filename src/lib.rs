//! A Rust library for parsing and generating docx files.
//!
//! ## Example
//!
//! Using methods [`from_file`] and [`to_file`] for reading from and writing to file directly.
//!
//! [`from_file`]: struct.Docx.html#method.from_file
//! [`to_file`]: struct.Docx.html#method.to_file
//!
//! ```no_run
//! use docx::Docx;
//!
//! // reading docx from file
//! let mut docx = Docx::from_file("demo.docx").unwrap();
//!
//! // do what you want to do…
//! // for example, appending something
//! docx.create_para().new_run().text("Lorem Ipsum");
//!
//! // writing back to the original file
//! docx.to_file("demo.docx").unwrap();
//! ```
//!
//! Alternatively, you can use [`parse`] (accepts [`Read`] + [`Seek`]) and [`generate`] (accepts [`Write`] + [`Seek`]).
//!
//! [`parse`]: struct.Docx.html#method.parse
//! [`generate`]: struct.Docx.html#method.generate
//! [`Read`]: std::io::Read
//! [`Write`]: std::io::Write
//! [`Seek`]: std::io::Seek
//!
//! ## Glossary
//!
//! Some terms used in this crate.
//!
//! * Body: main surface for editing
//! * Paragraph: block-level container of content, begins with a new line
//! * Run(Character): non-block region of text
//! * Style: a set of paragraph and character properties which can be applied to text
//!
//! ## Note
//!
//! ### Toggle Properties
//!
//! Some fields in this crate (e.g. [`bold`] and [`italics`]) are declared as `Option<bool>` and this is not redundant at all.
//!
//! This indicates that they are **toggle properties** which can be **inherited** from style (`None`) or **disabled/enabled explicitly** (`Some`).
//!
//! [`bold`]: style/struct.CharStyle.html#structfield.bold
//! [`italics`]: style/struct.CharStyle.html#structfield.italics
//!
//! For example, you can disable bold of a run within a paragraph specifies bold by setting it to `Some(false)`:
//!
//! ```rust
//! use docx::Docx;
//!
//! let mut docx = Docx::default();
//!
//! docx
//!   .create_style()
//!   .name("Normal")
//!   .char()
//!   .bold(true)
//!   .italics(true);
//!
//! let para = docx.create_para();
//! para.prop().name("Normal");
//!
//! para.new_run().text("I'm bold and italics.").text_break();
//!
//! para
//!   .new_run()
//!   .text("I'm neither bold nor italics.")
//!   .prop()
//!   .bold(false)
//!   .italics(false);
//! ```

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
mod docx;
pub mod errors;
pub mod font_table;
pub mod prelude;
pub mod rels;
mod schema;
pub mod style;

pub use docx::Docx;
