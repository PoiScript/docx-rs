extern crate proc_macro;
extern crate regex;

mod read;
pub(crate) mod types;
mod write;

use proc_macro::TokenStream;
use read::{impl_read_enum, impl_read_struct, impl_read_with_attrs_struct};
use std::str::FromStr;
use types::{parse_enum, parse_struct};
use write::{impl_write_enum, impl_write_struct};

#[proc_macro_derive(XmlStruct, attributes(xml))]
pub fn xml_struct(input: TokenStream) -> TokenStream {
  let s = parse_struct(input.to_string());

  let xml_impl = format!(
    r#"impl XmlStruct for {0} {{
  fn write<W>(&self, writer: &mut quick_xml::Writer<W>) -> Result<()>
  where
    W: std::io::Write + std::io::Seek,
  {{
    use quick_xml::events::*;

    {1}
  }}
  fn read_with_attrs(
     attrs: quick_xml::events::attributes::Attributes,
     reader: &mut quick_xml::Reader<&[u8]>
  ) -> {0} {{
    use quick_xml::events::*;

    {2}
  }}
  fn read(reader: &mut quick_xml::Reader<&[u8]>) -> {0} {{
    use quick_xml::events::*;

    {3}
  }}
}}"#,
    s.name,
    impl_write_struct(&s),
    impl_read_with_attrs_struct(&s),
    impl_read_struct(&s)
  );

  TokenStream::from_str(&xml_impl).unwrap()
}

#[proc_macro_derive(XmlEnum, attributes(xml))]
pub fn xml_enum(input: TokenStream) -> TokenStream {
  let e = parse_enum(input.to_string());

  let xml_impl = format!(
    r#"impl XmlEnum for {0} {{
  fn write<W>(&self, writer: &mut quick_xml::Writer<W>) -> Result<()>
  where
    W: std::io::Write + std::io::Seek,
  {{
    use quick_xml::events::*;

    {1}
  }}

  fn read_with_attrs(
     attrs: quick_xml::events::attributes::Attributes,
     tag: &[u8],
     reader: &mut quick_xml::Reader<&[u8]>
  ) -> {0} {{
    use quick_xml::events::*;

    {2}
  }}
}}"#,
    e.name,
    impl_write_enum(&e),
    impl_read_enum(&e)
  );

  TokenStream::from_str(&xml_impl).unwrap()
}
