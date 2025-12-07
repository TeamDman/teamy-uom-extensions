//! Time-specific human formatting helpers and trait.
#![allow(unused_imports)]

/// Trait with human-friendly formatting helpers for `Time`.
pub trait HumanTimeExt {
    /// Human friendly time (approximate, e.g. "1.5m", "2s", "3ms").
    fn format_human(&self) -> String;

    /// Human readable representation with nanosecond precision where appropriate.
    fn format_human_precise(&self) -> String;
}

#[cfg(feature = "human")]
mod human_impl {
    use super::*;
    use humantime;

    impl HumanTimeExt for crate::Time {
        fn format_human(&self) -> String {
            let secs = self.get::<uom::si::time::second>();
            let dur = std::time::Duration::from_secs_f64(secs.abs());
            let formatted = humantime::format_duration(dur).to_string();
            if secs.is_sign_negative() { format!("-{}", formatted) } else { formatted }
        }

        fn format_human_precise(&self) -> String {
            let secs = self.get::<uom::si::time::second>();
            // Convert to exact nanoseconds (clamped) and format.
            let nanos_f = secs * 1e9_f64;
            let sign = nanos_f.is_sign_negative();
            let nanos_abs = nanos_f.abs().round();
            let nanos_u128 = nanos_abs.min(u128::MAX as f64) as u128;
            let dur = std::time::Duration::from_nanos(nanos_u128 as u64);
            let formatted = humantime::format_duration(dur).to_string();
            if sign { format!("-{}", formatted) } else { formatted }
        }
    }
}

#[cfg(not(feature = "human"))]
mod no_human_impl {
    use super::*;

    impl HumanTimeExt for crate::Time {
        fn format_human(&self) -> String {
            format!("{} s", self.get::<uom::si::time::second>())
        }

        fn format_human_precise(&self) -> String {
            format!("{} ns", (self.get::<uom::si::time::second>() * 1e9_f64).round())
        }
    }
}
