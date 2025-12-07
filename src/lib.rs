//! teamy-uom-extensions
//!
//! Small companion crate providing human-friendly display helpers for `uom` types
//! (uses `humantime` and `humansize` if the `human` feature is enabled).
//!
//! This crate intentionally focuses on the `f64` SI aliases from `uom` for the most
//! convenient runtime-friendly formatting of values like `Time`, `Information` and
//! `InformationRate`.
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[cfg(all(feature = "full", feature = "human"))]
pub use humansize;
#[cfg(feature = "human")]
pub use humansize::BINARY;
#[cfg(feature = "human")]
pub use humansize::DECIMAL;
// Use the humansize options type when the `human` feature is enabled, otherwise
// provide a placeholder type so the public API remains stable.
#[cfg(feature = "human")]
pub use humansize::FormatSizeOptions;
#[cfg(all(feature = "full", feature = "human"))]
pub use humantime;
/// Re-export commonly used uom types we extend here so callers don't need to import `uom` directly.
///
/// If exactly one storage feature is selected we provide convenient root-level aliases so
/// callers can continue to use `teamy_uom_extensions::Information` like before. If multiple
/// storage types (eg `f32` and `f64`) are enabled we expose per-storage modules
/// `teamy_uom_extensions::f32` and `teamy_uom_extensions::f64` so callers can be explicit.
// The crate no longer provides fine-grain `f32` / `f64` convenience modules.
// If you want everything exported from the underlying crates for demos / prototyping
// enable the `full` feature which re-exports `uom`, `humansize` and `humantime`.
#[cfg(feature = "full")]
pub use uom;

mod si;

// Re-export the public traits at crate root for ergonomic import paths (keeps examples/tests working).
pub use crate::si::information::HumanInformationExt;
pub use crate::si::information_rate::HumanInformationRateExt;
pub use crate::si::time::HumanTimeExt;

// SI submodules implement the traits and are included above.

// Implementations are associated with the public traits above and are compiled
// into the crate automatically for the enabled feature set.

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::information::byte;
    use uom::si::information_rate::byte_per_second;
    use uom::si::time::second;

    #[test]
    fn information_formatting_f64() {
        let i = uom::si::f64::Information::new::<byte>(1536.0_f64);
        assert!(i.format_human(crate::DECIMAL).len() > 0);
        assert!(i.format_human(crate::BINARY).len() > 0);
    }

    #[test]
    fn information_formatting_f32() {
        let i = uom::si::f32::Information::new::<byte>(1536.0_f32);
        assert!(i.format_human(crate::DECIMAL).len() > 0);
        assert!(i.format_human(crate::BINARY).len() > 0);
    }

    #[test]
    fn time_formatting_f64_and_f32() {
        let t64 = uom::si::f64::Time::new::<second>(90.0_f64);
        let _ = t64.format_human();
        let _ = t64.format_human_precise();

        let t32 = uom::si::f32::Time::new::<second>(90.0_f32);
        let _ = t32.format_human();
        let _ = t32.format_human_precise();
    }

    #[test]
    fn info_rate_formatting_f64_and_f32() {
        let r64 = uom::si::f64::InformationRate::new::<byte_per_second>(2048.0_f64);
        assert!(r64.format_human(crate::DECIMAL).len() > 0);
        assert!(r64.format_human(crate::BINARY).len() > 0);

        let r32 = uom::si::f32::InformationRate::new::<byte_per_second>(2048.0_f32);
        assert!(r32.format_human(crate::DECIMAL).len() > 0);
        assert!(r32.format_human(crate::BINARY).len() > 0);
    }
    // The remaining tests are handled by the single/both feature-specific test modules above.
}
// crate-root library — no binary here.
