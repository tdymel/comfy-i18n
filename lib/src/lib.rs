#![doc = include_str!("../README.md")]

pub use small_ctor::ctor;

pub mod macro_use {
    pub use dfmt::Template;
    pub use dfmt::{ArgumentKey, ArgumentValue};
    pub use dfmt::{dformat, dformat_unchecked};
    pub use log::warn;
}

pub use comfy_i18n_macro::{i18n, i18n_init, t};
