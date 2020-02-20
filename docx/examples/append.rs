use docx::{document::Paragraph, Docx, DocxFile, DocxResult};

fn main() -> DocxResult<()> {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .try_init();

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
