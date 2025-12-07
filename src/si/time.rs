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

    // Implement for any `uom::si::time::Time<U, V>` where the underlying numeric
    // value can be converted to f64. This lets the extension work with f32, f64
    // and other numeric storage types provided by `uom`.
    impl<U, V> HumanTimeExt for uom::si::time::Time<U, V>
    where
        U: uom::si::Units<V> + ?Sized,
        V: uom::num::Num + uom::Conversion<V> + uom::num::ToPrimitive + PartialOrd + Copy,
        uom::si::time::second: uom::Conversion<V, T = V::T>,
    {
        fn format_human(&self) -> String {
            let secs_v = self.get::<uom::si::time::second>();
            let secs = secs_v.to_f64().unwrap_or(0.0_f64);
            let dur = std::time::Duration::from_secs_f64(secs.abs());
            let formatted = humantime::format_duration(dur).to_string();
            if secs.is_sign_negative() {
                format!("-{}", formatted)
            } else {
                formatted
            }
        }

        fn format_human_precise(&self) -> String {
            let secs_v = self.get::<uom::si::time::second>();
            let secs = secs_v.to_f64().unwrap_or(0.0_f64);
            // Convert to exact nanoseconds (clamped) and format.
            let nanos_f = secs * 1e9_f64;
            let sign = nanos_f.is_sign_negative();
            let nanos_abs = nanos_f.abs().round();
            let nanos_u128 = nanos_abs.min(u128::MAX as f64) as u128;
            let dur = std::time::Duration::from_nanos(nanos_u128 as u64);
            let formatted = humantime::format_duration(dur).to_string();
            if sign {
                format!("-{}", formatted)
            } else {
                formatted
            }
        }
    }
}

#[cfg(not(feature = "human"))]
mod no_human_impl {
    use super::*;

    impl<U, V> HumanTimeExt for uom::si::time::Time<U, V>
    where
        U: uom::si::Units<V> + ?Sized,
        V: uom::num::Num + uom::Conversion<V> + uom::num::ToPrimitive + PartialOrd + Copy,
        uom::si::time::second: uom::Conversion<V, T = V::T>,
        uom::si::time::nanosecond: uom::Conversion<V, T = V::T>,
    {
        fn format_human(&self) -> String {
            let secs_v = self.get::<uom::si::time::second>();
            let secs = secs_v.to_f64().unwrap_or(0.0_f64);
            format!("{} s", secs)
        }

        fn format_human_precise(&self) -> String {
            let secs_v = self.get::<uom::si::time::second>();
            let nanos = secs_v.to_f64().unwrap_or(0.0_f64) * 1e9_f64;
            format!("{} ns", nanos.round())
        }
    }
}
