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
            type Err = strong_xml::XmlError;

            fn from_str(s: &str) -> strong_xml::XmlResult<Self> {
                match s {
                    $($value => Ok($name::$variant),)*
                    s => Err(strong_xml::XmlError::UnknownValue {
                        expected: stringify!($($value,)*).to_owned(),
                        found: String::from(s),
                    })
                }
            }
        }
    }
}

#[macro_export]
macro_rules! __setter {
    ($field:ident: Option<$ty:ty>) => {
        #[inline(always)]
        pub fn $field<T: Into<$ty>>(mut self, value: T) -> Self {
            self.$field = Some(value.into());
            self
        }
    };
    ($field:ident: $ty:ty) => {
        #[inline(always)]
        pub fn $field<T: Into<$ty>>(mut self, value: T) -> Self {
            self.$field = value.into();
            self
        }
    };
}

#[macro_export]
macro_rules! __test_read_write {
    ($type:tt, $($struct:expr, $string:expr,)*) => {
        #[test]
        fn test_read_write() -> strong_xml::XmlResult<()> {
            let _ = env_logger::builder()
                .is_test(true)
                .format_timestamp(None)
                .try_init();

            $(
                // test writing
                assert_eq!($string, ($struct).to_string()?);
                // test reading
                assert_eq!($struct, $type::from_str($string)?);
            )*

            Ok(())
        }
    };
}
