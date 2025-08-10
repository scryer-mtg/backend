use std::time::Duration;

pub trait NumberDurationExt {
    fn hours(self) -> Duration;
    fn minutes(self) -> Duration;
    fn seconds(self) -> Duration;
    fn milliseconds(self) -> Duration;
    fn microseconds(self) -> Duration;
}

impl NumberDurationExt for u64 {
    fn hours(self) -> Duration {
        from_hours(self)
    }

    fn minutes(self) -> Duration {
        from_minutes(self)
    }

    fn seconds(self) -> Duration {
        from_seconds(self)
    }

    fn milliseconds(self) -> Duration {
        from_milliseconds(self)
    }

    fn microseconds(self) -> Duration {
        from_microseconds(self)
    }
}

pub const fn from_hours(hours: u64) -> Duration {
    Duration::from_secs(hours * 3600)
}

pub const fn from_minutes(minutes: u64) -> Duration {
    Duration::from_secs(minutes * 60)
}

pub const fn from_seconds(seconds: u64) -> Duration {
    Duration::from_secs(seconds)
}

pub const fn from_milliseconds(milliseconds: u64) -> Duration {
    Duration::from_millis(milliseconds)
}

pub const fn from_microseconds(microseconds: u64) -> Duration {
    Duration::from_micros(microseconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! extension_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (ext_res, fn_res, actual_duration) = $value;
                    assert_eq!(fn_res, actual_duration);
                    assert_eq!(ext_res, actual_duration);
                    assert_eq!(ext_res, fn_res);
                }
            )*
        };
    }

    extension_tests! {
        test_0_hours: (0.hours(), from_hours(0), Duration::from_secs(0)),
        test_0_minutes: (0.minutes(), from_minutes(0), Duration::from_secs(0)),
        test_0_seconds: (0.seconds(), from_seconds(0), Duration::from_secs(0)),
        test_0_milliseconds: (0.milliseconds(), from_milliseconds(0), Duration::from_secs(0)),
        test_0_microseconds: (0.microseconds(), from_microseconds(0), Duration::from_secs(0)),
        test_1_hours: (1.hours(), from_hours(1), Duration::from_secs(3600)),
        test_1_minutes: (1.minutes(), from_minutes(1), Duration::from_secs(60)),
        test_1_seconds: (1.seconds(), from_seconds(1), Duration::from_secs(1)),
        test_1_milliseconds: (1.milliseconds(), from_milliseconds(1), Duration::from_millis(1)),
        test_1_microseconds: (1.microseconds(), from_microseconds(1), Duration::from_micros(1)),
        test_12_hours: (12.hours(), from_hours(12), Duration::from_secs(12 * 3600)),
        test_60_minutes: (60.minutes(), from_minutes(60), Duration::from_secs(60 * 60)),
        test_60_seconds: (60.seconds(), from_seconds(60), Duration::from_secs(60)),
        test_1000_milliseconds: (1000.milliseconds(), from_milliseconds(1000), Duration::from_millis(1000)),
        test_1000_microseconds: (1000.microseconds(), from_microseconds(1000), Duration::from_micros(1000)),
    }
}
