//! Definitions for the native LUM token and its fractional lamports.

#![allow(clippy::arithmetic_side_effects)]

/// There are 10^9 lamports in one LUM
pub const LAMPORTS_PER_LUM: u64 = 1_000_000_000;

/// Approximately convert fractional native tokens (lamports) into native tokens (LUM)
pub fn lamports_to_lum(lamports: u64) -> f64 {
    lamports as f64 / LAMPORTS_PER_LUM as f64
}

/// Approximately convert native tokens (LUM) into fractional native tokens (lamports)
pub fn sol_to_lamports(sol: f64) -> u64 {
    (sol * LAMPORTS_PER_LUM as f64) as u64
}

use std::fmt::{Debug, Display, Formatter, Result};
pub struct Sol(pub u64);

impl Sol {
    fn write_in_lum(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "◎{}.{:09}",
            self.0 / LAMPORTS_PER_LUM,
            self.0 % LAMPORTS_PER_LUM
        )
    }
}

impl Display for Sol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_lum(f)
    }
}

impl Debug for Sol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_lum(f)
    }
}
