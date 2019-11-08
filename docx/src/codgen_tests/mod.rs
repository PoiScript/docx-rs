use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::error::{Error, Result};

#[derive(XmlWrite, XmlRead, PartialEq, Debug, IntoOwned)]
#[xml(tag = "tag1")]
struct Tag1<'a> {
    #[xml(attr = "att1")]
    att1: Option<Cow<'a, str>>,
    #[xml(text)]
    content: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, IntoOwned)]
#[xml(tag = "tag2")]
#[xml(leaf)]
struct Tag2<'a> {
    #[xml(attr = "att1")]
    att1: Cow<'a, str>,
    #[xml(attr = "att2")]
    att2: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, IntoOwned)]
#[xml(tag = "tag3")]
struct Tag3<'a> {
    #[xml(attr = "att1")]
    att1: Cow<'a, str>,
    #[xml(child = "tag1")]
    tag1: Vec<Tag1<'a>>,
    #[xml(child = "tag2")]
    tag2: Option<Tag2<'a>>,
    #[xml(flatten_text = "text")]
    text: Option<Cow<'a, str>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, IntoOwned)]
enum Tag<'a> {
    #[xml(tag = "tag1")]
    Tag1(Tag1<'a>),
    #[xml(tag = "tag2")]
    Tag2(Tag2<'a>),
    #[xml(tag = "tag3")]
    Tag3(Tag3<'a>),
}

macro_rules! test_suite {
    ($func:ident, $type:tt, $string:tt, $struct:expr) => {
        #[test]
        fn $func() -> Result<()> {
            let _ = env_logger::builder()
                .is_test(true)
                .format_timestamp(None)
                .try_init();

            // test writing
            let mut writer = vec![];
            let element = $struct;
            element.write(&mut writer)?;
            assert_eq!($string, String::from_utf8(writer)?);

            // test reading
            assert_eq!($struct, $type::from_str($string)?);

            Ok(())
        }
    };
}

test_suite!(
    tag3_1,
    Tag3,
    r#"<tag3 att1="att1"><tag1>content</tag1><text>tag3_content</text></tag3>"#,
    Tag3 {
        att1: "att1".into(),
        tag1: vec![Tag1 {
            att1: None,
            content: "content".into(),
        }],
        tag2: None,
        text: Some("tag3_content".into()),
    }
);

test_suite!(
    tag3_2,
    Tag3,
    r#"<tag3 att1="att1"><tag1 att1="att11">content1</tag1><tag1 att1="att12">content2</tag1></tag3>"#,
    Tag3 {
        att1: "att1".into(),
        tag1: vec![
            Tag1 {
                att1: Some("att11".into()),
                content: "content1".into(),
            },
            Tag1 {
                att1: Some("att12".into()),
                content: "content2".into(),
            },
        ],
        tag2: None,
        text: None,
    }
);

test_suite!(
    tag1,
    Tag,
    r#"<tag1 att1="att1">content</tag1>"#,
    Tag::Tag1(Tag1 {
        att1: Some("att1".into()),
        content: "content".into(),
    })
);

test_suite!(
    tag,
    Tag,
    r#"<tag2 att1="att1" att2="att2"/>"#,
    Tag::Tag2(Tag2 {
        att1: "att1".into(),
        att2: "att2".into(),
    })
);
