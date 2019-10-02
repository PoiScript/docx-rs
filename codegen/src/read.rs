use proc_macro2::TokenStream;
use quote::quote;
use std::iter::once;
use syn::{Ident, LitByteStr};

use crate::types::Field;
use crate::types::*;

pub fn impl_read(element: &Element) -> TokenStream {
    match element {
        Element::Enum(e) => read_enum(e),
        Element::Leaf(l) => {
            let name = &l.name;
            let init_values = l.attributes.iter().map(|e| init_value(&e.0));

            let set_attrs = set_attrs(name, &l.tag, &l.attributes);

            let return_values = l.attributes.iter().map(|e| return_value(&e.0, name));

            quote! {
                log::trace!("Start reading {}.", stringify!(#name));

                #( #init_values )*

                #set_attrs

                log::trace!("Finished reading {}.", stringify!(#name));

                Ok(#name { #( #return_values, )* })
            }
        }
        Element::Text(t) => {
            let name = &t.name;
            let tag = &t.tag;

            let init_values = t
                .attributes
                .iter()
                .map(|e| init_value(&e.0))
                .chain(once(init_value(&t.text_field)));

            let set_attrs = set_attrs(name, tag, &t.attributes);
            let set_text = {
                let name = &t.text_field.name;
                quote! { #name = Some(r.read_text(#tag, &mut Vec::new())?); }
            };

            let return_values = t
                .attributes
                .iter()
                .map(|e| return_value(&e.0, name))
                .chain(once(return_value(&t.text_field, name)));

            quote! {
                log::trace!("Start reading {}.", stringify!(#name));

                #( #init_values )*

                #set_attrs
                #set_text

                log::trace!("Finished reading {}.", stringify!(#name));

                Ok(#name { #( #return_values, )* })
            }
        }
        Element::Parent(p) => {
            let name = &p.name;
            let tag = &p.tag;

            let mut init_values = p
                .attributes
                .iter()
                .chain(p.children.iter())
                .chain(p.leaf_children.iter())
                .chain(p.flatten_text.iter())
                .map(|(field, _)| field)
                .collect::<Vec<_>>();

            init_values.dedup_by_key(|field| &field.name);

            let init_values = init_values.iter().map(|field| init_value(&field));

            let set_attrs = set_attrs(name, tag, &p.attributes);

            let match_leaf_children = p.leaf_children.iter().map(|(field, tag)| {
                let name = &field.name;
                let ty = &field.ty;
                if let Some(ty) = ty.is_vec() {
                    let ident = ty.get_ident();
                    quote! { #tag => #name.push(#ident::read(r, Some(bs))?) }
                } else if let Some(ty) = ty.is_option() {
                    let ident = ty.get_ident();
                    quote! { #tag => #name = Some(#ident::read(r, Some(bs))?) }
                } else {
                    let ident = ty.get_ident();
                    quote! { #tag => #name = Some(#ident::read(r, Some(bs))?) }
                }
            });

            let match_cildren = p
                .children
                .iter()
                .map(|(field, tag)| {
                    let name = &field.name;
                    let ty = &field.ty;
                    if let Some(ty) = ty.is_vec() {
                        let ident = ty.get_ident();
                        quote! { #tag => #name.push(#ident::read(r, Some(bs))?) }
                    } else if let Some(ty) = ty.is_option() {
                        let ident = ty.get_ident();
                        quote! { #tag => #name = Some(#ident::read(r, Some(bs))?) }
                    } else {
                        let ident = ty.get_ident();
                        quote! { #tag => #name = Some(#ident::read(r, Some(bs))?) }
                    }
                })
                .chain(p.flatten_text.iter().map(|(field, tag)| {
                    let name = &field.name;
                    quote! { #tag => #name = Some(r.read_text(#tag, &mut Vec::new())?) }
                }));

            let mut return_values = p
                .attributes
                .iter()
                .chain(p.children.iter())
                .chain(p.leaf_children.iter())
                .chain(p.flatten_text.iter())
                .map(|(field, _)| field)
                .collect::<Vec<_>>();

            return_values.dedup_by_key(|field| &field.name);

            let return_values = return_values
                .iter()
                .map(|field| return_value(&field, &name));

            quote! {
                log::trace!("Start reading {}.", stringify!(#name));

                #( #init_values )*

                #set_attrs

                let mut buf = Vec::new();
                loop {
                    match r.read_event(&mut buf)? {
                        Event::Empty(bs) => match bs.name() {
                            #( #match_leaf_children, )*
                            tag => log::info!(
                                "Unhandled empy tag: {:?} when parsing {}.",
                                String::from_utf8_lossy(tag),
                                stringify!(#name)
                            ),
                        }
                        Event::Start(bs) => match bs.name() {
                            #( #match_cildren, )*
                            tag => {
                                log::info!(
                                    "Unhandled start tag: {:?} when parsing {}. Skipping...",
                                    String::from_utf8_lossy(tag),
                                    stringify!(#name)
                                );
                                r.read_to_end(tag, &mut Vec::new())?;
                            }
                        }
                        Event::End(bs) => {
                            if bs.name() == #tag {
                                break;
                            } else {
                                log::error!(
                                    "Unexpected end tag: {:?} when parsing {}.",
                                    String::from_utf8_lossy(bs.name()),
                                    stringify!(#name)
                                );
                            }
                        }
                        Event::Eof => {
                            log::error!("Unexpected eof when parsing {}.", stringify!(#name));
                            return Err(Error::UnexpectedEof);
                        }
                        _ => (),
                    }
                    buf.clear();
                }

                log::trace!("Finished reading {}.", stringify!(#name));

                Ok(#name { #( #return_values, )* })
            }
        }
    }
}

fn init_value(field: &Field) -> TokenStream {
    let name = &field.name;
    let value = field.ty.init_value();
    quote! { let mut #name = #value; }
}

fn return_value(field: &Field, struct_name: &Ident) -> TokenStream {
    let name = &field.name;
    let ty = &field.ty;
    if ty.is_option().is_some() || ty.is_vec().is_some() || ty.is_bool() {
        quote! { #name }
    } else {
        quote! {
            #name: #name.ok_or(Error::MissingField {
                name: stringify!(#struct_name),
                field: stringify!(#name),
            })?
        }
    }
}

fn set_attrs(name: &Ident, tag: &LitByteStr, attributes: &Vec<(Field, LitByteStr)>) -> TokenStream {
    if attributes.is_empty() {
        return quote! {
            // Skip the start element
            if bs.is_none() {
                let mut buf = Vec::new();
                loop {
                    match r.read_event(&mut buf)? {
                        Event::Start(bs) | Event::Empty(bs) => {
                            if bs.name() == #tag {
                                break;
                            } else {
                                return Err(Error::UnexpectedTag {
                                    expected: stringify!(#tag),
                                    found: String::from_utf8(bs.name().to_vec())?,
                                });
                            }
                        }
                        Event::Eof => {
                            log::error!("Unexpected eof when parsing {}.", stringify!(#name));
                            return Err(Error::UnexpectedEof);
                        }
                        _ => (),
                    }
                    buf.clear();
                }
            }
        };
    }

    let match_attrs = attributes.iter().map(|(field, tag)| {
        let name = &field.name;
        let value = &field.ty.parse_attr_value();

        if field.ty.is_bool() {
            quote! ( #tag => #name = #value )
        } else {
            quote! ( #tag => #name = Some(#value) )
        }
    });

    let loop_attrs = quote! {
        for attr in bs.attributes().filter_map(|a| a.ok()) {
            match attr.key {
                #( #match_attrs, )*
                _ => (),
            }
        }
    };

    quote! {
        if let Some(bs) = bs {
            #loop_attrs
        } else {
            let mut buf = Vec::new();
            loop {
                match r.read_event(&mut buf)? {
                    Event::Start(bs) | Event::Empty(bs) => {
                        if bs.name() == #tag {
                            #loop_attrs
                            break;
                        } else {
                            return Err(Error::UnexpectedTag {
                                expected: stringify!(#tag),
                                found: String::from_utf8(bs.name().to_vec())?,
                            });
                        }
                    }
                    Event::Eof => {
                        log::error!("Unexpected eof when parsing {}.", stringify!(#name));
                        return Err(Error::UnexpectedEof);
                    }
                    _ => (),
                }
                buf.clear();
            }
        }
    }
}

fn read_enum(e: &EnumElement) -> TokenStream {
    let name = &e.name;
    let match_bs = e.elements.iter().map(|(var, tag)| {
        let ty = &var.ty;
        let var_name = &var.name;
        quote!(
            #tag => #ty::read(r, Some(bs)).map(#name::#var_name)
        )
    });
    let tags = e.elements.iter().map(|(_, tag)| tag);

    let match_name = quote! {
        return match bs.name() {
            #( #match_bs, )*
            _ => Err(Error::UnexpectedTag {
                expected: stringify!( #( #tags ),* ),
                found: String::from_utf8(bs.name().to_vec())?,
            }),
        }
    };

    quote! {
        if let Some(bs) = bs {
            #match_name
        } else {
            let mut buf = Vec::new();
            loop {
                match r.read_event(&mut buf)? {
                    Event::Start(bs) | Event::Empty(bs) => #match_name,
                    Event::Eof => {
                        log::error!("Unexpected eof when parsing {}.", stringify!(#name));
                        return Err(Error::UnexpectedEof);
                    },
                    _ => (),
                }
                buf.clear();
            }
        }
    }
}
