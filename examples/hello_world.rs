use docx::{
    document::{Paragraph, Run, TextSpace},
    formatting::{CharacterProperty, JustificationVal, ParagraphProperty},
    styles::{DefaultStyle, Style, StyleType},
    Docx, DocxResult,
};

fn main() -> DocxResult<()> {
    let mut docx = Docx::default();

    docx.styles.default(
        DefaultStyle::default().character(
            CharacterProperty::default()
                .size(42usize)
                .color((0x00, 0xff, 0x00)),
        ),
    );

    docx.styles.push(
        // create a new paragraph style called `TestStyle`
        Style::new(StyleType::Paragraph, "TestStyle")
            .name("Test Style")
            .character(CharacterProperty::default().color(0xff0000)), // override default font color
    );

    let para = Paragraph::default()
        .property(
            ParagraphProperty::default()
                .style_id("TestStyle") // inherite from `TestStyle`
                .justification(JustificationVal::Start),
        )
        .push(
            Run::default()
                .property(CharacterProperty::default().bold(true))
                .push_text("hello, world"),
        );

    docx.document.push(para);

    let para = Paragraph::default()
        .property(
            ParagraphProperty::default()
                .style_id("TestStyle")
                .justification(JustificationVal::Center),
        )
        .push(
            Run::default()
                .property(CharacterProperty::default().outline(true))
                .push_text("hello, world"),
        );

    docx.document.push(para);

    let para = Paragraph::default()
        .property(
            ParagraphProperty::default()
                .style_id("TestStyle")
                .justification(JustificationVal::End),
        )
        .push(
            Run::default()
                .property(CharacterProperty::default().italics(true))
                .push_text(("hello, ", TextSpace::Preserve))
                .push_text("world"),
        );

    docx.document.push(para);

    docx.write_file("hello_world.docx")?;

    Ok(())
}
