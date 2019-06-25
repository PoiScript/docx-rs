use docx::prelude::*;
use docx::style::Justification;
use docx::Result;

fn main() -> Result<()> {
    let mut docx = Docx::default();

    let mut test_style = Style::default();
    test_style.name("TestStyle").char().sz(42).color("ff0000");
    docx.insert_style(test_style);

    let mut para = Para::default();
    para.prop().name("TestStyle").jc(Justification::Start);
    let mut run = Run::text("hello, world");
    run.prop().bold(true);
    para.run(run);
    docx.insert_para(para);

    let mut para = Para::default();
    para.prop().name("TestStyle").jc(Justification::Center);
    let mut run = Run::text("hello, world");
    run.prop().outline(true);
    para.run(run);
    docx.insert_para(para);

    let mut para = Para::default();
    para.prop().name("TestStyle").jc(Justification::End);
    let mut run = Run::text("hello, world");
    run.prop().strike(true);
    para.run(run);
    docx.insert_para(para);

    docx.write_file("hello_world.docx")?;

    Ok(())
}
