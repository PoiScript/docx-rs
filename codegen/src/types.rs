use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Lit::*, Meta::*, *};

macro_rules! bytes_str {
    ($t:expr) => {
        LitByteStr::new($t.value().as_bytes(), ::proc_macro2::Span::call_site())
    };
}

#[allow(clippy::large_enum_variant)]
pub enum Element {
    Enum(EnumElement),
    Leaf(LeafElement),
    Text(TextElement),
    Parent(ParentElement),
}

pub struct EnumElement {
    pub name: Ident,
    pub elements: Vec<(Variant, LitByteStr)>,
}

pub struct Variant {
    pub name: Ident,
    pub ty: Type,
}

pub struct LeafElement {
    pub name: Ident,
    pub tag: LitByteStr,
    pub extend_attrs: Option<Ident>,
    pub attributes: Vec<(Field, LitByteStr)>,
}

pub struct TextElement {
    pub name: Ident,
    pub tag: LitByteStr,
    pub extend_attrs: Option<Ident>,
    pub attributes: Vec<(Field, LitByteStr)>,
    pub text_field: Field,
}

pub struct ParentElement {
    pub name: Ident,
    pub tag: LitByteStr,
    pub extend_attrs: Option<Ident>,
    pub attributes: Vec<(Field, LitByteStr)>,
    pub children: Vec<(Field, LitByteStr)>,
    pub leaf_children: Vec<(Field, LitByteStr)>,
    pub flatten_text: Vec<(Field, LitByteStr)>,
}

#[derive(Clone)]
pub struct Field {
    pub name: Ident,
    pub ty: Type,
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

        for meta_items in attrs.iter().filter_map(get_xml_meta_items) {
            for meta_item in meta_items {
                use NestedMeta::Meta;
                match meta_item {
                    Meta(Path(ref p)) if p.is_ident("leaf") => {
                        leaf = true;
                    }
                    Meta(NameValue(ref m)) if m.path.is_ident("tag") => {
                        if let Str(ref lit) = m.lit {
                            tag = Some(lit.clone());
                        }
                    }
                    Meta(NameValue(ref m)) if m.path.is_ident("extend_attrs") => {
                        if let Str(ref lit) = m.lit {
                            extend_attrs = Some(Ident::new(&lit.value(), Span::call_site()));
                        }
                    }
                    item => panic!("Unsupported attrs: {:?}", item),
                }
            }
        }

        let mut attributes = Vec::new();
        let mut text_field = None;
        let mut children = Vec::new();
        let mut leaf_children = Vec::new();
        let mut flatten_text = Vec::new();

        for field in data.fields.iter() {
            let name = &field.ident;
            let ty = &field.ty;

            for meta_item in field.attrs.iter().filter_map(get_xml_meta_items).flatten() {
                use NestedMeta::Meta;

                let field = Field {
                    name: name.clone().unwrap(),
                    ty: ty.clone(),
                };

                match meta_item {
                    Meta(NameValue(ref m)) if m.path.is_ident("attr") => {
                        if let Str(ref lit) = m.lit {
                            attributes.push((field, bytes_str!(lit.clone())));
                        }
                    }
                    Meta(Path(ref p)) if p.is_ident("text") => {
                        text_field = Some(field);
                    }
                    Meta(NameValue(ref m)) if m.path.is_ident("child") => {
                        if let Str(ref lit) = m.lit {
                            children.push((field, bytes_str!(lit.clone())));
                        }
                    }
                    Meta(NameValue(ref m)) if m.path.is_ident("leaf_child") => {
                        if let Str(ref lit) = m.lit {
                            leaf_children.push((field, bytes_str!(lit.clone())));
                        }
                    }
                    Meta(NameValue(ref m)) if m.path.is_ident("flatten_text") => {
                        if let Str(ref lit) = m.lit {
                            flatten_text.push((field, bytes_str!(lit.clone())));
                        }
                    }
                    meta => panic!(
                        "Unkown attribute {:?} when parsing field {}.",
                        meta, field.name,
                    ),
                }
            }
        }

