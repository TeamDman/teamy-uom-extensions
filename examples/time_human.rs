use teamy_uom_extensions::HumanTimeExt;
use uom::si::f64::Time;
use uom::si::time::millisecond;
use uom::si::time::minute;
use uom::si::time::second;

fn main() {
    println!("=== Time Human-Readable Demo ===");

    let durations = [
        ("45s", Time::new::<second>(45.0)),
        ("2.5m", Time::new::<minute>(2.5)),
        ("1500ms", Time::new::<millisecond>(1500.0)),
    ];

    for (desc, t) in durations.iter() {
        println!(
            "{:<8} -> human: {:<12}  nanos: {:<12}",
            desc,
            t.format_human(),
            t.format_human_precise()
        );
    }
}
