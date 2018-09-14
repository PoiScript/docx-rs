extern crate proc_macro;
extern crate regex;

mod read;
pub(crate) mod types;
mod write;

use proc_macro::TokenStream;
use read::{impl_read, impl_read_with_attrs};
use std::str::FromStr;
use types::parse_struct;
use write::impl_write;

#[proc_macro_derive(Xml, attributes(xml))]
pub fn xml(input: TokenStream) -> TokenStream {
  let s = parse_struct(input.to_string());

  let xml_impl = format!(
    r#"impl {0} {{
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
    impl_write(&s),
    impl_read_with_attrs(&s),
    impl_read(&s)
  );

  TokenStream::from_str(&xml_impl).unwrap()
}
