use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    error::{Error, Result},
    formatting::{IndentLevel, NumberingId},
};

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

    #[test]
    fn test_convert() {
        assert_eq!(
            NumberingProperty {
                id: NumberingId { value: 20 },
                level: IndentLevel { value: 40 },
            },
            (20, 40).into(),
        );
    }
}
