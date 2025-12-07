use teamy_uom_extensions::{Information, HumanInformationExt, DECIMAL, BINARY};
use uom::si::information::{byte, kilobyte, megabyte, kibibyte, mebibyte};

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
        println!("{:<10} -> decimal: {:<12}  binary: {:<12} (raw {} B)",
            desc,
            size.format_human(DECIMAL),
            size.format_human(BINARY),
            size.get::<byte>() as u64
        );
    }
}
