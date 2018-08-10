//! An implementation of the [Groestl][1] cryptographic hash function.
//!
//! [1]: http://www.groestl.info/
//!
//! # Usage
//!
//! Groestl can produce a digest of any size between 1 and 64 bytes inclusive.
//! This crate defines the common digest sizes (`Groestl224`, `Groestl256`,
//! `Groestl384`, and `Groestl512`), but allows you to specify a custom size
//! with the `GroestlSmall` and `GroestlBig` structs. `GroestlSmall` allows you
//! to specify a digest size between 1 and 32 inclusive, and `GroestlBig` allows
//! you to specify a digest size between 33 and 64 inclusive.
//!
//! ```rust
//! use groestl::{Digest, Groestl256};
//!
//! let mut hasher = Groestl256::default();
//! hasher.input(b"my message");
//! let result = hasher.result();
//! ```

#![no_std]
#[macro_use] extern crate opaque_debug;
#[macro_use] pub extern crate digest;
extern crate byte_tools;
extern crate block_buffer;
#[cfg(feature = "std")]
extern crate std;

pub use digest::Digest;
use digest::{Input, BlockInput, FixedOutput, VariableOutput, Reset};
use digest::InvalidOutputSize;
use digest::generic_array::GenericArray;
use digest::generic_array::typenum::{Unsigned, U28, U32, U48, U64, U128};

mod consts;
mod groestl;
mod state;
mod matrix;
#[macro_use]
mod macros;

use groestl::Groestl;

impl_groestl!(Groestl512, U64, U128);
impl_groestl!(Groestl384, U48, U128);
impl_groestl!(Groestl256, U32, U64);
impl_groestl!(Groestl224, U28, U64);

impl_variable_groestl!(GroestlBig, U128, 32, 64);
impl_variable_groestl!(GroestlSmall, U64, 0, 32);
