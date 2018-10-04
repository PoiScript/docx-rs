extern crate docx;

use docx::prelude::*;

fn main() {
  let mut docx = Docx::default();

  let mut para = Para::default();
  para.text("hello, world");
  docx.insert_para(para);

  docx.write_file("origin.docx").unwrap();

  let mut docx = Docx::from_file("origin.docx").unwrap();

  let mut para = Para::default();
  para.text("world, hello");
  docx.insert_para(para);

  docx.write_file("origin_appended.docx").unwrap();
}
