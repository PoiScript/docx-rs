# docx

[![Build Status](https://travis-ci.org/PoiScript/docx-rs.svg?branch=master)](https://travis-ci.org/PoiScript/docx-rs)
[![docs.rs](https://docs.rs/docx/badge.svg)](https://docs.rs/docx)
[![crates.io](https://img.shields.io/crates/v/docx.svg)](https://crates.io/crates/docx)

A Rust library for parsing and generating docx files.

## Create a new document

Use `Docx::default` to create a new empty `Docx`, then use
`Docx::write_file` for saving it to a file.

```rust
let mut docx = Docx::default();
let mut para = Para::default();

// create a new paragraph and insert it
para.text("Lorem Ipsum");
docx.insert_para(para);

docx.write_file("demo.docx").unwrap();
```

Also see: `Docx::write`.

## Reading from files

Use `DocxFile::from_file` to extract content from docx files, then use
`DocxFile::parse` to generate a `Docx` struct.

```rust
let docx = DocxFile::from_file("origin.docx").unwrap();
let mut docx = docx.parse().unwrap();
let mut para = Para::default();

para.text("Lorem Ipsum");
docx.insert_para(para);

docx.write_file("origin_appended.docx").unwrap();
```

To reduce allocations, `DocxFile::parse` returns a `Docx` struct contains
references to `DocxFile` itself. It means you have to make sure that
`DocxFile` lives as long as its returned `Docx`:

```rust
let mut docx_option = None;
{
    let docx_file = DocxFile::from_file("foo.docx").unwrap();
    let mut docx = docx_file.parse().unwrap();
    docx_option = Some(docx);
    // `docx_file` gets dropped here and code fails to compile
}
docx_option.unwrap().write_file("foo.docx").unwrap();
```

Or you can use `Docx::into_owned` to convert it into `Docx<'static>`:

```rust
let mut docx_option = None;
{
    let docx_file = DocxFile::from_file("foo.docx").unwrap();
    let mut docx = docx_file.parse().unwrap();
    docx_option = Some(docx.into_owned());
}
docx_option.unwrap().write_file("foo.docx").unwrap();
```

Also see: `DocxFile::from_reader`.

## License

MIT
