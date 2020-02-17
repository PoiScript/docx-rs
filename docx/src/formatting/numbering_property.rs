use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    error::{Error, Result},
    formatting::{IndentLevel, NumberingId},
};

/// Numbering Property
///
/// ```rust
/// use docx::formatting::*;
///
/// let prop = NumberingProperty::from((20, 40));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:numPr")]
pub struct NumberingProperty {
    /// Specifies a reference to a numbering definition instance
    #[xml(child = "w:numId")]
    pub id: NumberingId,
    /// Specifies the numbering level of the numbering definition to use for the paragraph.
    #[xml(child = "w:ilvl")]
    pub level: IndentLevel,
}

impl From<(usize, usize)> for NumberingProperty {
    fn from(val: (usize, usize)) -> Self {
        NumberingProperty {
            id: NumberingId { value: val.0 },
            level: IndentLevel { value: val.1 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        NumberingProperty,
        NumberingProperty::default(),
        r#"<w:numPr><w:numId w:val="0"/><w:ilvl w:val="0"/></w:numPr>"#,
        NumberingProperty::from((20, 40)),
        r#"<w:numPr><w:numId w:val="20"/><w:ilvl w:val="40"/></w:numPr>"#,
    );
}
