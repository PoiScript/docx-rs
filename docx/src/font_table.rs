//! Font Table part
//!
//! The corresponding ZIP item is `/word/fontTable.xml`.

use docx_codegen::Xml;
use quick_xml::events::BytesStart;

use crate::{
    __w_val_element,
    error::{Error, Result},
    schema::{SCHEMA_MAIN, SCHEMA_RELATIONSHIPS},
};

#[derive(Debug, Default, Xml)]
#[xml(tag = "w:fonts")]
#[xml(extend_attrs = "font_table_extend_attrs")]
pub struct FontTable {
    #[xml(child = "w:font")]
    pub fonts: Vec<Font>,
}

#[inline]
fn font_table_extend_attrs(_: &FontTable, start: &mut BytesStart) {
    start.push_attribute(("xmlns:w", SCHEMA_MAIN));
    start.push_attribute(("xmlns:r", SCHEMA_RELATIONSHIPS));
}

#[derive(Debug, Default, Xml)]
#[xml(tag = "w:font")]
pub struct Font {
    #[xml(attr = "w:name")]
    pub name: String,
    #[xml(child = "w:val")]
    pub charset: Option<Charset>,
    #[xml(child = "w:family")]
    pub family: Option<Family>,
    #[xml(child = "w:pitch")]
    pub pitch: Option<Pitch>,
}

__w_val_element!(Charset, "w:charset", String);
__w_val_element!(Family, "w:fmaily", String);
__w_val_element!(Pitch, "w:pitch", String);
