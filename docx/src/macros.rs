macro_rules! string_enum {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        impl std::convert::AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                match *self {
                    $( $name::$variant => $value.as_bytes(), )*
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self> {
                match s {
                    $($value => Ok($name::$variant),)*
                    s => Err(Error::UnknownValue {
                        expected: stringify!($($value,)*),
                        found: String::from(s),
                    })
                }
            }
        }
    }
}

macro_rules! w_val_element {
    ($name: ident, $tag:literal, $ty:ty) => {
        #[derive(Debug, Xml)]
        #[xml(tag = $tag)]
        #[xml(leaf)]
        pub struct $name {
            #[xml(attr = "w:val")]
            pub value: $ty,
        }

        impl $name {
            pub fn new(value: $ty) -> Self {
                $name { value }
            }
        }
    };
}
