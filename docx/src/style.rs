//! Style Definitions part
//!
//! The corresponding ZIP item is `/word/styles.xml`.

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;
use std::io::Write;

use crate::{
    __string_enum,
    error::{Error, Result},
    schema::SCHEMA_MAIN,
};

/// The root element of the styles of the document
///
/// Styles are predefined sets of properties which can be applied to text.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:styles")]
#[xml(extend_attrs = "styles_extend_attrs")]
pub struct Styles<'a> {
    /// Specifies the default set of properties.
    #[xml(child = "w:docDefaults")]
    pub default: Option<DefaultStyle<'a>>,
    /// Specifies a set of properties.
    #[xml(child = "w:style")]
    pub styles: Vec<Style<'a>>,
}

#[inline]
fn styles_extend_attrs<W: Write>(_: &Styles, mut w: W) -> Result<()> {
    write!(w, " xmlns:w=\"{}\"", SCHEMA_MAIN)?;
    Ok(())
}

impl<'a> Styles<'a> {
    /// Appends a style to the back of the styles, and returns it.
    pub fn create_style(&mut self) -> &mut Style<'a> {
        self.styles.push(Style::default());
        self.styles.last_mut().unwrap()
    }
}

/// The root element of the default style
///
/// This style is inherited by every paragraph and run.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:docDefaults")]
pub struct DefaultStyle<'a> {
    #[xml(child = "w:rPrDefault")]
    pub char: Option<DefaultCharStyle<'a>>,
    #[xml(child = "w:pPrDefault")]
    pub para: Option<DefaultParaStyle<'a>>,
}

/// The root element of the default character properties
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:rPrDefault")]
pub struct DefaultCharStyle<'a> {
    /// Specifies a set of character properties
    #[xml(child = "w:rPr")]
    pub inner: CharStyle<'a>,
}

/// The root element of the default paragraph properties
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:pPrDefault")]
pub struct DefaultParaStyle<'a> {
    /// Specifies a set of paragraph properties
    #[xml(child = "w:pPr")]
    pub inner: ParaStyle<'a>,
}

/// The root element of a style
///
/// This style is applied to a region of a document.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:style")]
#[xml(extend_attrs = "style_extend_attrs")]
pub struct Style<'a> {
    /// Specifies the primary name and the unique identifier
    ///
    /// This identifier is used throughout the document to apply style in content.
    #[xml(child = "w:name")]
    pub name: Option<StyleName<'a>>,
    /// Specifies a set of paragraph properties
    #[xml(child = "w:pPr")]
    pub para: Option<ParaStyle<'a>>,
    /// Specifies a set of character properties
    #[xml(child = "w:rPr")]
    pub char: Option<CharStyle<'a>>,
}

#[inline]
fn style_extend_attrs<W: Write>(s: &Style, mut w: W) -> Result<()> {
    write!(w, " w:type=\"paragraph\"")?;
    if let Some(ref name) = s.name {
        write!(w, " w:styleId=\"{}\"", name.value)?;
    }
    Ok(())
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:name")]
pub struct StyleName<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a> StyleName<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(value: S) -> Self {
        StyleName {
            value: value.into(),
        }
    }
}

impl<'a> Style<'a> {
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
    pub fn para(&mut self) -> &mut ParaStyle<'a> {
        self.para.get_or_insert(ParaStyle::default())
    }

    /// Returns the character properties
    pub fn char(&mut self) -> &mut CharStyle<'a> {
        self.char.get_or_insert(CharStyle::default())
    }
}

/// The root element of a set of character properties
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:rPr")]
pub struct CharStyle<'a> {
    /// Specifies the color to be used to display text.
    #[xml(child = "w:color")]
    pub color: Option<Color<'a>>,
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
    pub underline: Option<Underline<'a>>,
}

impl<'a> CharStyle<'a> {
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

    pub fn underline(&mut self, color: Option<&'a str>, ty: Option<UnderlineStyle>) -> &mut Self {
        self.underline = Some(Underline {
            color: color.map(Into::into),
            val: ty,
        });
        self
    }

