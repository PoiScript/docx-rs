#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod read;
pub(crate) mod types;
mod write;

use proc_macro::TokenStream;
use read::impl_read;
use syn::{Data, DeriveInput};
use types::{Enum, Item, Struct};
use write::impl_write;

#[proc_macro_derive(Xml, attributes(xml))]
pub fn derive_xml(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let item = match input.data {
    Data::Enum(ref data) => Item::Enum(Enum::parse(data, &input.ident, &input.generics)),
    Data::Struct(ref data) => Item::Struct(Struct::parse(
      data,
      &input.attrs,
      &input.ident,
      &input.generics,
    )),
    Data::Union(_) => panic!("#[derive(Xml)] doesn't support Union."),
  };

  let name = &input.ident;
  let generics = &input.generics;

  let impl_write = impl_write(&item);
  let impl_read = impl_read(&item);

  let gen = quote!{
    impl #generics Xml for #name #generics {
      fn write<W>(&self, w: &mut ::quick_xml::Writer<W>) -> Result<()>
      where
        W: ::std::io::Write,
      {
        use quick_xml::events::*;

        #impl_write
      }

      fn read<B>(
        r: &mut ::quick_xml::Reader<B>,
        bs: Option<&::quick_xml::events::BytesStart>,
      ) -> Result<#name #generics>
      where
        B: ::std::io::BufRead,
      {
        use quick_xml::events::*;

        #impl_read
      }
    }
  };

  gen.into()
}
