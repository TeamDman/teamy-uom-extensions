//! InformationRate-specific human formatting helpers and trait.

/// Trait with human-friendly formatting helpers for `InformationRate`.
pub trait HumanInformationRateExt {
    /// Human friendly representation using the provided `FormatSizeOptions` with a "/s" suffix.
    fn format_human(&self, options: crate::FormatSizeOptions) -> String;
}

#[cfg(feature = "human")]
mod human_impl {
    use super::*;
    use humansize::format_size;

    impl<U, V> HumanInformationRateExt for uom::si::information_rate::InformationRate<U, V>
    where
        U: uom::si::Units<V> + ?Sized,
        V: uom::num::Num + uom::Conversion<V> + uom::num::ToPrimitive + Copy + PartialOrd,
        uom::si::information_rate::byte_per_second: uom::Conversion<V, T = V::T>,
    {
        fn format_human(&self, options: crate::FormatSizeOptions) -> String {
            let bytes_v = self.get::<uom::si::information_rate::byte_per_second>();
            let bytes = bytes_v.to_f64().unwrap_or(0.0_f64);
            if bytes <= 0.0 {
                return "0 B/s".to_string();
            }
            let val = bytes.max(0.0).round() as u64;
            format!("{}/s", format_size(val, options))
        }
    }
}

#[cfg(not(feature = "human"))]
mod no_human_impl {
    use super::*;

    impl<U, V> HumanInformationRateExt for uom::si::information_rate::InformationRate<U, V>
    where
        U: uom::si::Units<V> + ?Sized,
        V: uom::num::Num + uom::Conversion<V> + uom::num::ToPrimitive + Copy + PartialOrd,
        uom::si::information_rate::byte_per_second: uom::Conversion<V, T = V::T>,
    {
        fn format_human(&self, _options: crate::FormatSizeOptions) -> String {
            let bytes_v = self.get::<uom::si::information_rate::byte_per_second>();
            let bytes = bytes_v.to_f64().unwrap_or(0.0_f64);
            format!("{} B/s", bytes)
        }
    }
}
