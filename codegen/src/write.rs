use proc_macro2::TokenStream;
use quote::quote;
use syn::LitByteStr;

use crate::types::{Element, Field, TypeExt};

pub fn impl_write(element: &Element) -> TokenStream {
    match element {
        Element::Enum(e) => {
            let name = &e.name;

            let matches = e
                .elements
                .iter()
                .map(|(var, _)| &var.name)
                .map(|var_name| quote! { #name::#var_name(s) => s.write(w)? });

            quote! {
                match self {
                    #( #matches, )*
                }
            }
        }
        Element::Text(t) => {
            let tag = &t.tag;
            let write_attrs = t.attributes.iter().map(|(fld, tag)| write_attrs(fld, tag));
            let extend_attrs = &t.extend_attrs;
            let text_field_name = &t.text_field.name;

            quote! {
                let mut start = BytesStart::borrowed(#tag, #tag.len());

                #( #write_attrs )*

                #extend_attrs(&self, &mut start);

                w.write_event(Event::Start(start))?;
                w.write_event(Event::Text(BytesText::from_plain_str(self.#text_field_name.as_ref())))?;
                w.write_event(Event::End(BytesEnd::borrowed(#tag)))?;
            }
        }
        Element::Parent(p) => {
            let tag = &p.tag;
            let write_attrs = p.attributes.iter().map(|(fld, tag)| write_attrs(fld, tag));
            let extend_attrs = &p.extend_attrs;
            let write_children = p
                .children
                .iter()
                .chain(p.leaf_children.iter())
                .map(|(field, _)| write_child(field));
            let write_flatten_text = p
                .flatten_text
                .iter()
                .map(|(field, tag)| write_flatten_text(field, tag));

            quote! {
                let mut start = BytesStart::borrowed(#tag, #tag.len());

                #( #write_attrs )*

                #extend_attrs(&self, &mut start);

                w.write_event(Event::Start(start))?;

                #( #write_children )*
                #( #write_flatten_text )*

                w.write_event(Event::End(BytesEnd::borrowed(#tag)))?;
            }
        }
        Element::Leaf(l) => {
            let tag = &l.tag;
            let write_attrs = l.attributes.iter().map(|(fld, tag)| write_attrs(fld, tag));
            let extend_attrs = &l.extend_attrs;

            quote! {
                let mut start = BytesStart::borrowed(#tag, #tag.len());

                #( #write_attrs )*

                #extend_attrs(&self, &mut start);

                w.write_event(Event::Empty(start))?;
            }
        }
    }
}

fn write_attrs(field: &Field, tag: &LitByteStr) -> TokenStream {
    let name = &field.name;
    let ty = &field.ty;

    if let Some(ty) = ty.is_option() {
        if ty.is_bool() || ty.is_usize() {
            quote! {
                if let Some(ref #name) = self.#name {
                    start.push_attribute((#tag as &[u8], #name.to_string().as_bytes()));
                }
            }
        } else if ty.is_string() {
            quote! {
                if let Some(ref #name) = self.#name {
                    start.push_attribute((#tag as &[u8], #name.as_bytes()));
                }
            }
        } else {
            quote! {
                if let Some(ref #name) = self.#name {
                    start.push_attribute((#tag as &[u8], #name.as_ref()));
                }
            }
        }
    } else {
        if ty.is_bool() || ty.is_usize() {
            quote! { start.push_attribute((#tag as &[u8], self.#name.to_string().as_bytes())); }
        } else if ty.is_string() {
            quote! { start.push_attribute((#tag as &[u8], self.#name.as_bytes())); }
        } else {
            quote! { start.push_attribute((#tag as &[u8], self.#name.as_ref())); }
        }
    }
}

fn write_child(field: &Field) -> TokenStream {
    let name = &field.name;
    let ty = &field.ty;

    if ty.is_option().is_some() {
        quote! {
            if let Some(ref #name) = self.#name {
                #name.write(w)?;
            }
        }
    } else if ty.is_vec().is_some() {
        quote! {
            for #name in &self.#name {
                #name.write(w)?;
            }
        }
    } else {
        quote! {
            self.#name.write(w)?;
        }
    }
}

fn write_flatten_text(field: &Field, tag: &LitByteStr) -> TokenStream {
    let name = &field.name;
    let ty = &field.ty;

    if ty.is_option().is_some() {
        quote! {
            if let Some(s) = &self.#name {
                w.write_event(Event::Start(BytesStart::borrowed(#tag, #tag.len())))?;
                w.write_event(Event::Text(BytesText::from_plain_str(&s)))?;
                w.write_event(Event::End(BytesEnd::borrowed(#tag)))?;
            }
        }
    } else {
        quote! {
            w.write_event(Event::Start(BytesStart::borrowed(#tag, #tag.len())))?;
            w.write_event(Event::Text(BytesText::from_plain_str(&self.#name)))?;
            w.write_event(Event::End(BytesEnd::borrowed(#tag)))?;
        }
    }
}
