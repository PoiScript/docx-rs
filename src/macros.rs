#[macro_export]
#[doc(hidden)]
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
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($value => Ok($name::$variant),)*
                    s => Err(format!(
                        "Unkown Value. Found `{}`, Expected `{}`",
                        s,
                        stringify!($($value,)*)
                    ))
                }
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
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
#[doc(hidden)]
macro_rules! __xml_test_suites {
    ($type:tt, $($struct:expr, $string:expr,)*) => {
        #[test]
        fn xml_test_suites() -> strong_xml::XmlResult<()> {
            let _ = env_logger::builder()
                .is_test(true)
                .format_timestamp(None)
                .try_init();

            $(
                assert_eq!($string, ($struct).to_string()?);
                assert_eq!($struct, $type::from_str($string)?);
            )*

            Ok(())
        }
    };
}
