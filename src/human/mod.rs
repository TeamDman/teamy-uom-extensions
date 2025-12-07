//! Human-friendly helpers - the existing `si` helpers live under this namespace
//! so we can keep feature-gated `human` helpers grouped together.

pub mod si;

// Re-export the existing SI traits at the crate root (lib.rs will re-export these again)
pub use si::information::HumanInformationExt;
pub use si::information_rate::HumanInformationRateExt;
pub use si::time::HumanTimeExt;
