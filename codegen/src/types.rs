use proc_macro2::Span;

use syn::{Lit::*, Meta::*, *};

#[allow(clippy::large_enum_variant)]
pub enum Element {
    Enum(EnumElement),
    Leaf(LeafElement),
    Text(TextElement),
    Parent(ParentElement),
}

pub struct Variant {
    pub name: Ident,
    pub ty: syn::Type,
}

pub struct Field {
    pub name: Ident,
    pub ty: Type,
}

pub struct EnumElement {
    pub name: Ident,
    pub elements: Vec<(LitStr, Variant)>,
}

pub struct LeafElement {
    pub name: Ident,
    pub tag: LitStr,
    pub extend_attrs: Option<Ident>,
    pub attributes: Vec<(LitStr, Field)>,
}

pub struct TextElement {
    pub name: Ident,
    pub tag: LitStr,
    pub extend_attrs: Option<Ident>,
    pub attributes: Vec<(LitStr, Field)>,
    pub text: Field,
}

pub struct ParentElement {
    pub name: Ident,
    pub tag: LitStr,
    pub extend_attrs: Option<Ident>,
    pub attributes: Vec<(LitStr, Field)>,
    pub children: Vec<(LitStr, Field)>,
    pub flatten_text: Vec<(LitStr, Field)>,
}

impl Element {
    pub fn parse(input: &DeriveInput) -> Element {
        match input.data {
            Data::Struct(ref data) => Self::parse_struct(data, &input.attrs, &input.ident),
            Data::Enum(ref data) => Self::parse_enum(data, &input.ident),
            Data::Union(_) => panic!("#[derive(Xml)] doesn't support Union."),
        }
    }

    pub fn parse_struct(data: &DataStruct, attrs: &[Attribute], ident: &Ident) -> Element {
        let mut leaf = false;
        let mut tag = None;
        let mut extend_attrs = None;

        for meta in attrs.iter().filter_map(get_xml_meta).flatten() {
            match meta {
                NestedMeta::Meta(Path(ref p)) if p.is_ident("leaf") => {
                    leaf = true;
                }
                NestedMeta::Meta(NameValue(ref m)) if m.path.is_ident("tag") => {
                    if let Str(ref lit) = m.lit {
                        tag = Some(lit.clone());
                    }
                }
                NestedMeta::Meta(NameValue(ref m)) if m.path.is_ident("extend_attrs") => {
                    if let Str(ref lit) = m.lit {
                        extend_attrs = Some(Ident::new(&lit.value(), Span::call_site()));
                    }
                }
                item => panic!("Unsupported attrs: {:?}.", item),
            }
        }

        let mut attributes = Vec::new();
        let mut text = None;
        let mut children = Vec::new();
        // let mut leaf_children = Vec::new();
        let mut flatten_text = Vec::new();

        for field in data.fields.iter() {
            let name = &field.ident;
            let ty = &field.ty;

            for meta in field.attrs.iter().filter_map(get_xml_meta).flatten() {
                let field = Field {
                    name: name.clone().unwrap(),
                    ty: ty.into(),
                };

                match meta {
                    NestedMeta::Meta(NameValue(ref m)) if m.path.is_ident("attr") => {
                        if let Str(ref lit) = m.lit {
                            attributes.push((lit.clone(), field));
                        }
                    }
                    NestedMeta::Meta(Path(ref p)) if p.is_ident("text") => {
                        text = Some(field);
                    }
                    NestedMeta::Meta(NameValue(ref m)) if m.path.is_ident("child") => {
                        if let Str(ref lit) = m.lit {
                            children.push((lit.clone(), field));
                        }
                    }
                    // NestedMeta::Meta(NameValue(ref m)) if m.path.is_ident("leaf_child") => {
                    //     if let Str(ref lit) = m.lit {
                    //         leaf_children.push((bytes_str!(lit.clone()), field));
                    //     }
                    // }
                    NestedMeta::Meta(NameValue(ref m)) if m.path.is_ident("flatten_text") => {
                        if let Str(ref lit) = m.lit {
                            flatten_text.push((lit.clone(), field));
                        }
                    }
                    meta => panic!(
                        "Unkown attribute {:?} when parsing field {}.",
                        meta, field.name,
                    ),
                }
            }
        }

        let tag = tag.expect(&format!("Struct doesn't have tag attribute."));

        if leaf {
            if text.is_none() && children.is_empty() {
                Element::Leaf(LeafElement {
                    name: ident.clone(),
                    tag,
                    extend_attrs,
                    attributes,
                })
            } else {
                panic!("Invalid LeafElement: {}", ident.clone());
            }
        } else if let Some(text) = text {
            if children.is_empty() {
                Element::Text(TextElement {
                    name: ident.clone(),
                    tag,
                    extend_attrs,
                    attributes,
                    text,
                })
            } else {
                panic!("Invalid TextElement: {}", ident.clone());
            }
        } else {
            Element::Parent(ParentElement {
                name: ident.clone(),
                tag,
                extend_attrs,
                attributes,
                children,
                flatten_text,
            })
        }
    }

