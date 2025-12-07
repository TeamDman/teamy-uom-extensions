use teamy_uom_extensions::HumanInformationExt;
use teamy_uom_extensions::HumanInformationRateExt;
use teamy_uom_extensions::HumanTimeExt;
use uom::si::f64::Time;
use uom::si::information::kibibyte;
use uom::si::time::second;

fn main() {
    println!("=== InformationRate Math Demo ===");
    {
        use uom::si::f64::Information;
        use uom::si::f64::InformationRate;

        let item_count = 1_234_567;
        let item_size = Information::new::<kibibyte>(1.0);
        let time_taken = Time::new::<second>(123.0);
        let total_size = item_count as f64 * item_size;
        let info_rate = InformationRate::from(total_size / time_taken);
        println!(
            "Transferred {} items of size {} in {} seconds ({})",
            item_count,
            item_size.format_human(teamy_uom_extensions::BINARY),
            time_taken.format_human(),
            info_rate.format_human(teamy_uom_extensions::BINARY)
        );
    }
    {
        
        use uom::si::usize::Information;
        use uom::si::f64::InformationRate;

        let item_count = 1_234_567;
        let item_size = Information::new::<kibibyte>(1);
        let time_taken = Time::new::<second>(123.0);
        let total_size = item_count * item_size;
        let info_rate: InformationRate = todo!(); //InformationRate::from(total_size  / time_taken);
        println!(
            "Transferred {} items of size {} in {} seconds ({})",
            item_count,
            item_size.format_human(teamy_uom_extensions::BINARY),
            time_taken.format_human(),
            info_rate.format_human(teamy_uom_extensions::BINARY)
        );
    }
}
