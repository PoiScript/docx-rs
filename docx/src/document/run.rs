use derive_more::From;
use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter,
    document::{r#break::Break, text::Text},
    error::{Error, Result},
    formatting::CharacterProperty,
};

/// Run
///
/// Run is a non-block region of text with properties.
///
/// ```rust
/// use docx::document::*;
/// use docx::formatting::*;
///
/// let run = Run::default()
///     .prop(CharacterProperty::default())
///     .push_text("text")
///     .push_break(None)
///     .push_text((" text ", TextSpace::Preserve))
///     .push_break(BreakType::Column);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:r")]
pub struct Run<'a> {
    /// Specifies the properties of a run
    ///
    /// Just as paragraph, a run's properties is applied to all the contents of the run.
    #[xml(child = "w:rPr")]
    pub prop: Option<CharacterProperty<'a>>,
    #[xml(child = "w:t", child = "w:br")]
    /// Specifies the content of a run
    pub content: Vec<RunContent<'a>>,
}

impl<'a> Run<'a> {
    __setter!(prop: Option<CharacterProperty<'a>>);

    #[inline(always)]
    pub fn push<T: Into<RunContent<'a>>>(mut self, content: T) -> Self {
        self.content.push(content.into());
        self
    }

    #[inline(always)]
    pub fn push_text<T: Into<Text<'a>>>(mut self, content: T) -> Self {
        self.content.push(RunContent::Text(content.into()));
        self
    }

    #[inline(always)]
    pub fn push_break<T: Into<Break>>(mut self, br: T) -> Self {
        self.content.push(RunContent::Break(br.into()));
        self
    }

    pub fn iter_text(&self) -> impl Iterator<Item = &Cow<'a, str>> {
        self.content.iter().filter_map(|content| match content {
            RunContent::Text(Text { text, .. }) => Some(text),
            RunContent::Break(_) => None,
        })
    }

    pub fn iter_text_mut(&mut self) -> impl Iterator<Item = &mut Cow<'a, str>> {
        self.content.iter_mut().filter_map(|content| match content {
            RunContent::Text(Text { text, .. }) => Some(text),
            RunContent::Break(_) => None,
        })
    }
}

/// A set of elements that can be contained as the content of a run.
#[derive(Debug, From, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RunContent<'a> {
    #[xml(tag = "w:t")]
    Text(Text<'a>),
    #[xml(tag = "w:br")]
    Break(Break),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Run,
        Run::default(),
        r#"<w:r></w:r>"#,
        Run::default().prop(CharacterProperty::default()),
        r#"<w:r><w:rPr></w:rPr></w:r>"#,
        Run::default().push_break(None),
        r#"<w:r><w:br/></w:r>"#,
        Run::default().push_text(""),
        r#"<w:r><w:t></w:t></w:r>"#,
    );
}
