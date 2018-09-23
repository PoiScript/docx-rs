macro_rules! string_enum {
  ($name:ident { $($variant:ident = $value:expr, )* }) => {
    #[derive(Debug)]
    pub enum $name {
      $($variant,)*
    }

    impl $name {
      fn to_str(&self) -> &'static str {
        match *self {
          $($name::$variant => $value,)*
        }
      }

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
