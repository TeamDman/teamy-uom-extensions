use teamy_uom_extensions::BINARY;
use teamy_uom_extensions::DECIMAL;
use teamy_uom_extensions::HumanInformationRateExt;
use uom::si::f64::InformationRate;
use uom::si::information_rate::byte_per_second;
use uom::si::information_rate::kilobyte_per_second;

fn main() {
    println!("=== InformationRate Human-Readable Demo ===");

    let rates = [
        ("512 B/s", InformationRate::new::<byte_per_second>(512.0)),
        ("1.5 kB/s", InformationRate::new::<kilobyte_per_second>(1.5)),
        ("1536 B/s", InformationRate::new::<byte_per_second>(1536.0)),
    ];

    for (desc, r) in rates.into_iter() {
        println!(
            "{:<10} -> decimal: {:<10}  binary: {:<10} (raw {} B/s)",
            desc,
            r.format_human(DECIMAL),
            r.format_human(BINARY),
            r.get::<byte_per_second>() as u64
        );
    }
}
