use teamy_uom_extensions::{
    HumanInformationExt,
    HumanInformationRateExt,
    HumanTimeExt,
    Information,
    InformationRate,
    Time,
    DECIMAL,
    BINARY,
};
use uom::si::information::{megabyte, gigabyte};
use uom::si::time::{second, minute};
use uom::si::information_rate::byte_per_second;

fn main() {
    println!("=== Combined Human-Readable UOM Demo ===\n");

    let file_size = Information::new::<megabyte>(250.0);
    let transfer_time = Time::new::<minute>(2.5);
    let transfer_rate: InformationRate = (file_size / transfer_time).into();

    println!("File size:     {} ({})", file_size.format_human(DECIMAL), file_size.format_human(BINARY));
    println!("Transfer time: {} ({})", transfer_time.format_human(), transfer_time.format_human_precise());
    println!("Transfer rate: {} ({})", transfer_rate.format_human(DECIMAL), transfer_rate.format_human(BINARY));

    let backup_size = Information::new::<gigabyte>(1.5);
    let backup_time = Time::new::<second>(450.0);
    let backup_rate: InformationRate = (backup_size / backup_time).into();

    println!("\nBackup size: {} ({})", backup_size.format_human(DECIMAL), backup_size.format_human(BINARY));
    println!("Backup rate: {} ({})", backup_rate.format_human(DECIMAL), backup_rate.format_human(BINARY));

    let stream_rate = InformationRate::new::<byte_per_second>(2_500_000.0);
    let stream_duration = Time::new::<minute>(120.0);
    let total_streamed: Information = (stream_rate * stream_duration).into();

    println!("\nStream rate: {}  duration: {}  total: {}", stream_rate.format_human(DECIMAL), stream_duration.format_human(), total_streamed.format_human(DECIMAL));
}
