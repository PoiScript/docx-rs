extern crate docx;

use docx::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
  let path = Path::new("origin.docx");
  let file = File::create(&path).unwrap();

  let mut docx = Docx::default();

  docx.create_para().new_run().text("hello, world");

  docx.generate(file).unwrap();

  let file = File::open(&path).unwrap();

  let mut docx = Docx::parse(file).unwrap();

  docx.create_para().new_run().text("world, hello");

  let path = Path::new("origin_appended.docx");
  let file = File::create(&path).unwrap();

  docx.generate(file).unwrap();
}
