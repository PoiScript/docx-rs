//! Style Definitions part
//!
//! The corresponding ZIP item is `/word/styles.xml`.

use crate::errors::{Error, Result};
use crate::schema::SCHEMA_MAIN;
use quick_xml::events::BytesStart;
use std::borrow::Borrow;

/// The root element of the styles of the document
///
/// Styles are predefined sets of properties which can be applied to text.
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:styles")]
#[xml(extend_attrs = "styles_extend_attrs")]
pub struct Styles {
    /// Specifies the default set of properties.
    #[xml(child = "w:docDefaults")]
    pub default: Option<DefaultStyle>,
    /// Specifies a set of properties.
    #[xml(child = "w:style")]
    pub styles: Vec<Style>,
}

#[inline]
fn styles_extend_attrs(_: &Styles, start: &mut BytesStart) {
    start.push_attribute(("xmlns:w", SCHEMA_MAIN));
}

impl Styles {
    /// Appends a style to the back of the styles, and returns it.
    pub fn create_style(&mut self) -> &mut Style {
        self.styles.push(Style::default());
        self.styles.last_mut().unwrap()
    }
}

/// The root element of the default style
///
/// This style is inherited by every paragraph and run.
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:docDefaults")]
pub struct DefaultStyle {
    #[xml(child = "w:rPrDefault")]
    pub char: Option<DefaultCharStyle>,
    #[xml(child = "w:pPrDefault")]
    pub para: Option<DefaultParaStyle>,
}

/// The root element of the default character properties
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:rPrDefault")]
pub struct DefaultCharStyle {
    /// Specifies a set of character properties
    #[xml(child = "w:rPr")]
    pub inner: CharStyle,
}

/// The root element of the default paragraph properties
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:pPrDefault")]
pub struct DefaultParaStyle {
    /// Specifies a set of paragraph properties
    #[xml(child = "w:pPr")]
    pub inner: ParaStyle,
}

/// The root element of a style
///
/// This style is applied to a region of a document.
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:style")]
#[xml(extend_attrs = "style_extend_attrs")]
pub struct Style {
    /// Specifies the primary name and the unique identifier
    ///
    /// This identifier is used throughout the document to apply style in content.
    #[xml(child = "w:name")]
    pub name: Option<StyleName>,
    /// Specifies a set of paragraph properties
    #[xml(child = "w:pPr")]
    pub para: Option<ParaStyle>,
    /// Specifies a set of character properties
    #[xml(child = "w:rPr")]
    pub char: Option<CharStyle>,
}

w_val_element!(StyleName, "w:name", String);

#[inline]
fn style_extend_attrs(s: &Style, start: &mut BytesStart) {
    start.push_attribute(("w:type", "paragraph"));
    if let Some(ref name) = s.name {
        start.push_attribute(("w:styleId", name.value.borrow()));
    }
}

impl Style {
    /// Setting the name of this style
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(StyleName::new(name.to_owned()));
        self
    }

    /// Resetting the name of this style
    pub fn reset_name(&mut self) -> &mut Self {
        self.name = None;
        self
    }

    /// Returns the paragraph properties
    pub fn para(&mut self) -> &mut ParaStyle {
        self.para.get_or_insert(ParaStyle::default())
    }

    /// Returns the character properties
    pub fn char(&mut self) -> &mut CharStyle {
        self.char.get_or_insert(CharStyle::default())
    }
}

/// The root element of a set of character properties
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:rPr")]
pub struct CharStyle {
    /// Specifies the color to be used to display text.
    #[xml(child = "w:color")]
    pub color: Option<Color>,
    /// Specifies the font size in half points.
    #[xml(child = "w:sz")]
    pub sz: Option<Size>,
    /// Specifies that the text of the text run is to be bold.
    #[xml(child = "w:b")]
    pub bold: Option<Bold>,
    /// Specifies that the text of the text run is to be italics.
    #[xml(child = "w:i")]
    pub italics: Option<Italics>,
    /// Specifies that the contents are to be displayed with a horizontal line through the center of the line.
    #[xml(child = "w:strike")]
    pub strike: Option<Strike>,
    #[xml(child = "w:dstrike")]
    pub dstrike: Option<Dstrike>,
    #[xml(child = "w:outline")]
    pub outline: Option<Outline>,
    #[xml(child = "w:u")]
    pub underline: Option<Underline>,
}

w_val_element!(Color, "w:color", String);
w_val_element!(Size, "w:sz", usize);
w_val_element!(Bold, "w:bold", bool);
w_val_element!(Italics, "w:i", bool);
w_val_element!(Strike, "w:strike", bool);
w_val_element!(Dstrike, "w:dstrike", bool);
w_val_element!(Outline, "w:outline", bool);

impl CharStyle {
    pub fn sz(&mut self, sz: usize) -> &mut Self {
        self.sz = Some(Size::new(sz));
        self
    }

    pub fn reset_sz(&mut self) -> &mut Self {
        self.sz = None;
        self
    }

