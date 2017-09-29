use std::time::Duration;

pub trait TimeUtil {
    fn as_millis(&self) -> u64;
    fn as_nanos(&self) -> u64;
    fn mul_decimal(&self, k: f32) -> Duration;
}

impl TimeUtil for Duration {
    fn as_millis(&self) -> u64 {
        return self.as_secs() * 1_000 + self.subsec_nanos() as u64 / 1_000_000;
    }

    fn as_nanos(&self) -> u64 {
        return self.as_secs() * 1_000_000_000 + self.subsec_nanos() as u64;
    }

    fn mul_decimal(&self, k: f32) -> Duration {
        Duration::from_millis((self.as_millis() as f32 * k) as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_millis_returns_number_of_milliseconds_super_second() {
        assert_eq!(Duration::from_millis(500341).as_millis(), 500341);
    }

    #[test]
    fn as_millis_returns_number_of_milliseconds_sub_second() {
        assert_eq!(Duration::from_millis(12).as_millis(), 12);
    }

    #[test]
    fn as_nanos_returns_number_of_milliseconds_super_second() {
        assert_eq!(Duration::from_millis(500341).as_nanos(), 500341000000);
    }

    #[test]
    fn as_nanos_returns_number_of_milliseconds_sub_second() {
        assert_eq!(Duration::from_millis(12).as_nanos(), 12000000);
    }

    #[test]
    fn mul_decimal_provides_super_second_results() {
        assert_eq!(Duration::from_millis(500).mul_decimal(2.5).as_millis(), 1250);
    }

    #[test]
    fn mul_decimal_provides_sub_second_results() {
        assert_eq!(Duration::from_millis(500).mul_decimal(0.1).as_millis(), 50);
    }
}