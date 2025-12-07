//! Information-specific human formatting helpers and trait.

/// Trait with human-friendly formatting helpers for `Information`.
pub trait HumanInformationExt {
    /// Human friendly representation using the provided `FormatSizeOptions`.
    ///
    /// Use `crate::DECIMAL` or `crate::BINARY` to request decimal (SI) or binary (IEC)
    /// formatting. When the `human` feature is enabled the provided `FormatSizeOptions`
    /// is passed through to `humansize::format_size` directly.
    fn format_human(&self, options: crate::FormatSizeOptions) -> String;
}

#[cfg(feature = "human")]
mod human_impl {
    use super::*;
    use humansize::format_size;

    impl HumanInformationExt for crate::Information {
        fn format_human(&self, options: crate::FormatSizeOptions) -> String {
            let bytes = self.get::<uom::si::information::byte>();
            if bytes <= 0.0 { return "0 B".to_string(); }
            let val = bytes.max(0.0).round() as u64;
            format_size(val, options)
        }
    }
}

#[cfg(not(feature = "human"))]
mod no_human_impl {
    use super::*;

    impl HumanInformationExt for crate::Information {
        fn format_human(&self, _options: crate::FormatSizeOptions) -> String {
            format!("{} B", self.get::<uom::si::information::byte>())
        }
    }
}
