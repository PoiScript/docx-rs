use std::collections::LinkedList;

use utility::LinkUtil;

pub struct ContentTypesXml<'a> {
  defaults: Vec<(&'a str, &'a str)>,
  overrides: Vec<(&'a str, &'a str)>,
}

impl<'a> ContentTypesXml<'a> {
  pub fn default() -> ContentTypesXml<'a> {
    ContentTypesXml {
      defaults: vec![
        (
          "rels",
          "application/vnd.openxmlformats-package.relationships+xml",
        ),
        ("xml", "application/xml"),
      ],
      overrides: vec![
        (
          "/word/document.xml",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
        ),
        (
          "/word/styles.xml",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml",
        ),
        (
          "/docProps/app.xml",
          "application/vnd.openxmlformats-officedocument.extended-properties+xml",
        ),
        (
          "/word/settings.xml",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
        ),
        (
          "/word/theme/theme1.xml",
          "application/vnd.openxmlformats-officedocument.theme+xml",
        ),
        (
          "/word/fontTable.xml",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml",
        ),
        (
          "/word/webSettings.xml",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml",
        ),
        (
          "/docProps/core.xml",
          "application/vnd.openxmlformats-package.core-properties+xml",
        ),
      ],
    }
  }

  pub fn generate(&self) -> Vec<u8> {
    let mut events = LinkedList::new();

    for i in 0..self.defaults.len() {
      let (part_name, content_type) = self.defaults[i];
      events.add_tag_with_attr(
        b"Default",
        vec![("PartName", part_name), ("ContentType", content_type)],
      );
    }

    for i in 0..self.overrides.len() {
      let (part_name, content_type) = self.overrides[i];
      events.add_tag_with_attr(
        b"Override",
        vec![("PartName", part_name), ("ContentType", content_type)],
      );
    }

    events
      .wrap_tag_with_attr(
        b"Types",
        vec![(
          "xmlns",
          "http://schemas.openxmlformats.org/package/2006/content-types",
        )],
      )
      .add_decl()
      .to_xml()
  }
}
