//! Sub-modules for SI-related human formatting helpers.
#![allow(unused_imports)]
//!
//! This module contains the `time`, `information` and `information_rate` submodules
//! which provide public traits and implementations for the `uom::si::f64` aliases.

pub mod information;
pub mod information_rate;
pub mod time;

// Re-export traits for ergonomic access from the crate root.
pub use information::HumanInformationExt;
pub use information_rate::HumanInformationRateExt;
pub use time::HumanTimeExt;
