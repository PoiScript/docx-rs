extern crate docx;

use docx::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
  let path = Path::new("hello_world.docx");
  let file = File::create(&path).unwrap();

  let mut docx = Docx::default();

  {
    let test_style = docx.create_style();

    test_style.with_name("TestStyle");

    test_style.char_style().with_sz("42").with_color("ff0000");
  }

  {
    let para = docx.create_para();

    para.new_run().add_text("hello, world");

    para.get_style().with_name("TestStyle").with_jc("start");
  }

  {
    let para = docx.create_para();

    para.new_run().add_text("hello, world");

    para.get_style().with_name("TestStyle").with_jc("center");
  }

  {
    let para = docx.create_para();

    para.new_run().add_text("hello, world");

    para.get_style().with_name("TestStyle").with_jc("end");
  }

  docx.generate(file).unwrap();
}
