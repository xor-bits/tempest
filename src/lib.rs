#![no_std]

//

extern crate alloc;

//

pub use tempest_core::{Part, View};
pub use tempest_macro::view;

//

#[doc(hidden)]
pub const fn __static_part(s: &'static str) -> Part {
    Part::Static(s)
}

#[doc(hidden)]
pub fn __param_part(p: impl core::fmt::Display + 'static) -> Part {
    Part::Parameter(alloc::boxed::Box::new(p))
}
