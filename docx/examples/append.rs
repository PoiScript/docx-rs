use docx::{document::Paragraph, Docx, DocxFile, Result};

fn main() -> Result<()> {
    let mut docx = Docx::default();

    let para = Paragraph::default().push_text("hello, world");

    docx.document.push(para);

    docx.write_file("origin.docx")?;

    let docx = DocxFile::from_file("origin.docx")?;

    let mut docx = docx.parse()?;

    let para = Paragraph::default().push_text("world, hello");

    docx.document.push(para);

    docx.write_file("origin_appended.docx")?;

    Ok(())
}
