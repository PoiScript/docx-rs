[![Build Status](https://travis-ci.org/PoiScript/docx-rs.svg?branch=master)](https://travis-ci.org/PoiScript/docx-rs)
[![docs.rs](https://docs.rs/docx/badge.svg)](https://docs.rs/docx)
[![crates.io](https://img.shields.io/crates/v/docx.svg)](https://crates.io/crates/docx)

# docx

A Rust library for parsing and generating docx files.

## Usage

```toml
docx = "0.1.3"
```

Using methods [`from_file`] and [`write_file`] for reading from and writing to file directly.

[`from_file`]: struct.Docx.html#method.from_file
[`write_file`]: struct.Docx.html#method.write_file

```rust
use docx::Docx;
use docx::document::Para;

// reading docx from file
let mut docx = Docx::from_file("demo.docx").unwrap();

// do whatever you want...

// for example, appending text
let mut para = Para::default();
para.text("Lorem Ipsum");
docx.insert_para(para);

// writing back to the original file
docx.write_file("demo.docx").unwrap();
```

Alternatively, you can use [`parse`] (accepts [`Read`] + [`Seek`]) and [`generate`] (accepts [`Write`] + [`Seek`]).

[`parse`]: struct.Docx.html#method.parse
[`generate`]: struct.Docx.html#method.generate
[`Read`]: std::io::Read
[`Write`]: std::io::Write
[`Seek`]: std::io::Seek

### Glossary

Some terms used in this crate.

* Body: main surface for editing
* Paragraph: block-level container of content, begins with a new line
* Run(Character): non-block region of text
* Style: a set of paragraph and character properties which can be applied to text

## Note

### Toggle Properties

Some fields in this crate (e.g. [`bold`] and [`italics`]) are declared as `Option<bool>` instead of `bool`.

This indicates that they are **toggle properties** which can be **inherited** (`None`) or **disabled/enabled explicitly** (`Some`).

[`bold`]: style/struct.CharStyle.html#structfield.bold
[`italics`]: style/struct.CharStyle.html#structfield.italics

For example, you can disable bold of a run within a paragraph specified bold by setting `bold` to `Some(false)`:

```rust
use docx::Docx;
use docx::document::{Para, Run};

let mut docx = Docx::default();

docx
  .create_style()
  .name("Normal")
  .char()
  .bold(true)
  .italics(true);

let mut para = Para::default();
para.prop().name("Normal");

// inherited from its parent
para.text("I'm bold and italics.").text_break();

let mut run = Run::text("But I'm not.");
run.prop().bold(false).italics(false);
para.run(run);

docx.insert_para(para);
```

## License

MIT
