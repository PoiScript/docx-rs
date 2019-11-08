use crate::types::{Element, Field, Type};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_into_owned(element: &Element) -> TokenStream {
    match element {
        Element::Enum(enum_ele) => {
            let name = &enum_ele.name;
            let var_names = enum_ele.elements.iter().map(|(_, var)| &var.name);

            quote! {
                match self {
                    #( #name::#var_names(s) => #name::#var_names(s.into_owned()), )*
                }
            }
        }
        Element::Leaf(leaf_ele) => {
            let name = &leaf_ele.name;
            let attrs = &leaf_ele.attributes;

            let attrs_into_owned = attrs.iter().map(|e| into_owned(&e.1, true));

            quote! {
                #name {
                    #( #attrs_into_owned, )*
                }
            }
        }
        Element::Text(text_ele) => {
            let name = &text_ele.name;
            let attrs = &text_ele.attributes;

            let attrs_into_owned = attrs.iter().map(|e| into_owned(&e.1, true));
            let text_into_owned = into_owned(&text_ele.text, false);

            quote! {
                #name {
                    #( #attrs_into_owned, )*
                    #text_into_owned
                }
            }
        }
        Element::Parent(parent_ele) => {
            let name = &parent_ele.name;
            let attrs = &parent_ele.attributes;
            let children = &parent_ele.children;
            let flatten_text = &parent_ele.flatten_text;

            let mut children = children.iter().map(|e| &e.1).collect::<Vec<_>>();

            children.dedup_by_key(|f| &f.name);

            let attrs_into_owned = attrs.iter().map(|e| into_owned(&e.1, true));
            let children_into_owned = children.iter().map(|e| into_owned(e, false));
            let flatten_text_into_owned = flatten_text.iter().map(|e| into_owned(&e.1, false));

            quote! {
                #name {
                    #( #attrs_into_owned, )*
                    #( #children_into_owned, )*
                    #( #flatten_text_into_owned, )*
                }
            }
        }
    }
}

fn into_owned(field: &Field, is_attr: bool) -> TokenStream {
    let name = &field.name;

    match field.ty {
        Type::Bool | Type::Usize | Type::OptionBool | Type::OptionUsize => quote! {
            #name: self.#name
        },
        Type::OptionT(_) | Type::T(_) if is_attr => quote! {
            #name: self.#name
        },
        Type::OptionCowStr => quote! {
            #name: match self.#name {
                Some(Cow::Borrowed(borrowed)) => Some(Cow::Owned(borrowed.into())),
                Some(Cow::Owned(owned)) => Some(Cow::Owned(owned)),
                None => None,
            }
        },
        Type::T(_) => quote! {
            #name: self.#name.into_owned()
        },
        Type::OptionT(_) => quote! {
            #name: self.#name.map(|x| x.into_owned())
        },
        Type::VecCowStr => quote! {
            #name: self.#name.into_iter().map(|x| match x {
                Cow::Borrowed(borrowed) => Cow::Owned(borrowed.into()),
                Cow::Owned(owned) => Cow::Owned(owned),
            }).collect()
        },
        Type::VecT(_) => quote! {
            #name: self.#name.into_iter().map(|s| s.into_owned()).collect()
        },
        Type::CowStr => quote! {
            #name: match self.#name {
                Cow::Borrowed(borrowed) => Cow::Owned(borrowed.into()),
                Cow::Owned(owned) => Cow::Owned(owned),
            }
        },
    }
}
