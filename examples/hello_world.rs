use docx::{
    document::{Paragraph, Run, TextSpace},
    formatting::{CharacterProperty, JustificationVal, ParagraphProperty},
    styles::{DefaultStyle, Style, StyleType},
    Docx, DocxResult,
};

fn main() -> DocxResult<()> {
    // Create an empty docx
    let mut docx = Docx::default();

    // Change the default text size and text color
    docx.styles.default(
        DefaultStyle::default().character(
            CharacterProperty::default()
                .size(42usize)
                .color((0x00, 0xff, 0x00)),
        ),
    );

    // Create a new paragraph style called `TestStyle`
    docx.styles.push(
        Style::new(StyleType::Paragraph, "TestStyle")
            .name("Test Style")
            .character(CharacterProperty::default().color(0xff0000)), // override the default text color
    );

    // Insert a left-aligned, bold paragraph
    let para = Paragraph::default()
        .property(
            ParagraphProperty::default()
                .style_id("TestStyle") // inherites from `TestStyle`
                .justification(JustificationVal::Start),
        )
        .push(
            Run::default()
                .property(CharacterProperty::default().bold(true))
                .push_text("hello, world"),
        );

    docx.document.add_content(para);

    // Insert a centered paragraph with an outline
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

    docx.document.add_content(para);

    // Insert a right-aligned, italics paragraph
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

    docx.document.add_content(para);

    docx.write_file("hello_world.docx")?;

    Ok(())
}
