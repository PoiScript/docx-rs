extern crate docx_rs;

use docx_rs::{Docx, Justification, Para, Style};

fn main() {
  let path = std::path::Path::new("hello_world.docx");
  let file = std::fs::File::create(&path).unwrap();

  // NOTE: style is passed into docx as a reference,
  // so we declare style struct before docx here
  // to make sure it's dropped after the docx.
  let style = Style::default()
    .with_name("TestStyle")
    .with_sz(42)
    .with_color("ff0000")
    .with_jc(&Justification::Center);
  let para = Para::new("hello, world");

  let mut docx = Docx::new();
  docx.append_para(para, &style);

  docx.generate(file).unwrap();
}
