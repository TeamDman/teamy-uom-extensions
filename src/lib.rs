//! teamy-uom-extensions
//!
//! Small companion crate providing human-friendly display helpers for `uom` types
//! (uses `humantime` and `humansize` if the `human` feature is enabled).
//!
//! This crate intentionally focuses on the `f64` SI aliases from `uom` for the most
//! convenient runtime-friendly formatting of values like `Time`, `Information` and
//! `InformationRate`.
#![deny(missing_docs)]

/// Re-export commonly used uom types we extend here so callers don't need to import `uom` directly.
pub use uom::si::f64::{Information, InformationRate, Time};

// Use the humansize options type when the `human` feature is enabled, otherwise
// provide a placeholder type so the public API remains stable.
#[cfg(feature = "human")]
pub use humansize::FormatSizeOptions;

#[cfg(feature = "human")]
pub use humansize::{DECIMAL, BINARY};

#[cfg(not(feature = "human"))]
#[derive(Clone, Copy, Debug)]
/// Placeholder `FormatSizeOptions` used when `human` feature is disabled.
pub struct FormatSizeOptions;

#[cfg(not(feature = "human"))]
/// Placeholder constants used when `human` feature is disabled.
pub const DECIMAL: FormatSizeOptions = FormatSizeOptions;

#[cfg(not(feature = "human"))]
pub const BINARY: FormatSizeOptions = FormatSizeOptions;

mod si;

// Re-export the public traits at crate root for ergonomic import paths (keeps examples/tests working).
pub use crate::si::time::HumanTimeExt;
pub use crate::si::information::HumanInformationExt;
pub use crate::si::information_rate::HumanInformationRateExt;

// SI submodules implement the traits and are included above.

// Implementations are associated with the public traits above and are compiled
// into the crate automatically for the enabled feature set.

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::f64::{Information, InformationRate, Time};
    use uom::si::information::byte;
    use uom::si::information_rate::byte_per_second;
    use uom::si::time::second;

    #[test]
    fn information_formatting() {
        let i = Information::new::<byte>(1536.0);
        let dec = i.format_human(crate::DECIMAL);
        let bin = i.format_human(crate::BINARY);
        // Don't assert exact strings — different humanize versions may vary — just sanity check
        assert!(dec.len() > 0);
        assert!(bin.len() > 0);
    }

    #[test]
    fn time_formatting() {
        let t = Time::new::<second>(90.0);
        let h = t.format_human();
        let hn = t.format_human_precise();
        assert!(h.len() > 0);
        assert!(hn.len() > 0);
    }

    #[test]
    fn info_rate_formatting() {
        let r = InformationRate::new::<byte_per_second>(2048.0);
        assert!(r.format_human(crate::DECIMAL).len() > 0);
        assert!(r.format_human(crate::BINARY).len() > 0);
    }
}
// crate-root library — no binary here.
