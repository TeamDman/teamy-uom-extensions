# teamy-uom-extensions — quick demo

Tiny examples showing the core formatting helpers. These examples use the default features
(`human`) so formatting helpers are available out of the box.

Information (bytes)
```rust
use teamy_uom_extensions::{HumanInformationExt, DECIMAL, BINARY};
use uom::si::f64::Information;
use uom::si::information::byte;

let i = Information::new::<byte>(1536.0);
println!("dec: {}  bin: {}", i.format_human(DECIMAL), i.format_human(BINARY));
```
Output (example):
```text
dec: 1.54 kB  bin: 1.50 KiB
```

Time
```rust
use teamy_uom_extensions::HumanTimeExt;
use uom::si::f64::Time;
use uom::si::time::second;

let t = Time::new::<second>(90.0);
println!("human: {}  precise: {}", t.format_human(), t.format_human_precise());
```
Output (example):
```text
human: 1m 30s  precise: 1m 30s
```

InformationRate
```rust
use teamy_uom_extensions::{HumanInformationRateExt, DECIMAL};
use uom::si::f64::InformationRate;
use uom::si::information_rate::byte_per_second;

let r = InformationRate::new::<byte_per_second>(2048.0);
println!("rate: {}", r.format_human(DECIMAL));
```
Output (example):
```text
rate: 2.05 kB/s
```

That's it — quick, minimal examples to demonstrate the core helpers. For full examples see the `examples/` directory.
