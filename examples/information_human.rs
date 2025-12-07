use teamy_uom_extensions::BINARY;
use teamy_uom_extensions::DECIMAL;
use teamy_uom_extensions::HumanInformationExt;
use uom::si::f64::Information;
use uom::si::information::byte;
use uom::si::information::kibibyte;
use uom::si::information::kilobyte;
use uom::si::information::mebibyte;
use uom::si::information::megabyte;

fn main() {
    println!("=== Information Human-Readable Demo ===");

    let examples = [
        ("512 bytes", Information::new::<byte>(512.0)),
        ("1024 bytes", Information::new::<byte>(1024.0)),
        ("1536 bytes", Information::new::<byte>(1536.0)),
        ("5.5 KB", Information::new::<kilobyte>(5.5)),
        ("250 MB", Information::new::<megabyte>(250.0)),
        ("5.5 KiB", Information::new::<kibibyte>(5.5)),
        ("250 MiB", Information::new::<mebibyte>(250.0)),
    ];

    for (desc, size) in examples.into_iter() {
        println!(
            "{:<10} -> decimal: {:<12}  binary: {:<12} (raw {} B)",
            desc,
            size.format_human(DECIMAL),
            size.format_human(BINARY),
            size.get::<byte>() as u64
        );
    }
}