        if leaf {
            if text_field.is_none() && children.is_empty() {
                Element::Leaf(LeafElement {
                    name: ident.clone(),
                    tag: bytes_str!(tag.unwrap()),
                    extend_attrs,
                    attributes,
                })
            } else {
                panic!("Invalid LeafElement: {}", ident.clone());
            }
        } else if text_field.is_some() {
            if children.is_empty() {
                Element::Text(TextElement {
                    name: ident.clone(),
                    tag: bytes_str!(tag.unwrap()),
                    extend_attrs,
                    attributes,
                    text_field: text_field.unwrap(),
                })
            } else {
                panic!("Invalid TextElement: {}", ident.clone());
            }
        } else {
            Element::Parent(ParentElement {
                name: ident.clone(),
                tag: bytes_str!(tag.unwrap()),
                extend_attrs,
                attributes,
                children,
                leaf_children,
                flatten_text,
            })
        }
    }

    pub fn parse_enum(data: &DataEnum, ident: &Ident) -> Element {
        let mut elements = Vec::new();

        for variant in &data.variants {
            let name = &variant.ident;
            let ty = &variant.fields.iter().nth(0).unwrap().ty;

            for meta_item in variant
                .attrs
                .iter()
                .filter_map(get_xml_meta_items)
                .flatten()
            {
                use NestedMeta::Meta;
                match meta_item {
                    Meta(NameValue(ref m)) if m.path.is_ident("tag") => {
                        if let Str(ref lit) = m.lit {
                            elements.push((
                                Variant {
                                    name: name.clone(),
                                    ty: ty.clone(),
                                },
                                bytes_str!(lit.clone()),
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

fn get_xml_meta_items(attr: &Attribute) -> Option<Vec<NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "xml" {
        match attr.parse_meta() {
            Ok(Meta::List(meta)) => Some(meta.nested.iter().cloned().collect()),
            _ => None,
        }
    } else {
        None
    }
}

pub trait TypeExt {
    fn is_option(&self) -> Option<&Self>;
    fn is_vec(&self) -> Option<&Self>;
    fn is_bool(&self) -> bool;
    fn is_string(&self) -> bool;
    fn is_usize(&self) -> bool;
    fn init_value(&self) -> TokenStream;
    fn parse_attr_value(&self) -> TokenStream;
    fn get_ident(&self) -> Option<Ident>;
}

impl TypeExt for Type {
    fn is_option(&self) -> Option<&Self> {
        let path = match self {
            Type::Path(ty) => &ty.path,
            _ => {
                return None;
            }
        };
        let seg = path.segments.last()?;
        let args = match &seg.arguments {
            PathArguments::AngleBracketed(bracketed) => &bracketed.args,
            _ => {
                return None;
            }
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

    fn is_vec(&self) -> Option<&Self> {
        let path = match self {
            Type::Path(ty) => &ty.path,
            _ => {
                return None;
            }
        };
        let seg = path.segments.last()?;
        let args = match &seg.arguments {
            PathArguments::AngleBracketed(bracketed) => &bracketed.args,
            _ => {
                return None;
            }
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

    // Specifies how to initialize this field by its type
    fn init_value(&self) -> TokenStream {
        if self.is_vec().is_some() {
            quote!(Vec::new())
        } else if self.is_bool() {
            quote!(false)
        } else {
            quote!(None)
        }
    }

    // Specifies how to parse attributes value (attr.value) by its type
    fn parse_attr_value(&self) -> TokenStream {
        let ty = self.is_option().unwrap_or(&self);

        if ty.is_string() {
            quote!(String::from_utf8(attr.value.into_owned().to_vec())?)
        } else if ty.is_bool() {
            quote! {{
                let value = ::std::str::from_utf8(attr.value.borrow())?;
                bool::from_str(value).or(usize::from_str(value).map(|v| v != 0))?
            }}
        } else {
            let ty = ty.get_ident();
            quote!( #ty::from_str(::std::str::from_utf8(attr.value.borrow())?)? )
        }
    }

    fn is_string(&self) -> bool {
        self.get_ident().map_or(false, |ty| ty == "String")
    }

    fn is_bool(&self) -> bool {
        self.get_ident().map_or(false, |ty| ty == "bool")
    }

    fn is_usize(&self) -> bool {
        self.get_ident().map_or(false, |ty| ty == "usize")
    }

    fn get_ident(&self) -> Option<Ident> {
        match self {
            Type::Path(ty) => ty.path.segments.last().map(|seg| seg.ident.clone()),
            Type::Reference(ty) => ty.elem.get_ident(),
            _ => None,
        }
    }
}
