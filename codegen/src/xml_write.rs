use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::types::{Element, EnumElement, Field, LeafElement, ParentElement, TextElement, Type};

pub fn write(element: &Element) -> TokenStream {
    match element {
        Element::Leaf(leaf_ele) => write_leaf_ele(leaf_ele),
        Element::Text(text_ele) => write_text_ele(text_ele),
        Element::Parent(parent_ele) => write_parent_ele(parent_ele),
        Element::Enum(enum_ele) => write_enum_ele(enum_ele),
    }
}

fn write_leaf_ele(leaf_ele: &LeafElement) -> TokenStream {
    let ele_name = &leaf_ele.name;
    let tag = &leaf_ele.tag;
    let attrs = &leaf_ele.attributes;

    let extend_attrs = if let Some(extend_attrs) = &leaf_ele.extend_attrs {
        quote! { #extend_attrs(&self, &mut writer)?; }
    } else {
        quote!()
    };

    let write_attrs = attrs.iter().map(|(fld, tag)| write_attrs(fld, tag));

    quote! {
        log::debug!("Started writing LeafElement {}.", stringify!(#ele_name));

        write!(&mut writer, concat!("<", #tag))?;

        #( #write_attrs )*

        #extend_attrs

        write!(&mut writer, "/>")?;

        log::debug!("Finished writing LeafElement {}.", stringify!(#ele_name));
    }
}

fn write_text_ele(text_ele: &TextElement) -> TokenStream {
    let ele_name = &text_ele.name;
    let tag = &text_ele.tag;
    let attrs = &text_ele.attributes;
    let text = &text_ele.text.name;

    let extend_attrs = if let Some(extend_attrs) = &text_ele.extend_attrs {
        quote! { #extend_attrs(&self, &mut writer)?; }
    } else {
        quote!()
    };

    let write_attrs = attrs.iter().map(|(fld, tag)| write_attrs(fld, tag));

    quote! {
        log::debug!("Started writing TextElement {}.", stringify!(#ele_name));

        write!(&mut writer, concat!("<", #tag))?;

        #( #write_attrs )*

        #extend_attrs

        write!(&mut writer, ">")?;

        write!(&mut writer, "{}", crate::xml::escape(&self.#text))?;

        write!(&mut writer, concat!("</", #tag, ">"))?;

        log::debug!("Finished writing TextElement {}.", stringify!(#ele_name));
    }
}

fn write_parent_ele(parent_ele: &ParentElement) -> TokenStream {
    let ele_name = &parent_ele.name;
    let tag = &parent_ele.tag;
    let attrs = &parent_ele.attributes;
    let flatten_text = &parent_ele.flatten_text;
    let children = &parent_ele.children;

    let mut children = children.iter().map(|e| &e.1).collect::<Vec<_>>();
    children.dedup_by_key(|f| &f.name);

    let extend_attrs = if let Some(extend_attrs) = &parent_ele.extend_attrs {
        quote! { #extend_attrs(&self, &mut writer)?; }
    } else {
        quote!()
    };

    let write_attrs = attrs.iter().map(|(fld, tag)| write_attrs(fld, tag));
    let write_children = children.iter().map(|fld| write_child(fld));
    let write_flatten_text = flatten_text
        .iter()
        .map(|(tag, fld)| write_flatten_text(tag, fld));

    quote! {
        log::debug!("Started writing ParentElement {}.", stringify!(#ele_name));

        write!(&mut writer, concat!("<", #tag))?;

        #( #write_attrs )*

        #extend_attrs

        write!(&mut writer, ">")?;

        #( #write_children )*
        #( #write_flatten_text )*

        write!(&mut writer, concat!("</", #tag, ">"))?;

        log::debug!("Finished writing ParentElement {}.", stringify!(#ele_name));
    }
}

fn write_attrs(tag: &LitStr, field: &Field) -> TokenStream {
    let name = &field.name;

    match field.ty {
        Type::OptionCowStr | Type::OptionBool | Type::OptionUsize => quote! {
            if let Some(ref value) = self.#name {
                write!(&mut writer, concat!(" ", #tag, "=", "\"{}\""), value)?;
            }
        },
        Type::CowStr | Type::Bool | Type::Usize => quote! {
            write!(&mut writer, concat!(" ", #tag, "=", "\"{}\""), self.#name)?;
        },
        Type::OptionT(_) => quote! {
            if let Some(ref value) = self.#name {
                write!(&mut writer, concat!(" ", #tag, "=", "\"{}\""), value)?;
            }
        },
        Type::T(_) => quote! {
            write!(&mut writer, concat!(" ", #tag, "=", "\"{}\""), self.#name)?;
        },
        _ => panic!("#[xml(attr = \"\")] only supports Cow<str>, Option<Cow<str>>, bool, Option<bool>, usize, Option<usize> and Option<T>."),
    }
}

fn write_child(field: &Field) -> TokenStream {
    let name = &field.name;

    match &field.ty {
        Type::OptionT(_) => {
            quote! {
                if let Some(ref ele) = self.#name {
                    ele.write(&mut writer)?;
                }
            }
        }
        Type::VecT(_) => quote! {
            for ele in &self.#name {
                ele.write(&mut writer)?;
            }
        },
        Type::T(_) => quote! {
            &self.#name.write(&mut writer)?;
        },
        _ => panic!("#[xml(child = \"\")] only support Vec<T>, Option<T> and T."),
    }
}

fn write_flatten_text(tag: &LitStr, field: &Field) -> TokenStream {
    let name = &field.name;

    match &field.ty {
        Type::CowStr => quote! {
            write!(&mut writer, concat!("<" , #tag, ">"))?;

            write!(&mut writer, "{}", crate::xml::escape(&self.#name))?;

            write!(&mut writer, concat!("</" , #tag, ">"))?;
        },
        Type::OptionCowStr => quote! {
            if let Some(value) = &self.#name {
                write!(&mut writer, concat!("<" , #tag, ">"))?;

                write!(&mut writer, "{}", crate::xml::escape(&value))?;

                write!(&mut writer, concat!("</" , #tag, ">"))?;
            }
        },
        Type::VecCowStr => quote! {
           for value in &self.#name {
                write!(&mut writer, concat!("<" , #tag, ">"))?;

                write!(&mut writer, "{}", crate::xml::escape(&value))?;

                write!(&mut writer, concat!("</" , #tag, ">"))?;
            }
        },
        _ => panic!(
            "#[xml(flatten_text)] only support Cow<str>, Vec<Cow<str>> and Option<Cow<str>>."
        ),
    }
}

fn write_enum_ele(enum_ele: &EnumElement) -> TokenStream {
    let name = &enum_ele.name;
    let var_names = enum_ele.elements.iter().map(|(_, var)| &var.name);

    quote! {
        match self {
            #( #name::#var_names(s) => s.write(writer)?, )*
        }
    }
}
