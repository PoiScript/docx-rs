use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitStr};

use crate::types::{
    trim_lifetime, Element, EnumElement, Field, LeafElement, ParentElement, TextElement, Type,
};

pub fn read(element: &Element) -> TokenStream {
    match element {
        Element::Enum(enum_ele) => read_enum_element(enum_ele),
        Element::Leaf(leaf_ele) => read_leaf_element(leaf_ele),
        Element::Text(text_ele) => read_text_element(text_ele),
        Element::Parent(parent_ele) => read_parent_element(parent_ele),
    }
}

fn read_leaf_element(leaf_ele: &LeafElement) -> TokenStream {
    let ele_name = &leaf_ele.name;
    let tag = &leaf_ele.tag;
    let attrs = &leaf_ele.attributes;

    let init_attrs = attrs.iter().map(|e| init_value(&e.1));
    let read_attrs = attrs.iter().map(|e| read_attrs(&e.0, &e.1));
    let return_attrs = attrs.iter().map(|e| return_value(&e.1, ele_name));

    let read_to_end = read_to_end();

    quote! {
        log::debug!("Started reading LeafElement {}.", stringify!(#ele_name));

        #( #init_attrs )*

        while let Some(__token) = reader.next() {
            let __token = __token?;
            match __token {
                Token::ElementStart { span: __span, .. } => {
                    let __tag = &__span.as_str()[1..];
                    if __tag == #tag {
                        break;
                    } else {
                        log::info!(
                            "Unhandled tag: {:?} when parsing {}. Skipping.",
                            __tag, stringify!(#ele_name)
                        );
                        #read_to_end
                    }
                }
                Token::ElementEnd { .. } | Token::Attribute { .. } | Token::Text { .. } | Token::Cdata { .. } => {
                    return Err(Error::UnexpectedToken{ token: format!("{:?}", __token) });
                },
                _ => (),
            }
        }

        while let Some(__token) = reader.next() {
            match __token? {
                Token::Attribute { span: __span, value: __value, .. } => {
                    let __value = __value.as_str();
                    let __span = &__span.as_str();
                    let __key = &__span[0..__span.len() - __value.len() - 3];
                    match __key {
                        #( #read_attrs, )*
                        _ => log::info!(
                            "Unhandled attribute: {:?} when reading LeafElement {}. Skipping.",
                            __key, stringify!(#ele_name)
                        ),
                    }
                }
                Token::ElementEnd { end: ElementEnd::Empty, .. } => {
                    let __res = #ele_name {
                        #( #return_attrs, )*
                    };

                    log::debug!("Finished reading LeafElement {}.", stringify!(#ele_name));

                    return Ok(__res);
                },
                __token => {
                    return Err(Error::UnexpectedToken{ token: format!("{:?}", __token) });
                }
            }
        }

        Err(Error::UnexpectedEof)
    }
}

fn read_text_element(text_ele: &TextElement) -> TokenStream {
    let ele_name = &text_ele.name;
    let tag = &text_ele.tag;
    let text = &text_ele.text.name;
    let attrs = &text_ele.attributes;

    let init_attrs = attrs.iter().map(|e| init_value(&e.1));
    let read_attrs = attrs.iter().map(|e| read_attrs(&e.0, &e.1));
    let return_attrs = attrs.iter().map(|e| return_value(&e.1, ele_name));

    let read_to_end = read_to_end();

    quote! {
        log::debug!("Started reading TextElement {}.", stringify!(#ele_name));

        #( #init_attrs )*

        while let Some(__token) = reader.next() {
            let __token = __token?;
            match __token {
                Token::ElementStart { span: __span, .. } => {
                    let __tag = &__span.as_str()[1..];
                    if __tag == #tag {
                        break;
                    } else {
                        log::info!(
                            "Unhandled tag: {:?} when reading TextElement {}. Skipping.",
                            __tag, stringify!(#ele_name)
                        );
                        #read_to_end
                    }
                }
                Token::ElementEnd { .. } | Token::Attribute { .. } | Token::Text { .. } | Token::Cdata { .. } => {
                    return Err(Error::UnexpectedToken{ token: format!("{:?}", __token) });
                },
                _ => (),
            }
        }

        while let Some(token) = reader.next() {
            match token? {
                Token::Attribute { span: __span, value: __value, .. } => {
                    let __value = __value.as_str();
                    let __span = __span.as_str();
                    let __key = &__span[0..__span.len() - __value.len() - 3];
                    match __key {
                        #( #read_attrs, )*
                        _ => log::info!(
                            "Unhandled attribute: {:?} when reading TextElement {}. Skipping.",
                            __key, stringify!(#ele_name)
                        ),
                    }
                }
                Token::ElementEnd { end: ElementEnd::Open, .. } => {
                    let mut __text = None;
                    while let Some(__token) = reader.next() {
                        match __token? {
                            Token::ElementEnd { end: ElementEnd::Open, .. } | Token::Attribute { .. } => (),
                            Token::Text { text } => {
                                __text = Some(crate::xml::unescape(text.as_str())?);
                            }
                            Token::ElementEnd { end: ElementEnd::Close(_, _), span: __span } => {
                                let __span = __span.as_str();
                                let __tag = &__span[2..__span.len() - 1];
                                if __tag == #tag {
                                    let __res = #ele_name {
                                        #text: __text.unwrap_or(Cow::Borrowed("")),
                                        #( #return_attrs, )*
                                    };

                                    log::debug!("Finished reading TextElement {}.", stringify!(#ele_name));

                                    return Ok(__res);
                                } else {
                                    return Err(Error::TagMismatch {
                                        expected: #tag.to_owned(),
                                        found: __tag.to_owned(),
                                    });
                                }
                            }
                            __token => {
                                return Err(Error::UnexpectedToken{ token: format!("{:?}", __token) });
                            }
                        }
                    }
                    return Err(Error::UnexpectedEof);
                },
                __token => {
                    return Err(Error::UnexpectedToken{ token: format!("{:?}", __token) });
                }
            }
        }
        Err(Error::UnexpectedEof)
    }
}

fn read_parent_element(parent_ele: &ParentElement) -> TokenStream {
    let ele_name = &parent_ele.name;
    let tag = &parent_ele.tag;
    let attrs = &parent_ele.attributes;
    let children = &parent_ele.children;
    let flatten_text = &parent_ele.flatten_text;

    let mut children_fields = children.iter().map(|e| &e.1).collect::<Vec<_>>();
    children_fields.dedup_by_key(|f| &f.name);

    let init_attrs = attrs.iter().map(|e| init_value(&e.1));
    let read_attrs = attrs.iter().map(|e| read_attrs(&e.0, &e.1));
    let return_attrs = attrs.iter().map(|e| return_value(&e.1, ele_name));

    let init_children = children_fields.iter().map(|f| init_value(f));
    let read_children = children.iter().map(|e| read_children(&e.0, &e.1));
    let return_children = children_fields.iter().map(|f| return_value(f, ele_name));

    let init_flatten_text = flatten_text.iter().map(|e| init_value(&e.1));
    let read_flatten_text = flatten_text.iter().map(|e| read_flatten_text(&e.0, &e.1));
    let return_flatten_text = flatten_text.iter().map(|e| return_value(&e.1, ele_name));

    let read_to_end = read_to_end();

    quote! {
        log::debug!("Started reading ParentElement {}.", stringify!(#ele_name));

        #( #init_attrs )*
        #( #init_children )*
        #( #init_flatten_text )*

        while let Some(__token) = reader.next() {
            let __token = __token?;
            match __token {
                Token::ElementStart { span: __span, .. } => {
                    let __tag = &__span.as_str()[1..];
                    if __tag == #tag {
                        break;
                    } else {
                        log::info!(
                            "Unhandled tag: {:?} when reading ParentElement {}. Skipping.",
                            __tag, stringify!(#ele_name)
                        );
                        #read_to_end
                    }
                }
                Token::ElementEnd { .. } | Token::Attribute { .. } | Token::Text { .. } | Token::Cdata { .. } => {
                    return Err(Error::UnexpectedToken{ token: format!("{:?}", __token) });
                },
                _ => (),
            }
        }

        while let Some(__token) = reader.next() {
            match __token? {
                Token::Attribute { span: __span, value: __value, .. } => {
                    let __value = __value.as_str();
                    let __span = __span.as_str();
                    let __key = &__span[0..__span.len() - __value.len() - 3];
                    match __key {
                        #( #read_attrs, )*
                        _ => log::info!(
                            "Unhandled attribute: {:?} when parsing ParentElement {}. Skipping.",
                            __key, stringify!(#ele_name)
                        ),
                    }
                }
                Token::ElementEnd { end: ElementEnd::Open, .. } => {
                    while let Some(__token) = reader.peek() {
                        match __token {
                            Ok(Token::ElementStart { span: __span, .. }) => {
                                let __tag = &__span.as_str()[1..];
                                match __tag {
                                    #( #read_children, )*
                                    #( #read_flatten_text, )*
                                    _ => {
                                        log::info!(
                                            "Unhandled tag: {:?} when parsing ParentElement {}. Skipping.",
                                            __tag, stringify!(#ele_name)
                                        );
                                        #read_to_end
                                    },
                                }
                            }
                            Ok(Token::ElementEnd { end: ElementEnd::Close(_, _), span: __span }) => {
                                let __span = __span.as_str();
                                let __tag = &__span[2..__span.len() - 1];
                                if __tag == #tag {
                                    let __res = #ele_name {
                                        #( #return_attrs, )*
                                        #( #return_children, )*
                                        #( #return_flatten_text, )*
                                    };

                                    log::debug!("Finished reading ParentElmenet {}.", stringify!(#ele_name));

                                    reader.next();

                                    return Ok(__res);
                                } else {
                                    return Err(Error::TagMismatch {
                                        expected: #tag.to_owned(),
                                        found: __tag.to_owned(),
                                    });
                                }
                            }
                            Ok(Token::ElementEnd { .. }) | Ok(Token::Attribute { .. }) | Ok(Token::Text { .. }) | Ok(Token::Cdata { .. }) => {
                                return Err(Error::UnexpectedToken{ token: format!("{:?}", __token) });
                            }
                            _ => (),
                        }
                    }
                    return Err(Error::UnexpectedEof);
                }
                __token => {
                    return Err(Error::UnexpectedToken{ token: format!("{:?}", __token) });
                }
            }
        }
        Err(Error::UnexpectedEof)
    }
}

fn init_value(field: &Field) -> TokenStream {
    let name = &field.name;

    match field.ty {
        Type::VecT(_) | Type::VecCowStr => quote! { let mut #name = vec![]; },
        Type::OptionCowStr
        | Type::OptionT(_)
        | Type::OptionBool
        | Type::OptionUsize
        | Type::CowStr
        | Type::T(_)
        | Type::Bool
        | Type::Usize => quote! { let mut #name = None; },
    }
}

fn return_value(field: &Field, ele_name: &Ident) -> TokenStream {
    let name = &field.name;

    match field.ty {
        Type::OptionCowStr
        | Type::OptionT(_)
        | Type::OptionBool
        | Type::OptionUsize
        | Type::VecCowStr
        | Type::VecT(_) => quote! { #name },
        Type::CowStr | Type::T(_) | Type::Usize | Type::Bool => quote! {
            #name: #name.ok_or(Error::MissingField {
                name: stringify!(#ele_name).to_owned(),
                field: stringify!(#name).to_owned(),
            })?
        },
    }
}

fn read_attrs(tag: &LitStr, field: &Field) -> TokenStream {
    let name = &field.name;

    match &field.ty {
        Type::CowStr | Type::OptionCowStr => quote! {
            #tag => #name = Some(Cow::Borrowed(__value))
        },
        Type::Bool | Type::OptionBool => quote! {
            #tag => {
                use std::str::FromStr;
                #name = Some(bool::from_str(__value).or(usize::from_str(__value).map(|v| v != 0))?);
            }
        },
        Type::Usize | Type::OptionUsize => quote! {
            #tag => {
                use std::str::FromStr;
                #name = Some(usize::from_str(__value)?);
            }
        },
        Type::T(ty) | Type::OptionT(ty) => quote! {
            #tag => {
                use std::str::FromStr;
                #name = Some(#ty::from_str(__value)?);
            }
        },
        _ => panic!("#[xml(attr =\"\")] only supports Cow<str>, Option<Cow<str>>, bool, Option<bool>, usize, Option<usize> and Option<T>.")
    }
}

fn read_children(tag: &LitStr, field: &Field) -> TokenStream {
    let name = &field.name;

    match &field.ty {
        Type::VecT(ty) => {
            if let Some(ident) = trim_lifetime(ty) {
                quote! {
                    #tag => #name.push(#ident::from_reader(reader)?)
                }
            } else {
                quote! {
                    #tag => #name.push(#ty::from_reader(reader)?)
                }
            }
        }
        Type::OptionT(ty) => {
            if let Some(ident) = trim_lifetime(ty) {
                quote! {
                    #tag => #name = Some(#ident::from_reader(reader)?)
                }
            } else {
                quote! {
                    #tag => #name = Some(#ty::from_reader(reader)?)
                }
            }
        }
        Type::T(ty) => {
            if let Some(ident) = trim_lifetime(ty) {
                quote! {
                    #tag => #name = Some(#ident::from_reader(reader)?)
                }
            } else {
                quote! {
                    #tag => #name = Some(#ty::from_reader(reader)?)
                }
            }
        }
        _ => panic!("#[xml(child = \"\")] only support Vec<T>, Option<T> and T."),
    }
}

fn read_flatten_text(tag: &LitStr, field: &Field) -> TokenStream {
    let name = &field.name;

    let set_text = match field.ty {
        Type::VecCowStr => quote! {
            #name.push(crate::xml::unescape(__text.as_str())?),
        },
        Type::CowStr | Type::OptionCowStr => quote! {
            #name = Some(crate::xml::unescape(__text.as_str())?),
        },
        _ => panic!(
            "#[xml(flatten_text)] only support Cow<str>, Vec<Cow<str>> and Option<Cow<str>>."
        ),
    };

    quote! {
        #tag => {
            log::debug!("Started reading flatten_text {}.", stringify!(#name));

            while let Some(__token) = reader.next() {
                match __token? {
                    Token::ElementStart { span: __span, .. } => {
                        let __tag = &__span.as_str()[1..];
                        if __tag != #tag {
                            return Err(Error::TagMismatch {
                                expected: #tag.to_owned(),
                                found: __tag.to_owned(),
                            });
                        }
                    }
                    Token::Text { text: __text } =>  #set_text
                    Token::ElementEnd { end: ElementEnd::Close(_, _), span: __span } => {
                        let __span = __span.as_str();
                        let __tag = &__span[2..__span.len() - 1];
                        if __tag == #tag {
                            break;
                        } else {
                            return Err(Error::TagMismatch {
                                expected: #tag.to_owned(),
                                found: __tag.to_owned(),
                            });
                        }
                    }
                    _ => (),
                }
            }

            log::debug!("Finished reading flatten_text {}.", stringify!(#name));
        }
    }
}

fn read_to_end() -> TokenStream {
    quote! {
        let __end_tag = __tag;
        let mut __depth = 1;
        while let Some(token) = reader.next() {
            match token? {
                Token::ElementStart { span: __span, .. } => {
                    let __tag = &__span.as_str()[1..];
                    if __tag == __end_tag {
                        __depth += 1;
                    }
                }
                Token::ElementEnd { end: ElementEnd::Empty, .. } => {
                    break;
                }
                Token::ElementEnd { end: ElementEnd::Close(_, _), span: __span } => {
                    let __span = __span.as_str();
                    let __tag = &__span[2..__span.len() - 1];
                    if __tag == __end_tag {
                        if __depth == 1 {
                            break;
                        } else {
                            __depth -= 1;
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

fn read_enum_element(enum_ele: &EnumElement) -> TokenStream {
    let ele_name = &enum_ele.name;
    let read_variants = enum_ele.elements.iter().map(|(tag, var)| {
        let var_name = &var.name;
        let ty = &var.ty;

        if let Some(ident) = trim_lifetime(ty) {
            quote! {
                #tag => return #ident::from_reader(reader).map(#ele_name::#var_name)
            }
        } else {
            quote! {
                #tag => return #ty::from_reader(reader).map(#ele_name::#var_name)
            }
        }
    });

    let read_to_end = read_to_end();

    quote! {
        while let Some(__token) = reader.peek() {
            match __token {
                Ok(Token::ElementStart { span: __span, .. }) => {
                    let __tag = &__span.as_str()[1..];
                    match __tag {
                        #( #read_variants, )*
                        _ => {
                            log::info!(
                                "Unhandled tag: {:?} when parsing {}. Skipping.",
                                __tag, stringify!(#ele_name)
                            );
                            #read_to_end
                        }
                    }
                },
                Ok(Token::ElementEnd { .. }) | Ok(Token::Attribute { .. }) | Ok(Token::Text { .. }) | Ok(Token::Cdata { .. }) => {
                    return Err(Error::UnexpectedToken{ token: format!("{:?}", __token) });
                },
                _ => (),
            }
        }
        Err(Error::UnexpectedEof)
    }
}
