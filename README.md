# teamy-uom-extensions

Small companion crate that adds human-friendly formatting helpers for common `uom::si::f64` quantity types.

Features
- Human readable formatting for Time, Information, and InformationRate.
- Integrates with `humantime` and `humansize` (feature: `human`, enabled by default).

Quick start

Build & run tests:

```pwsh
cargo test
```

Run examples (default features include `human`):

```pwsh
cargo run --example information_human
cargo run --example information_rate_human
cargo run --example time_human
cargo run --example combined_human
```

If you want to compile without the human-friendly formatting (e.g., for no_std or smaller binary):

```pwsh
cargo run --example information_human --no-default-features
```

This crate intentionally targets `uom::si::f64` aliases so examples are simple and ergonomic.  Contributions welcome.
