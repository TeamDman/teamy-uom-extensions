//! Convenience extension to build an f64 InformationRate from an Information and a Time.
//!
//! This helper converts the information amount and the time to f64 values and returns
//! a `uom::si::f64::InformationRate` which is convenient for display and telemetry.
//!
//! It exists because `uom`'s autoconvert can sometimes give surprising units when doing
//! mixed-type arithmetic; this helper makes the conversion explicit and predictable.

/// Extension trait providing `.over(time)` to calculate rate as `InformationRate<f64>`.
pub trait InformationOverExt {
    /// Compute the information rate (bytes per second) using f64 arithmetic.
    fn over<Utime, Vtime>(&self, time: uom::si::time::Time<Utime, Vtime>) -> uom::si::f64::InformationRate
    where
        Utime: uom::si::Units<Vtime> + ?Sized,
        Vtime: uom::num::Num + uom::Conversion<Vtime> + uom::num::ToPrimitive + Copy + PartialOrd,
        uom::si::time::second: uom::Conversion<Vtime, T = Vtime::T>;
}

impl<Uint, Vint> InformationOverExt for uom::si::information::Information<Uint, Vint>
where
    Uint: uom::si::Units<Vint> + ?Sized,
    Vint: uom::num::Num + uom::Conversion<Vint> + uom::num::ToPrimitive + Copy + PartialOrd,
    uom::si::information::byte: uom::Conversion<Vint, T = Vint::T>,
{
    fn over<Utime, Vtime>(&self, time: uom::si::time::Time<Utime, Vtime>) -> uom::si::f64::InformationRate
    where
        Utime: uom::si::Units<Vtime> + ?Sized,
        Vtime: uom::num::Num + uom::Conversion<Vtime> + uom::num::ToPrimitive + Copy + PartialOrd,
        uom::si::time::second: uom::Conversion<Vtime, T = Vtime::T>,
    {
        // Convert both sides to f64 using the byte/second base units, guard against zero
        let bytes = self.get::<uom::si::information::byte>().to_f64().unwrap_or(0.0_f64);
        let secs = time.get::<uom::si::time::second>().to_f64().unwrap_or(0.0_f64);
        let per_sec = if secs > 0.0_f64 { bytes / secs } else { 0.0_f64 };
        uom::si::f64::InformationRate::new::<uom::si::information_rate::byte_per_second>(per_sec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::information::byte;
    use uom::si::time::second;

    #[test]
    fn over_computes_bytes_per_second() {
        let info = uom::si::f64::Information::new::<byte>(2048.0);
        let time = uom::si::f64::Time::new::<second>(2.0);
        let rate = info.over(time);
        let val = rate.get::<uom::si::information_rate::byte_per_second>();
        assert_eq!(val, 1024.0_f64);
    }

    #[test]
    fn zero_time_returns_zero_rate() {
        let info = uom::si::f64::Information::new::<byte>(1024.0);
        let time = uom::si::f64::Time::new::<second>(0.0);
        let rate = info.over(time);
        let val = rate.get::<uom::si::information_rate::byte_per_second>();
        assert_eq!(val, 0.0_f64);
    }
}
