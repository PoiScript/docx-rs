#![recursion_limit = "256"]

extern crate proc_macro;

mod into_owned;
mod types;
mod xml_read;
mod xml_write;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam};

use crate::{types::Element, xml_read::read, xml_write::write};

#[proc_macro_derive(XmlRead, attributes(xml))]
pub fn derive_xml_read(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;

    let lifetime = match &generics.params.last() {
        Some(GenericParam::Lifetime(lt)) => Some(lt),
        _ => None,
    };

    let element = Element::parse(&input);

    let impl_read = read(&element);

    let gen = quote! {
        impl #generics #name #generics {
            pub(crate) fn from_str(string: & #lifetime str) -> Result<#name #generics> {
                let mut reader = xmlparser::Tokenizer::from(string).peekable();
                Self::from_reader(&mut reader)
            }

            pub(crate) fn from_reader(
                reader: &mut std::iter::Peekable<xmlparser::Tokenizer #generics>
            ) -> Result<#name #generics> {
                use xmlparser::{ElementEnd, Token, Tokenizer};

                #impl_read
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(XmlWrite, attributes(xml))]
pub fn derive_xml_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;

    let element = Element::parse(&input);

    let impl_write = write(&element);

    let gen = quote! {
        impl #generics #name #generics {
            pub(crate) fn write<W: std::io::Write>(&self, mut writer: W) -> Result<()> {
                #impl_write

                Ok(())
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(IntoOwned)]
pub fn derive_into_owned(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;

    let element = Element::parse(&input);

    let impl_into_owned = into_owned::impl_into_owned(&element);

    let lifetime = match &generics.params.last() {
        Some(GenericParam::Lifetime(_)) => Some(quote!( <'static> )),
        _ => None,
    };

    let gen = quote! {
        impl #generics #name #generics {
            pub fn into_owned(self) -> #name #lifetime {
                #impl_into_owned
            }
        }
    };

    gen.into()
}
