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

    test_style.name("TestStyle");

    test_style.char().sz(42).color("ff0000");
  }

  {
    let para = docx.create_para();

    para.prop().name("TestStyle").jc(Justification::Start);

    let run = para.new_run();

    run.text("hello, world");

    run.prop().bold(true);
  }

  {
    let para = docx.create_para();

    para.prop().name("TestStyle").jc(Justification::Center);

    let run = para.new_run();

    run.text("hello, world");

    run.prop().outline(true);
  }

  {
    let para = docx.create_para();

    para.prop().name("TestStyle").jc(Justification::End);

    let run = para.new_run();

    run.text("hello, world");

    run.prop().strike(true);
  }

  docx.generate(file).unwrap();
}