    pub fn reset_underline(&mut self) -> &mut Self {
        self.underline = None;
        self
    }
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:color")]
pub struct Color<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a> Color<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(value: S) -> Self {
        Color {
            value: value.into(),
        }
    }
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:sz")]
pub struct Size {
    #[xml(attr = "w:val")]
    pub value: usize,
}

impl Size {
    pub fn new(value: usize) -> Self {
        Size { value }
    }
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:bold")]
pub struct Bold {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl Bold {
    pub fn new(value: bool) -> Self {
        Bold { value }
    }
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:i")]
pub struct Italics {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl Italics {
    pub fn new(value: bool) -> Self {
        Italics { value }
    }
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:strike")]
pub struct Strike {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl Strike {
    pub fn new(value: bool) -> Self {
        Strike { value }
    }
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:dstrike")]
pub struct Dstrike {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl Dstrike {
    pub fn new(value: bool) -> Self {
        Dstrike { value }
    }
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:outline")]
pub struct Outline {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl Outline {
    pub fn new(value: bool) -> Self {
        Outline { value }
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:u")]
pub struct Underline<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
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

__string_enum! {
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
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:pPr")]
pub struct ParaStyle<'a> {
    #[xml(child = "w:pStyle")]
    pub name: Option<ParaStyleId<'a>>,
    #[xml(child = "w:jc")]
    pub jc: Option<Jc>,
    #[xml(child = "w:pBdr")]
    pub border: Option<Borders<'a>>,
    #[xml(child = "w:numBdr")]
    pub num: Option<Numbers>,
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:pStyle")]
pub struct ParaStyleId<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a> ParaStyleId<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(value: S) -> Self {
        ParaStyleId {
            value: value.into(),
        }
    }
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:jc")]
pub struct Jc {
    #[xml(attr = "w:val")]
    pub value: Justification,
}

impl Jc {
    pub fn new(value: Justification) -> Self {
        Jc { value }
    }
}

impl<'a> ParaStyle<'a> {
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

__string_enum! {
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

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:pPr")]
pub struct Borders<'a> {
    #[xml(child = "w:top")]
    pub top: Option<TopBorder<'a>>,
    #[xml(child = "w:bottom")]
    pub botton: Option<BottomBorder<'a>>,
    #[xml(child = "w:left")]
    pub left: Option<LeftBorder<'a>>,
    #[xml(child = "w:right")]
    pub right: Option<RightBorder<'a>>,
    #[xml(child = "w:between")]
    pub between: Option<BetweenBorder<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:top")]
pub struct TopBorder<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>,
    #[xml(attr = "w:val")]
    pub style: Option<BorderStyle>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:bottom")]
pub struct BottomBorder<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>,
    #[xml(attr = "w:val")]
    pub style: Option<BorderStyle>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:left")]
pub struct LeftBorder<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>,
    #[xml(attr = "w:val")]
    pub style: Option<BorderStyle>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:right")]
pub struct RightBorder<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>,
    #[xml(attr = "w:val")]
    pub style: Option<BorderStyle>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:between")]
pub struct BetweenBorder<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>,
    #[xml(attr = "w:val")]
    pub style: Option<BorderStyle>,
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

__string_enum! {
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

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:numPr")]
pub struct Numbers {
    /// Specifies a reference to a numbering definition instance
    #[xml(child = "w:numId")]
    pub id: NumId,
    /// Specifies the numbering level of the numbering definition to use for the paragraph.
    #[xml(child = "w:ilvl")]
    pub level: NumLvl,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:numId")]
pub struct NumId {
    #[xml(attr = "w:val")]
    pub value: usize,
}

impl NumId {
    pub fn new(value: usize) -> Self {
        NumId { value }
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:ilvl")]
pub struct NumLvl {
    #[xml(attr = "w:val")]
    pub value: usize,
}

impl NumLvl {
    pub fn new(value: usize) -> Self {
        NumLvl { value }
    }
}
