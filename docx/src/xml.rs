use jetscii::{bytes, BytesConst};
use lazy_static::lazy_static;
use memchr::memchr;
use std::borrow::Cow;
use std::char;

use crate::error::{Error, Result};

pub fn escape<'a>(raw: &'a str) -> Cow<'a, str> {
    lazy_static! {
        static ref ESCAPE_BYTES: BytesConst = bytes!(b'<', b'>', b'&', b'\'', b'"');
    }

    let bytes = raw.as_bytes();

    if let Some(off) = ESCAPE_BYTES.find(bytes) {
        let mut result = String::with_capacity(raw.len());

        result.push_str(&raw[0..off]);

        let mut pos = off + 1;

        match bytes[pos - 1] {
            b'<' => result.push_str("&lt;"),
            b'>' => result.push_str("&gt;"),
            b'&' => result.push_str("&amp;"),
            b'\'' => result.push_str("&apos;"),
            b'"' => result.push_str("&quot;"),
            _ => unreachable!(),
        }

        while let Some(off) = ESCAPE_BYTES.find(&bytes[pos..]) {
            result.push_str(&raw[pos..pos + off]);

            pos += off + 1;

            match bytes[pos - 1] {
                b'<' => result.push_str("&lt;"),
                b'>' => result.push_str("&gt;"),
                b'&' => result.push_str("&amp;"),
                b'\'' => result.push_str("&apos;"),
                b'"' => result.push_str("&quot;"),
                _ => unreachable!(),
            }
        }

        result.push_str(&raw[pos..]);

        Cow::Owned(result)
    } else {
        Cow::Borrowed(raw)
    }
}

pub fn unescape<'a>(raw: &'a str) -> Result<Cow<'a, str>> {
    let bytes = raw.as_bytes();

    if let Some(i) = memchr(b'&', bytes) {
        let mut result = String::with_capacity(raw.len());

        result.push_str(&raw[0..i]);

        let mut pos = i + 1;

        if let Some(i) = memchr(b';', &bytes[pos..]) {
            recognize(&raw[pos..pos + i], &mut result)?;

            pos += i + 1;
        } else {
            return Err(Error::UnterminatedEntity {
                entity: String::from(&raw[pos - 1..]),
            });
        }

        while let Some(i) = memchr(b'&', &bytes[pos..]) {
            result.push_str(&raw[pos..pos + i]);

            pos += i + 1;

            if let Some(i) = memchr(b';', &bytes[pos..]) {
                recognize(&raw[pos..pos + i], &mut result)?;

                pos += i + 1;
            } else {
                return Err(Error::UnterminatedEntity {
                    entity: String::from(&raw[pos - 1..]),
                });
            }
        }

        result.push_str(&raw[pos..]);

        Ok(Cow::Owned(result))
    } else {
        Ok(Cow::Borrowed(raw))
    }
}

fn recognize(entity: &str, result: &mut String) -> Result<()> {
    match entity {
        "quot" => result.push('"'),
        "apos" => result.push('\''),
        "gt" => result.push('>'),
        "lt" => result.push('<'),
        "amp" => result.push('&'),
        _ => {
            let val = if entity.starts_with("#x") {
                u32::from_str_radix(&entity[2..], 16).ok()
            } else if entity.starts_with("#") {
                u32::from_str_radix(&entity[1..], 10).ok()
            } else {
                None
            };
            match val.and_then(char::from_u32) {
                Some(c) => result.push(c),
                None => {
                    return Err(Error::UnrecognizedSymbol {
                        symbol: String::from(entity),
                    })
                }
            }
        }
    }
    Ok(())
}

#[test]
fn test_escape() {
    assert_eq!(escape("< < <"), "&lt; &lt; &lt;");
    assert_eq!(
        escape("<script>alert('Hello XSS')</script>"),
        "&lt;script&gt;alert(&apos;Hello XSS&apos;)&lt;/script&gt;"
    );
}

#[test]
fn test_unescape() {
    assert_eq!(unescape("test").unwrap(), "test");
    assert_eq!(unescape("&lt;test&gt;").unwrap(), "<test>");
    assert_eq!(unescape("&#x30;").unwrap(), "0");
    assert_eq!(unescape("&#48;").unwrap(), "0");
}
