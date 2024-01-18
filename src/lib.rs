#![no_std]

//

extern crate alloc;

//

pub use tempest_core::{sanitized, unsanitized, Sanitized, Unsanitized, View, WrapView};
pub use tempest_macro::view;

//

#[cfg(test)]
mod tests {
    use crate::*;

    mod tempest {
        pub use crate::*;
    }

    #[test]
    fn trivial() {
        let html = view! {
            <div id="test"></div>
        };

        assert_eq!(html.to_string(), r#"<div id="test"></div>"#);
    }

    #[test]
    fn simple_paste() {
        let num = 4;
        let html = view! {
            <div>
                <a>"Num: " {num}</a>
            </div>
        };

        assert_eq!(html.to_string(), "<div><a>Num:&#32;4</a></div>");
    }

    #[test]
    fn components() {
        fn main(num: i32) -> impl View {
            view! {
                <a>"Num: " {num}</a>
            }
        }

        let html = view! {
            <div>
                {main(4)}
            </div>
        };

        assert_eq!(html.to_string(), "<div><a>Num:&#32;4</a></div>");
    }

    #[test]
    fn component_as_arg() {
        fn main(num: impl View) -> impl View {
            view! {
                <div>
                    {num}
                </div>
            }
        }

        let num = 4;
        let html = view! {
            <div>
                {main(view! { <a>"Num: " {num}</a> })}
            </div>
        };

        assert_eq!(html.to_string(), "<div><div><a>Num:&#32;4</a></div></div>");
    }
}