    pub fn parse_enum(data: &DataEnum, ident: &Ident) -> Element {
        let mut elements = Vec::new();

        for variant in &data.variants {
            let name = &variant.ident;
            let ty = &variant.fields.iter().nth(0).unwrap().ty;

            for meta in variant.attrs.iter().filter_map(get_xml_meta).flatten() {
                match meta {
                    NestedMeta::Meta(NameValue(ref m)) if m.path.is_ident("tag") => {
                        if let Str(ref lit) = m.lit {
                            elements.push((
                                lit.clone(),
                                Variant {
                                    name: name.clone(),
                                    ty: ty.clone(),
                                },
                            ));
                        }
                    }
                    _ => panic!("Unkown attribute when parsing variant {}.", &name),
                }
            }
        }

        Element::Enum(EnumElement {
            name: ident.clone(),
            elements,
        })
    }
}

fn get_xml_meta(attr: &Attribute) -> Option<Vec<NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "xml" {
        match attr.parse_meta() {
            Ok(Meta::List(meta)) => Some(meta.nested.iter().cloned().collect()),
            _ => None,
        }
    } else {
        None
    }
}

pub enum Type {
    // Vec<Cow<'a, str>>, flatten_text
    VecCowStr,
    // Vec<T>, children
    VecT(syn::Type),
    // Option<T>, children, attr
    OptionT(syn::Type),
    // Option<Cow<'a, str>>, flatten_text, attr
    OptionCowStr,
    // Option<bool>, attr
    OptionBool,
    // Option<usize>, attr
    OptionUsize,
    // Cow<'a, str>, flatten_text
    CowStr,
    // bool, attr
    Bool,
    // usize, attr
    Usize,
    // T, child, attr
    T(syn::Type),
}

impl From<&syn::Type> for Type {
    fn from(ty: &syn::Type) -> Self {
        if let Some(ty) = is_vec(ty) {
            if is_cow_str(ty) {
                Type::VecCowStr
            } else {
                Type::VecT(ty.clone())
            }
        } else if let Some(ty) = is_option(ty) {
            if is_cow_str(ty) {
                Type::OptionCowStr
            } else if is_bool(ty) {
                Type::OptionBool
            } else if is_usize(ty) {
                Type::OptionUsize
            } else {
                Type::OptionT(ty.clone())
            }
        } else if is_cow_str(ty) {
            Type::CowStr
        } else if is_usize(ty) {
            Type::Usize
        } else if is_bool(ty) {
            Type::Bool
        } else {
            Type::T(ty.clone())
        }
    }
}

fn is_vec(ty: &syn::Type) -> Option<&syn::Type> {
    let path = match ty {
        syn::Type::Path(ty) => &ty.path,
        _ => return None,
    };
    let seg = path.segments.last()?;
    let args = match &seg.arguments {
        PathArguments::AngleBracketed(bracketed) => &bracketed.args,
        _ => return None,
    };
    if seg.ident == "Vec" && args.len() == 1 {
        match args[0] {
            GenericArgument::Type(ref arg) => Some(arg),
            _ => None,
        }
    } else {
        None
    }
}

fn is_option(ty: &syn::Type) -> Option<&syn::Type> {
    let path = match ty {
        syn::Type::Path(ty) => &ty.path,
        _ => return None,
    };
    let seg = path.segments.last()?;
    let args = match &seg.arguments {
        PathArguments::AngleBracketed(bracketed) => &bracketed.args,
        _ => return None,
    };
    if seg.ident == "Option" && args.len() == 1 {
        match args[0] {
            GenericArgument::Type(ref arg) => Some(arg),
            _ => None,
        }
    } else {
        None
    }
}

fn is_cow_str(ty: &syn::Type) -> bool {
    let path = match ty {
        syn::Type::Path(ty) => &ty.path,
        _ => return false,
    };
    let seg = match path.segments.last() {
        Some(seg) => seg,
        None => return false,
    };
    let args = match &seg.arguments {
        PathArguments::AngleBracketed(bracketed) => &bracketed.args,
        _ => return false,
    };
    if seg.ident == "Cow" && args.len() == 2 {
        match &args[1] {
            GenericArgument::Type(syn::Type::Path(ty)) => ty.path.is_ident("str"),
            _ => false,
        }
    } else {
        false
    }
}

fn is_bool(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(ty) => ty.path.is_ident("bool"),
        _ => false,
    }
}

fn is_usize(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(ty) => ty.path.is_ident("usize"),
        _ => false,
    }
}

pub fn trim_lifetime(ty: &syn::Type) -> Option<&Ident> {
    let path = match ty {
        syn::Type::Path(ty) => &ty.path,
        _ => return None,
    };
    let seg = path.segments.last()?;
    Some(&seg.ident)
}
