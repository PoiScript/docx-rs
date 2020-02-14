use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    error::{Error, Result},
    style::CharacterStyle,
};

use super::{r#break::Break, text::Text};

/// The root element of a run within the paragraph
///
/// Run is a non-block region of text with properties.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:r")]
pub struct Run<'a> {
    /// Specifies the properties of a run
    ///
    /// Just as paragraph, a run's properties is applied to all the contents of the run.
    #[xml(child = "w:rPr")]
    pub prop: Option<CharacterStyle<'a>>,
    #[xml(child = "w:t", child = "w:br")]
    /// Specifies the content of a run
    pub content: Vec<RunContent<'a>>,
}

impl<'a> Run<'a> {
    /// Creates a new run containing the given text
    pub fn text<T: Into<Text<'a>>>(t: T) -> Self {
        Run {
            prop: None,
            content: vec![RunContent::Text(t.into())],
        }
    }

    /// Returns the properties of this run.
    pub fn prop(&mut self) -> &mut CharacterStyle<'a> {
        self.prop.get_or_insert(CharacterStyle::default())
    }

    /// Appends a break to the back of this run.
    pub fn text_break(&mut self) -> &mut Self {
        self.content.push(RunContent::Break(Break { ty: None }));
        self
    }
}

/// A set of elements that can be contained as the content of a run.
#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
pub enum RunContent<'a> {
    #[xml(tag = "w:t")]
    Text(Text<'a>),
    #[xml(tag = "w:br")]
    Break(Break),
}
