extern crate docx_rs;

use docx_rs::{Docx, Para};

fn main() {
  let path = std::path::Path::new("hello_world.docx");
  let file = std::fs::File::create(&path).unwrap();

  let mut docx = Docx::new();

  let para = Para::new("hello, world");
  docx.append_para(para);

  docx.generate(file).unwrap();
}
