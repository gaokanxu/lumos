//! Crate defining a library with a procedural macro and other
//! dependencies for building Lumos program errors

#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

extern crate self as lpl_program_error;

// Make these available downstream for the macro to work without
// additional imports
pub use {
    num_derive, num_traits, lumos_program,
    lpl_program_error_derive::{
        lpl_program_error, DecodeError, IntoProgramError, PrintProgramError,
    },
    thiserror,
};
