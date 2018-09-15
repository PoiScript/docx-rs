extern crate proc_macro;
extern crate proc_macro2;
extern crate regex;
#[macro_use]
extern crate quote;

mod read;
mod read_with_bytes_start;
pub(crate) mod types;
mod write;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Span;
use read::{impl_read_enum, impl_read_struct};
use read_with_bytes_start::{impl_read_with_bytes_start_enum, impl_read_with_bytes_start_struct};
use types::{parse_enum, parse_struct};
use write::{impl_write_enum, impl_write_struct};

#[proc_macro_derive(XmlStruct, attributes(xml))]
pub fn xml_struct(input: TokenStream) -> TokenStream {
  let s = parse_struct(input.to_string());

  let name = Ident::new(&s.name, Span::call_site());
  let write = impl_write_struct(&s);
  let read_with_bytes_start = impl_read_with_bytes_start_struct(&s);
  let read = impl_read_struct(&s);

  let gen = quote!{
    impl XmlStruct for #name {
      fn write<W>(&self, w: &mut quick_xml::Writer<W>) -> Result<()>
      where
        W: std::io::Write + std::io::Seek,
      {
        use quick_xml::events::*;

        #write
      }
      fn read_with_bytes_start(
        bs: &quick_xml::events::BytesStart,
        r: &mut quick_xml::Reader<&[u8]>
      ) -> #name {
        use quick_xml::events::*;

        #read_with_bytes_start
      }
      fn read(r: &mut quick_xml::Reader<&[u8]>) -> #name {
        use quick_xml::events::*;

        #read
      }
    }
  };

  gen.into()
}

#[proc_macro_derive(XmlEnum, attributes(xml))]
pub fn xml_enum(input: TokenStream) -> TokenStream {
  let e = parse_enum(input.to_string());

  let name = Ident::new(&e.name, Span::call_site());
  let write = impl_write_enum(&e);
  let read_with_bytes_start = impl_read_with_bytes_start_enum(&e);
  let read = impl_read_enum(&e);

  let gen = quote!{
    impl XmlEnum for #name {
      fn write<W>(&self, w: &mut quick_xml::Writer<W>) -> Result<()>
      where
        W: std::io::Write + std::io::Seek,
      {
        use quick_xml::events::*;

        #write
      }
      fn read_with_bytes_start(
        bs: &quick_xml::events::BytesStart,
        r: &mut quick_xml::Reader<&[u8]>
      ) -> #name {
        use quick_xml::events::*;

        #read_with_bytes_start
      }
      fn read(r: &mut quick_xml::Reader<&[u8]>) -> #name {
        use quick_xml::events::*;

        #read
      }
    }
  };

  gen.into()
}
