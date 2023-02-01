use std::fmt;

/// TimeHms parses a duration in seconds to a more human-readable representation
/// in hours, minutes and seconds.
/// ```
/// use time_hms::TimeHms;
///
/// let t = TimeHms::new(3723);
///
/// // 3723 seconds equal 1 hour, 2 minutes and 3 seconds
/// assert_eq!(t.h(), 1);
/// assert_eq!(t.m(), 2);
/// assert_eq!(t.s(), 3);
///
/// // A default way to format! / println! is included
/// assert_eq!(format!("{}", t), "01:02:03")
///```
#[derive(Debug, PartialEq)]
pub struct TimeHms {
    h: u64,
    m: u64,
    s: u64,
}

impl TimeHms {
    /// Converts a duration from a representation in seconds
    /// into a representation in hours, minutes and seconds.
    pub fn new(seconds: u64) -> TimeHms {
        let (m, s) = divmod(seconds, 60);
        let (h, m) = divmod(m, 60);

        TimeHms { h, m, s }
    }

    /// Returns the hour part of the duration.
    pub fn h(&self) -> u64 {
        self.h
    }

    /// Returns the minute part of the duration.
    pub fn m(&self) -> u64 {
        self.m
    }

    /// Returns the seconds part of the duration.
    pub fn s(&self) -> u64 {
        self.s
    }
}

impl fmt::Display for TimeHms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.h, self.m, self.s)
    }
}

/// Takes two numbers as arguments and returns their quotient and remainder when using integer division.
fn divmod(x: u64, y: u64) -> (u64, u64) {
    let quotient = x / y;
    let remainder = x % y;
    (quotient, remainder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divmod() {
        let test_cases = vec![
            (5, 2, (2, 1)),
            (10, 3, (3, 1)),
            (13, 5, (2, 3)),
            (0, 5, (0, 0)),
            (u64::MAX, 1, (u64::MAX, 0)),
            (u64::MAX, u64::MAX, (1, 0)),
        ];

        for (x, y, expected) in test_cases {
            assert_eq!(divmod(x, y), expected);
        }
    }

    #[test]
    fn valid_args() {
        let test_cases = vec![
            (0, (0, 0, 0)),
            (12345, (3, 25, 45)),
            (123456789, (34293, 33, 9)),
        ];

        for (input, expected) in test_cases {
            let t = TimeHms::new(input);
            let (h, m, s) = expected;
            assert_eq!(t.h, h);
            assert_eq!(t.m, m);
            assert_eq!(t.s, s);
        }
    }

    #[test]
    fn equal() {
        let t1 = TimeHms::new(3661);
        let t2 = TimeHms::new(3661);
        assert_eq!(t1, t2);
    }

    #[test]
    fn not_equal() {
        let t1 = TimeHms::new(3661);
        let t2 = TimeHms::new(3662);
        assert_ne!(t1, t2);
    }
}
