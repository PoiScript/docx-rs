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
        fn test_read_write() -> Result<()> {
            let _ = env_logger::builder()
                .is_test(true)
                .format_timestamp(None)
                .try_init();

            $(
                // test writing
                let mut writer = vec![];
                ($struct).write(&mut writer)?;
                assert_eq!($string, String::from_utf8(writer)?);
                // test reading
                assert_eq!($struct, $type::from_str($string)?);
            )*

            Ok(())
        }
    };
}
