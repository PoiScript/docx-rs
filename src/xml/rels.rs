use quick_xml::events::*;
use std::collections::LinkedList;

use events_list::EventListExt;
use xml::Xml;

static RELATIONSHIPS_NAMESPACES: [(&'static str, &'static str); 1] = [(
  "xmlns",
  "http://schemas.openxmlformats.org/package/2006/relationships",
)];

pub enum RelType {
  Core,
  Extended,
  OfficeDocument,
  Theme,
  Settings,
  FontTable,
  Styles,
  Image,
  Numbering,
  Hyperlink,
  Footnotes,
  EndNotes,
  Comments,
  CustomXml,
}

impl RelType {
  fn get_schemas(&self) -> &'static str {
    match self {
      &RelType::Core => "http://schemas.openxmlformats.org/officedocument/2006/relationships/metadata/core-properties",
      &RelType::Extended => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
      &RelType::OfficeDocument  => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
      &RelType::Theme => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
      &RelType::Settings => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
      &RelType::FontTable => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
      &RelType::Styles => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink",
      &RelType::Image => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
      &RelType::Numbering => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes",
      &RelType::Hyperlink => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes",
      &RelType::Footnotes => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
      &RelType::EndNotes => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
      &RelType::Comments => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
      &RelType::CustomXml => "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
    }
  }
}

pub struct RelsXml<'a> {
  relationships: Vec<(RelType, &'a str)>,
}

impl<'a> RelsXml<'a> {
  /// Return default relationships for document.xml
  pub fn document() -> RelsXml<'a> {
    RelsXml {
      relationships: vec![
        (RelType::Styles, "styles.xml"),
        (RelType::FontTable, "fontTable.xml"),
        (RelType::Settings, "settings.xml"),
      ],
    }
  }

  pub fn add_rel(&mut self, rel: (RelType, &'a str)) {
    self.relationships.push(rel);
  }
}

impl<'a> Xml<'a> for RelsXml<'a> {
  /// Return default relationships for the whole package
  fn default() -> RelsXml<'a> {
    RelsXml {
      relationships: vec![
        (RelType::Core, "docProps/core.xml"),
        (RelType::Extended, "docProps/app.xml"),
        (RelType::OfficeDocument, "word/document.xml"),
      ],
    }
  }

  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    for (i, (rel_type, target)) in self.relationships.iter().enumerate() {
      events.push_back(Event::Empty(
        BytesStart::borrowed(b"Relationship", b"Relationship".len()).with_attributes(vec![
          ("Id", format!("rId{}", i).as_str()),
          ("Target", target),
          ("Type", rel_type.get_schemas()),
        ]),
      ));
    }

    events.warp_attrs_tag("Relationships", RELATIONSHIPS_NAMESPACES.to_vec());

    events
  }
}
