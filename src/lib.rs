//! Rust bindings for **The decNumber C library** by IBM Fellow Mike Cowlishaw.

extern crate libc;

mod dec_common;
mod dec_context;
mod dec_context_c;
mod dec_conversion;
mod dec_conversion_c;
mod dec_double;
mod dec_double_c;
mod dec_number;
mod dec_number_c;
mod dec_quad;
mod dec_quad_c;
mod dec_single;
mod dec_single_c;

pub use dec_common::*;
pub use dec_context::*;
pub use dec_context_c::*;
pub use dec_conversion::*;
pub use dec_conversion_c::*;
pub use dec_double::*;
pub use dec_double_c::*;
pub use dec_number::*;
pub use dec_number_c::*;
pub use dec_quad::*;
pub use dec_quad_c::*;
pub use dec_single::*;
pub use dec_single_c::*;
