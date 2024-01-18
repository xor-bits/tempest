#![no_std]

//

extern crate alloc;

use core::fmt;

use alloc::{boxed::Box, string::ToString};

//

// #[derive(Debug)]
// #[repr(transparent)]
// pub struct View([Part]);

// impl View {
//     #[doc(hidden)]
//     pub const fn __unsafe_from_parts(parts: &[Part]) -> &Self {
//         unsafe { core::mem::transmute::<&[Part], &Self>(parts) }
//     }
// }

#[derive(Debug)]
pub struct View<const N: usize>([Part; N]);

impl<const N: usize> View<N> {
    #[doc(hidden)]
    pub const fn __new(parts: [Part; N]) -> Self {
        Self(parts)
    }
}

//

pub enum Part {
    Static(&'static str),
    Parameter(Box<dyn fmt::Display>),
}

//

impl<const N: usize> fmt::Display for View<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for part in self.0.iter() {
            fmt::Display::fmt(part, f)?;
        }

        Ok(())
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Part::Static(s) => f.write_str(s),
            Part::Parameter(p) => p.fmt(f),
        }
    }
}

impl fmt::Debug for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Part::Static(s) => f.debug_tuple("Static").field(s).finish(),
            Part::Parameter(p) => f.debug_tuple("Parameter").field(&p.to_string()).finish(),
        }
    }
}
