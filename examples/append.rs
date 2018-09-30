extern crate docx;

use docx::prelude::*;

fn main() {
  let mut docx = Docx::default();

  docx.create_para().new_run().text("hello, world");

  docx.write_file("origin.docx").unwrap();

  let mut docx = Docx::from_file("origin.docx").unwrap();

  docx.create_para().new_run().text("world, hello");

  docx.write_file("origin_appended.docx").unwrap();
}
