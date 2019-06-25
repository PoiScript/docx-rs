#[macro_use]
extern crate docx_codegen;

use docx::errors::{Error, Result};
use quick_xml::{Reader, Writer};
use std::io::Cursor;

#[derive(Xml, PartialEq, Debug)]
#[xml(tag = "tag1")]
struct Tag1 {
    #[xml(attr = "att1")]
    att1: Option<String>,
    #[xml(text)]
    content: String,
}

#[derive(Xml, PartialEq, Debug)]
#[xml(tag = "tag2")]
#[xml(leaf)]
struct Tag2 {
    #[xml(attr = "att1")]
    att1: String,
    #[xml(attr = "att2")]
    att2: String,
}

#[derive(Xml, PartialEq, Debug)]
#[xml(tag = "tag3")]
struct Tag3 {
    #[xml(attr = "att1")]
    att1: String,
    #[xml(child = "tag1")]
    tag1: Vec<Tag1>,
    #[xml(child = "tag2")]
    tag2: Option<Tag2>,
    #[xml(flatten_text = "text")]
    text: Option<String>,
}

#[derive(Xml, PartialEq, Debug)]
enum Tag {
    #[xml(tag = "tag1")]
    Tag1(Tag1),
    #[xml(tag = "tag2")]
    Tag2(Tag2),
    #[xml(tag = "tag3")]
    Tag3(Tag3),
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

        assert_eq!($t::read(&mut reader, None).unwrap(), $r);
    };
}

#[test]
fn test_write() {
    assert_write_eq!(
        r#"<tag3 att1="att1"><tag1 att1="tag1_att1">tag1_content</tag1><tag2 att1="tag2_att1" att2="tag2_att2"/></tag3>"#,
        Tag3 {
            att1: String::from("att1"),
            tag1: vec![Tag1 {
                att1: Some("tag1_att1".to_string()),
                content: "tag1_content".to_string(),
            }],
            tag2: Some(Tag2 {
                att1: "tag2_att1".to_string(),
                att2: "tag2_att2".to_string(),
            }),
            text: None,
        }
    );

    assert_write_eq!(
        r#"<tag3 att1="att1"><tag1>tag1_content</tag1><text>tag3_content</text><tag4/><tag5 att1="tag5_att1"/></tag3>"#,
        Tag3 {
            att1: "att1".to_string(),
            tag1: vec![Tag1 {
                att1: None,
                content: "tag1_content".to_string(),
            }],
            tag2: None,
            text: Some("tag3_content".to_string()),
        }
    );

    assert_write_eq!(
        r#"<tag3 att1="att1"><tag1>content</tag1><tag1>tag1</tag1><text>tag3_content</text><tag5 att1="tag5_att1"/></tag3>"#,
        Tag3 {
            att1: "att1".to_string(),
            tag1: vec![
                Tag1 {
                    att1: None,
                    content: "content".to_string(),
                },
                Tag1 {
                    att1: None,
                    content: "tag1".to_string(),
                },
            ],
            tag2: None,
            text: Some("tag3_content".to_string()),
        }
    );

    assert_write_eq!(
        r#"<tag1>tag1_content</tag1>"#,
        Tag::Tag1(Tag1 {
            att1: None,
            content: "tag1_content".to_string(),
        })
    );
}

#[test]
fn test_read() {
    assert_read_eq!(
        Tag3,
        r#"<tag3 att1="att1"><text>tag3_content</text><tag2 att2="att2" att1="att1"/><tag1 att1="att1">content</tag1></tag3>"#,
        Tag3 {
            att1: "att1".to_string(),
            tag1: vec![Tag1 {
                att1: Some("att1".to_string()),
                content: "content".to_string(),
            }],
            tag2: Some(Tag2 {
                att1: "att1".to_string(),
                att2: "att2".to_string(),
            }),
            text: Some("tag3_content".to_string()),
        }
    );

    assert_read_eq!(
        Tag3,
        r#"<tag3 att1="att1"><tag1>content</tag1><text>tag3_content</text><tag4/><tag5 att1="tag5_att1"/></tag3>"#,
        Tag3 {
            att1: "att1".to_string(),
            tag1: vec![Tag1 {
                att1: None,
                content: "content".to_string(),
            }],
            tag2: None,
            text: Some("tag3_content".to_string()),
        }
    );

    assert_read_eq!(
        Tag3,
        r#"<tag3 att1="att1"><tag1 att1="att11">content1</tag1><tag1 att1="att12">content2</tag1></tag3>"#,
        Tag3 {
            att1: "att1".to_string(),
            tag1: vec![
                Tag1 {
                    att1: Some("att11".to_string()),
                    content: "content1".to_string(),
                },
                Tag1 {
                    att1: Some("att12".to_string()),
                    content: "content2".to_string(),
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
            att1: Some("att1".to_string()),
            content: "content".to_string(),
        })
    );

    assert_read_eq!(
        Tag,
        r#"<tag2 att2="att2" att1="att1"/>"#,
        Tag::Tag2(Tag2 {
            att1: "att1".to_string(),
            att2: "att2".to_string(),
        })
    );
}
