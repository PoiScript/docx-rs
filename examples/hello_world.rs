extern crate docx_rs;

use docx_rs::{Docx, Justification};

fn main() {
  let path = std::path::Path::new("hello_world.docx");
  let file = std::fs::File::create(&path).unwrap();

  let mut docx = Docx::new();

  docx
    .create_style()
    .with_name("TestStyle")
    .with_sz(42)
    .with_color("ff0000")
    .with_jc(&Justification::Center);

  docx
    .create_para()
    .add_text("hello, world")
    .with_style_name("TestStyle");

  docx.generate(file).unwrap();
}
