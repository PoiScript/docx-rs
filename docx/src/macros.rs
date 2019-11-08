#[macro_export]
macro_rules! __string_enum {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match *self {
                    $( $name::$variant => write!(f, $value), )*
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self> {
                match s {
                    $($value => Ok($name::$variant),)*
                    s => Err(Error::UnknownValue {
                        expected: stringify!($($value,)*).to_owned(),
                        found: String::from(s),
                    })
                }
            }
        }
    }
}
