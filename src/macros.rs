macro_rules! string_enum {
  ($name:ident { $($variant:ident = $value:expr, )* }) => {
    impl AsRef<str> for $name {
      fn as_ref(&self) -> &str {
        match *self {
          $($name::$variant => $value,)*
        }
      }
    }

    impl FromStr for $name {
      type Err = Error;

      fn from_str(s: &str) -> Result<Self> {
        match s {
          $($value => Ok($name::$variant),)*
          s => Err(Error::UnknownValue {
            expected: stringify!($($value,)*).to_string(),
            found: String::from(s),
          })
        }
      }
    }
  }
}
