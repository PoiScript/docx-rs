#![recursion_limit = "128"]

extern crate proc_macro;

mod read;
mod types;
mod write;

use proc_macro::TokenStream;
use quote::quote;
use read::impl_read;
use syn::{parse_macro_input, DeriveInput};
use types::Element;
use write::impl_write;

#[proc_macro_derive(Xml, attributes(xml))]
pub fn derive_xml(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let element = Element::parse(&input);

    let name = &input.ident;
    let generics = &input.generics;

    let impl_write = impl_write(&element);
    let impl_read = impl_read(&element);

    let gen = quote! {
            impl #generics #name #generics {
                pub fn write<W>(&self, w: &mut ::quick_xml::Writer<W>) -> Result<()>
                where
                    W: ::std::io::Write,
                {
                    use quick_xml::events::*;

                    #impl_write

                    Ok(())
                }

                pub fn read<B>(
                    r: &mut ::quick_xml::Reader<B>,
                    bs: Option<::quick_xml::events::BytesStart>,
                ) -> Result<Self>
                where
                    B: ::std::io::BufRead,
                {
                    use quick_xml::events::*;
                    use std::borrow::Borrow;
                    use std::convert::AsRef;
                    use std::str::FromStr;

                    #impl_read
                }
            }
        };

    gen.into()
}
