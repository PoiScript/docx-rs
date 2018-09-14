#[macro_use]
extern crate codegen;
extern crate docx;
extern crate quick_xml;

use docx::errors::Result;
use quick_xml::{Reader, Writer};
use std::io::Cursor;

#[derive(Xml, PartialEq, Debug)]
#[xml(text = "tag1")]
struct Tag1 {
  #[xml(attr = "att1")]
  pub att1: Option<String>,
  #[xml(text)]
  pub content: String,
}

#[derive(Xml, PartialEq, Debug)]
#[xml(empty = "tag2")]
struct Tag2 {
  #[xml(attr = "att1")]
  pub att1: String,
  #[xml(attr = "att2")]
  pub att2: String,
}

#[derive(Xml, PartialEq, Debug)]
#[xml(parent = "tag3")]
struct Tag3 {
  #[xml(attr = "att1")]
  pub att1: String,
  #[xml(child = "tag1")]
  pub tag1: Tag1,
  #[xml(child = "tag2")]
  pub tag2: Option<Tag2>,
}

#[test]
fn test_write() {
  let elem = Tag3 {
    att1: String::from("att1"),
    tag1: Tag1 {
      att1: Some(String::from("tag1_att1")),
      content: String::from("tag1_content"),
    },
    tag2: Some(Tag2 {
      att1: String::from("tag2_att1"),
      att2: String::from("tag2_att2"),
    }),
  };

  let mut writer = Writer::new(Cursor::new(Vec::new()));
  elem.write(&mut writer).unwrap();
  let result = writer.into_inner().into_inner();

  assert_eq!(
    r#"<tag3 att1="att1"><tag1 att1="tag1_att1">tag1_content</tag1><tag2 att1="tag2_att1" att2="tag2_att2"/></tag3>"#,
    String::from_utf8(result).unwrap()
  );

  let elem = Tag3 {
    att1: String::from("att1"),
    tag1: Tag1 {
      att1: None,
      content: String::from("tag1_content"),
    },
    tag2: None,
  };

  let mut writer = Writer::new(Cursor::new(Vec::new()));
  elem.write(&mut writer).unwrap();
  let result = writer.into_inner().into_inner();

  assert_eq!(
    r#"<tag3 att1="att1"><tag1>tag1_content</tag1></tag3>"#,
    String::from_utf8(result).unwrap()
  );
}

#[test]
fn test_read() {
  let xml =
    r#"<tag3 att1="att1"><tag2 att2="att2" att1="att1"/><tag1 att1="att1">content</tag1></tag3>"#;
  let mut reader = Reader::from_str(xml);
  reader.trim_text(true);

  assert_eq!(
    Tag3 {
      att1: String::from("att1"),
      tag1: Tag1 {
        att1: Some(String::from("att1")),
        content: String::from("content"),
      },
      tag2: Some(Tag2 {
        att1: String::from("att1"),
        att2: String::from("att2"),
      }),
    },
    Tag3::read(&mut reader)
  );

  let xml = r#"<tag3 att1="att1"><tag1>content</tag1></tag3>"#;
  let mut reader = Reader::from_str(xml);
  reader.trim_text(true);

  assert_eq!(
    Tag3 {
      att1: String::from("att1"),
      tag1: Tag1 {
        att1: None,
        content: String::from("content"),
      },
      tag2: None,
    },
    Tag3::read(&mut reader)
  );
}
