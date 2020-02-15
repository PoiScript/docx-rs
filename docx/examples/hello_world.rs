use docx::{
    document::{Paragraph, Run, TextSpace},
    style::{CharacterStyle, DefaultStyle, JustificationVal, ParagraphStyle, Style},
    Docx, Result,
};

fn main() -> Result<()> {
    let mut docx = Docx::default();

    docx.styles.default(
        DefaultStyle::default().char(CharacterStyle::default().sz(42usize).color("00ff00")),
    );

    docx.styles.push(
        // create a new style called `TestStyle`
        Style::default()
            .name("TestStyle")
            .char(CharacterStyle::default().color("ff0000")), // override default font color
    );

    let para = Paragraph::default()
        .prop(
            ParagraphStyle::default()
                .name("TestStyle") // inherite from `TestStyle`
                .justification(JustificationVal::Start),
        )
        .push(
            Run::default()
                .prop(CharacterStyle::default().bold(true))
                .push_text("hello, world"),
        );

    docx.document.push(para);

    let para = Paragraph::default()
        .prop(
            ParagraphStyle::default()
                .name("TestStyle")
                .justification(JustificationVal::Center),
        )
        .push(
            Run::default()
                .prop(CharacterStyle::default().outline(true))
                .push_text("hello, world"),
        );

    docx.document.push(para);

    let para = Paragraph::default()
        .prop(
            ParagraphStyle::default()
                .name("TestStyle")
                .justification(JustificationVal::End),
        )
        .push(
            Run::default()
                .prop(CharacterStyle::default().italics(true))
                .push_text(("hello, ", TextSpace::Preserve))
                .push_text("world"),
        );

    docx.document.push(para);

    docx.write_file("hello_world.docx")?;

    Ok(())
}
