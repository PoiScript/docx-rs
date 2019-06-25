use docx::prelude::*;
use docx::Result;

fn main() -> Result<()> {
    let mut docx = Docx::default();

    let mut para = Para::default();
    para.text("hello, world");
    docx.insert_para(para);

    docx.write_file("origin.docx")?;

    let mut docx = Docx::from_file("origin.docx")?;

    let mut para = Para::default();
    para.text("world, hello");
    docx.insert_para(para);

    docx.write_file("origin_appended.docx")?;

    Ok(())
}