    pub fn color(&mut self, color: &str) -> &mut Self {
        self.color = Some(Color::new(color.to_owned()));
        self
    }

    pub fn reset_color(&mut self) -> &mut Self {
        self.color = None;
        self
    }

    pub fn bold(&mut self, val: bool) -> &mut Self {
        self.bold = Some(Bold::new(val));
        self
    }

    pub fn reset_bold(&mut self) -> &mut Self {
        self.bold = None;
        self
    }

    pub fn italics(&mut self, val: bool) -> &mut Self {
        self.italics = Some(Italics::new(val));
        self
    }

    pub fn reset_italics(&mut self) -> &mut Self {
        self.italics = None;
        self
    }

    pub fn strike(&mut self, val: bool) -> &mut Self {
        self.strike = Some(Strike::new(val));
        if let Some(Dstrike { value: true }) = self.dstrike {
            self.dstrike = None;
        }
        self
    }

    pub fn reset_strike(&mut self) -> &mut Self {
        self.strike = None;
        self
    }

    pub fn dstrike(&mut self, val: bool) -> &mut Self {
        self.dstrike = Some(Dstrike::new(val));
        if let Some(Strike { value: true }) = self.strike {
            self.strike = None;
        }
        self
    }

    pub fn reset_dstrike(&mut self) -> &mut Self {
        self.dstrike = None;
        self
    }

    pub fn outline(&mut self, val: bool) -> &mut Self {
        self.outline = Some(Outline::new(val));
        self
    }

    pub fn reset_outline(&mut self) -> &mut Self {
        self.outline = None;
        self
    }

    pub fn underline(&mut self, color: Option<&str>, ty: Option<UnderlineStyle>) -> &mut Self {
        self.underline = Some(Underline {
            color: color.map(|c| c.to_owned()),
            val: ty,
        });
        self
    }

    pub fn reset_underline(&mut self) -> &mut Self {
        self.underline = None;
        self
    }
}

#[derive(Debug, Default, Xml)]
#[xml(tag = "w:u")]
#[xml(leaf)]
pub struct Underline {
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    #[xml(attr = "w:val")]
    pub val: Option<UnderlineStyle>,
}

#[derive(Debug)]
pub enum UnderlineStyle {
    Dash,
    DashDotDotHeavy,
    DashDotHeavy,
    DashedHeavy,
    DashLong,
    DashLongHeavy,
    DotDash,
    DotDotDash,
    Dotted,
    DottedHeavy,
    Double,
    None,
    Single,
    Thick,
    Wave,
    WavyDouble,
    WavyHeavy,
    Words,
}

string_enum! {
    UnderlineStyle {
        Dash = "dash",
        DashDotDotHeavy = "dashDotDotHeavy",
        DashDotHeavy = "dashDotHeavy",
        DashedHeavy = "dashedHeavy",
        DashLong = "dashLong",
        DashLongHeavy = "dashLongHeavy",
        DotDash = "dotDash",
        DotDotDash = "dotDotDash",
        Dotted = "dotted",
        DottedHeavy = "dottedHeavy",
        Double = "double",
        None = "none",
        Single = "single",
        Thick = "thick",
        Wave = "wave",
        WavyDouble = "wavyDouble",
        WavyHeavy = "wavyHeavy",
        Words = "words",
    }
}

/// The root element of a set of paragraph properties
#[derive(Debug, Default, Xml)]
#[xml(tag = "w:pPr")]
pub struct ParaStyle {
    #[xml(child = "w:pStyle")]
    pub name: Option<ParaStyleId>,
    #[xml(child = "w:jc")]
    pub jc: Option<Jc>,
    #[xml(child = "w:pBdr")]
    pub border: Option<Borders>,
    #[xml(child = "w:numBdr")]
    pub num: Option<Numbers>,
}

w_val_element!(ParaStyleId, "w:pStyle", String);
w_val_element!(Jc, "w:jc", Justification);

impl ParaStyle {
    pub fn jc(&mut self, jc: Justification) -> &mut Self {
        self.jc = Some(Jc::new(jc));
        self
    }

    pub fn reset_jc(&mut self) -> &mut Self {
        self.jc = None;
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(ParaStyleId::new(name.to_owned()));
        self
    }

    pub fn reset_name(&mut self) -> &mut Self {
        self.name = None;
        self
    }
}

#[derive(Debug)]
pub enum Justification {
    Start,
    End,
    Center,
    Both,
    Distribute,
    Right,
    Left,
}

string_enum! {
    Justification {
        Start = "start",
        End = "end",
        Center = "center",
        Both = "both",
        Distribute = "distribute",
        Right = "right",
        Left = "left",
    }
}

#[derive(Debug, Default, Xml)]
#[xml(tag = "w:pPr")]
pub struct Borders {
    #[xml(child = "w:top")]
    pub top: Option<TopBorder>,
    #[xml(child = "w:bottom")]
    pub botton: Option<BottomBorder>,
    #[xml(child = "w:left")]
    pub left: Option<LeftBorder>,
    #[xml(child = "w:right")]
    pub right: Option<RightBorder>,
    #[xml(child = "w:between")]
    pub between: Option<BetweenBorder>,
}

