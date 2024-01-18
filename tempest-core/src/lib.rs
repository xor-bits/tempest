#![no_std]

//

extern crate alloc;

use core::fmt::{self, Write};

use alloc::string::{String, ToString};

//

pub fn sanitized<T>(val: T) -> Sanitized<T> {
    Sanitized(val)
}

pub fn unsanitized<T>(val: T) -> Unsanitized<T> {
    Unsanitized(val)
}

//

pub struct WrapView<F>(pub F);

//

pub struct DisplayView<'a, V: ?Sized>(pub &'a V);

impl<V: View + ?Sized> fmt::Display for DisplayView<'_, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        View::fmt(self.0, f)
    }
}

//

pub trait View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;

    fn display(&self) -> DisplayView<Self> {
        DisplayView(self)
    }

    fn to_string(&self) -> String {
        ToString::to_string(&self.display())
    }
}

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> View for WrapView<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(f)
    }
}

// impl<V: View, F: Fn() -> V> View for F {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self().fmt(f)
//     }
// }

impl<T: fmt::Display> View for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&Sanitized(self), f)
    }
}

//

pub struct Sanitized<T: ?Sized>(T);

impl<T: fmt::Display + ?Sized> fmt::Display for Sanitized<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Sanitizer { f }.write_fmt(format_args!("{}", &self.0))
    }
}

struct Sanitizer<'a, 'b> {
    f: &'a mut fmt::Formatter<'b>,
}

impl fmt::Write for Sanitizer<'_, '_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            if let Some(replacement) = clean_text(c) {
                self.f.write_str(replacement)?;
            } else {
                self.f.write_char(c)?;
            }
        }

        Ok(())
    }
}

//

pub struct Unsanitized<T: ?Sized>(T);

impl<T: fmt::Display + ?Sized> View for Unsanitized<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}", &self.0))
    }
}

//

// stolen (+ adapted) from: https://github.com/rust-ammonia/ammonia
fn clean_text(c: char) -> Option<&'static str> {
    let replacement = match c {
        // this character, when confronted, will start a tag
        '<' => "&lt;",
        // in an unquoted attribute, will end the attribute value
        '>' => "&gt;",
        // in an attribute surrounded by double quotes, this character will end the attribute value
        '\"' => "&quot;",
        // in an attribute surrounded by single quotes, this character will end the attribute value
        '\'' => "&apos;",
        // in HTML5, returns a bogus parse error in an unquoted attribute, while in SGML/HTML, it will end an attribute value surrounded by backquotes
        '`' => "&grave;",
        // in an unquoted attribute, this character will end the attribute
        '/' => "&#47;",
        // starts an entity reference
        '&' => "&amp;",
        // if at the beginning of an unquoted attribute, will get ignored
        '=' => "&#61;",
        // will end an unquoted attribute
        ' ' => "&#32;",
        '\t' => "&#9;",
        '\n' => "&#10;",
        '\x0c' => "&#12;",
        '\r' => "&#13;",
        // a spec-compliant browser will perform this replacement anyway, but the middleware might not
        '\0' => "&#65533;",
        // ALL OTHER CHARACTERS ARE PASSED THROUGH VERBATIM
        _ => return None,
    };
    Some(replacement)
}
