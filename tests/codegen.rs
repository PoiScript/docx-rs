extern crate docx;
#[macro_use]
extern crate docx_codegen;
extern crate quick_xml;

use docx::errors::Result;
use docx::xml::{XmlEnum, XmlStruct};
use quick_xml::{Reader, Writer};
use std::io::Cursor;

#[derive(XmlStruct, PartialEq, Debug, Default)]
#[xml(text = "tag1")]
struct Tag1 {
  #[xml(attr = "att1")]
  pub att1: Option<String>,
  #[xml(text)]
  pub content: String,
}

#[derive(XmlStruct, PartialEq, Debug, Default)]
#[xml(empty = "tag2")]
struct Tag2 {
  #[xml(attr = "att1")]
  pub att1: String,
  #[xml(attr = "att2")]
  pub att2: String,
}

#[derive(XmlStruct, PartialEq, Debug, Default)]
#[xml(parent = "tag3")]
struct Tag3 {
  #[xml(attr = "att1")]
  pub att1: String,
  #[xml(child = "tag1")]
  pub tag1: Vec<Tag1>,
  #[xml(child = "tag2")]
  pub tag2: Option<Tag2>,
  #[xml(text = "text")]
  pub text: Option<String>,
}

#[derive(XmlEnum, PartialEq, Debug)]
enum Tag {
  #[xml(text = "tag1")]
  Tag1(Tag1),
  #[xml(empty = "tag2")]
  Tag2(Tag2),
  #[xml(parent = "tag3")]
  Tag3(Tag3),
}

impl std::default::Default for Tag {
  fn default() -> Tag {
    Tag::Tag1(Tag1::default())
  }
}

macro_rules! assert_write_eq {
  ($l:tt, $r:expr) => {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    $r.write(&mut writer).unwrap();
    let result = writer.into_inner().into_inner();

    assert_eq!($l, String::from_utf8(result).unwrap());
  };
}

macro_rules! assert_read_eq {
  ($t:tt, $l:tt, $r:expr) => {
    let mut reader = Reader::from_str($l);
    reader.trim_text(true);

    assert_eq!($t::read(&mut reader).unwrap(), $r);
  };
}

#[test]
fn test_write() {
  assert_write_eq!(
    r#"<tag3 att1="att1"><tag1 att1="tag1_att1">tag1_content</tag1><tag2 att1="tag2_att1" att2="tag2_att2"/></tag3>"#,
    Tag3 {
      att1: String::from("att1"),
      tag1: vec![Tag1 {
        att1: Some(String::from("tag1_att1")),
        content: String::from("tag1_content"),
      }],
      tag2: Some(Tag2 {
        att1: String::from("tag2_att1"),
        att2: String::from("tag2_att2"),
      }),
      text: None,
    }
  );

  assert_write_eq!(
    r#"<tag3 att1="att1"><tag1>tag1_content</tag1><text>tag3_content</text></tag3>"#,
    Tag3 {
      att1: String::from("att1"),
      tag1: vec![Tag1 {
        att1: None,
        content: String::from("tag1_content"),
      }],
      tag2: None,
      text: Some(String::from("tag3_content")),
    }
  );

  assert_write_eq!(
    r#"<tag3 att1="att1"><tag1>content</tag1><tag1>tag1</tag1><text>tag3_content</text></tag3>"#,
    Tag3 {
      att1: String::from("att1"),
      tag1: vec![
        Tag1 {
          att1: None,
          content: String::from("content"),
        },
        Tag1 {
          att1: None,
          content: String::from("tag1"),
        },
      ],
      tag2: None,
      text: Some(String::from("tag3_content")),
    }
  );

  assert_write_eq!(
    r#"<tag1>tag1_content</tag1>"#,
    Tag::Tag1(Tag1 {
      att1: None,
      content: String::from("tag1_content"),
    })
  );
}

#[test]
fn test_read() {
  assert_read_eq!(
    Tag3,
    r#"<tag3 att1="att1"><text>tag3_content</text><tag2 att2="att2" att1="att1"/><tag1 att1="att1">content</tag1></tag3>"#,
    Tag3 {
      att1: String::from("att1"),
      tag1: vec![Tag1 {
        att1: Some(String::from("att1")),
        content: String::from("content"),
      }],
      tag2: Some(Tag2 {
        att1: String::from("att1"),
        att2: String::from("att2"),
      }),
      text: Some(String::from("tag3_content")),
    }
  );

  assert_read_eq!(
    Tag3,
    r#"<tag3 att1="att1"><tag1>content</tag1><text>tag3_content</text></tag3>"#,
    Tag3 {
      att1: String::from("att1"),
      tag1: vec![Tag1 {
        att1: None,
        content: String::from("content"),
      }],
      tag2: None,
      text: Some(String::from("tag3_content")),
    }
  );

  assert_read_eq!(
    Tag3,
    r#"<tag3 att1="att1"><tag1 att1="att11">content1</tag1><tag1 att1="att12">content2</tag1></tag3>"#,
    Tag3 {
      att1: String::from("att1"),
      tag1: vec![
        Tag1 {
          att1: Some(String::from("att11")),
          content: String::from("content1"),
        },
        Tag1 {
          att1: Some(String::from("att12")),
          content: String::from("content2"),
        },
      ],
      tag2: None,
      text: None,
    }
  );

  assert_read_eq!(
    Tag,
    r#"<tag1 att1="att1">content</tag1>"#,
    Tag::Tag1(Tag1 {
      att1: Some(String::from("att1")),
      content: String::from("content"),
    })
  );

  assert_read_eq!(
    Tag,
    r#"<tag2 att2="att2" att1="att1"/>"#,
    Tag::Tag2(Tag2 {
      att1: String::from("att1"),
      att2: String::from("att2"),
    })
  );
}