#[derive(Debug, Default, Xml)]
#[xml(leaf)]
#[xml(tag = "w:top")]
pub struct TopBorder {
    #[xml(attr = "w:val")]
    pub color: Option<String>,
    #[xml(attr = "w:val")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:val")]
    pub space: Option<usize>,
    #[xml(attr = "w:val")]
    pub sz: Option<usize>,
    #[xml(attr = "w:val")]
    pub val: Option<BorderStyle>,
}

#[derive(Debug, Default, Xml)]
#[xml(leaf)]
#[xml(tag = "w:bottom")]
pub struct BottomBorder {
    #[xml(attr = "w:val")]
    pub color: Option<String>,
    #[xml(attr = "w:val")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:val")]
    pub space: Option<usize>,
    #[xml(attr = "w:val")]
    pub sz: Option<usize>,
    #[xml(attr = "w:val")]
    pub val: Option<BorderStyle>,
}

#[derive(Debug, Default, Xml)]
#[xml(leaf)]
#[xml(tag = "w:left")]
pub struct LeftBorder {
    #[xml(attr = "w:val")]
    pub color: Option<String>,
    #[xml(attr = "w:val")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:val")]
    pub space: Option<usize>,
    #[xml(attr = "w:val")]
    pub sz: Option<usize>,
    #[xml(attr = "w:val")]
    pub val: Option<BorderStyle>,
}

#[derive(Debug, Default, Xml)]
#[xml(leaf)]
#[xml(tag = "w:right")]
pub struct RightBorder {
    #[xml(attr = "w:val")]
    pub color: Option<String>,
    #[xml(attr = "w:val")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:val")]
    pub space: Option<usize>,
    #[xml(attr = "w:val")]
    pub sz: Option<usize>,
    #[xml(attr = "w:val")]
    pub val: Option<BorderStyle>,
}

#[derive(Debug, Default, Xml)]
#[xml(leaf)]
#[xml(tag = "w:between")]
pub struct BetweenBorder {
    #[xml(attr = "w:val")]
    pub color: Option<String>,
    #[xml(attr = "w:val")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:val")]
    pub space: Option<usize>,
    #[xml(attr = "w:val")]
    pub sz: Option<usize>,
    #[xml(attr = "w:val")]
    pub val: Option<BorderStyle>,
}

#[derive(Debug)]
pub enum BorderStyle {
    Single,
    DashDotStroked,
    Dashed,
    DashSmallGap,
    DotDash,
    DotDotDash,
    Dotted,
    Double,
    DoubleWave,
    Inset,
    Nil,
    None,
    Outset,
    Thick,
    ThickThinLargeGap,
    ThickThinMediumGap,
    ThickThinSmallGap,
    ThinThickLargeGap,
    ThinThickMediumGap,
    ThinThickSmallGap,
    ThinThickThinLargeGap,
    ThinThickThinMediumGap,
    ThinThickThinSmallGap,
    ThreeDEmboss,
    ThreeDEngrave,
    Triple,
    Wave,
}

string_enum! {
    BorderStyle {
        Single = "single",
        DashDotStroked = "dashDotStroked",
        Dashed = "dashed",
        DashSmallGap = "dashSmallGap",
        DotDash = "dotDash",
        DotDotDash = "dotDotDash",
        Dotted = "dotted",
        Double = "double",
        DoubleWave = "doubleWave",
        Inset = "inset",
        Nil = "nil",
        None = "none",
        Outset = "outset",
        Thick = "thick",
        ThickThinLargeGap = "thickThinLargeGap",
        ThickThinMediumGap = "thickThinMediumGap",
        ThickThinSmallGap = "thickThinSmallGap",
        ThinThickLargeGap = "thinThickLargeGap",
        ThinThickMediumGap = "thinThickMediumGap",
        ThinThickSmallGap = "thinThickSmallGap",
        ThinThickThinLargeGap = "thinThickThinLargeGap",
        ThinThickThinMediumGap = "thinThickThinMediumGap",
        ThinThickThinSmallGap = "thinThickThinSmallGap",
        ThreeDEmboss = "threeDEmboss",
        ThreeDEngrave = "threeDEngrave",
        Triple = "triple",
        Wave = "wave",
    }
}

#[derive(Debug, Xml)]
#[xml(tag = "w:numPr")]
pub struct Numbers {
    /// Specifies a reference to a numbering definition instance
    #[xml(child = "w:numId")]
    pub id: NumId,
    /// Specifies the numbering level of the numbering definition to use for the paragraph.
    #[xml(child = "w:ilvl")]
    pub level: NumLvl,
}

w_val_element!(NumId, "w:numId", usize);
w_val_element!(NumLvl, "w:ilvl", usize);
