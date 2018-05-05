/// docProps/app.xml

use std::io::Cursor;
use quick_xml::events::*;
use quick_xml::Writer;

use utility::add_tag;

pub struct AppXml<'a> {
  template: &'a str,
  total_time: &'a str,
  pages: &'a str,
  words: &'a str,
  characters: &'a str,
  applications: &'a str,
  doc_security: &'a str,
  lines: &'a str,
  paragraphs: &'a str,
  scale_crop: &'a str,
  company: &'a str,
  links_up_to_date: &'a str,
  characters_with_spaces: &'a str,
  shared_doc: &'a str,
  hyperlinks_changed: &'a str,
  app_version: &'a str,
}

impl<'a> AppXml<'a> {
  pub fn default() -> AppXml<'a> {
    AppXml {
      template: "Normal.dotm",
      total_time: "1",
      pages: "1",
      words: "0",
      characters: "0",
      applications: "docx-rs",
      doc_security: "0",
      lines: "0",
      paragraphs: "1",
      scale_crop: "false",
      company: "MS",
      links_up_to_date: "false",
      characters_with_spaces: "25",
      shared_doc: "false",
      hyperlinks_changed: "false",
      app_version: "12.0000",
    }
  }

  pub fn generate(&self) -> Vec<u8> {
    let mut events = vec![
      Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))),
      Event::Start(BytesStart::borrowed(b"Properties", b"Properties".len())
        .with_attributes(vec![
          ("xmlns", "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"),
          ("xmlns:vt", "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes")
        ]))
    ];

    add_tag(&mut events, b"Template", self.template);
    add_tag(&mut events, b"TotalTime", self.total_time);
    add_tag(&mut events, b"Pages", self.pages);
    add_tag(&mut events, b"Words", self.words);
    add_tag(&mut events, b"Characters", self.characters);
    add_tag(&mut events, b"Application", self.applications);
    add_tag(&mut events, b"DocSecurity", self.doc_security);
    add_tag(&mut events, b"Lines", self.lines);
    add_tag(&mut events, b"Paragraphs", self.paragraphs);
    add_tag(&mut events, b"ScaleCrop", self.scale_crop);
    add_tag(&mut events, b"Company", self.company);
    add_tag(&mut events, b"LinksUpToDate", self.links_up_to_date);
    add_tag(&mut events, b"CharactersWithSpaces", self.characters_with_spaces);
    add_tag(&mut events, b"SharedDoc", self.shared_doc);
    add_tag(&mut events, b"HyperlinksChanged", self.hyperlinks_changed);
    add_tag(&mut events, b"AppVersion", self.app_version);

    events.push(Event::End(BytesEnd::borrowed(b"Properties")));

    let mut writer = Writer::new(Cursor::new(Vec::new()));

    for event in events {
      writer.write_event(event);
    }

    writer.into_inner().into_inner()
  }
}
