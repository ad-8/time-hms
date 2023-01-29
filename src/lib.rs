use std::fmt;

/// TimeHMS parses a duration in seconds to a more human-readable representation
/// in hours, minutes and seconds.
/// ```
/// use time_hms::TimeHMS;
///
/// let t = TimeHMS::new(3723).unwrap();
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
pub struct TimeHMS {
    h: i64,
    m: i64,
    s: i64,
}

impl TimeHMS {
    /// Converts a duration from a representation in seconds
    /// into a representation in hours, minutes and seconds.
    pub fn new(seconds: i64) -> Result<TimeHMS, &'static str> {
        if seconds < 0 {
            return Err("seconds must be >= 0");
        }

        let (m, s) = divmod(seconds, 60);
        let (h, m) = divmod(m, 60);

        Ok(TimeHMS { h, m, s })
    }

    /// Returns the hour part of the duration.
    pub fn h(&self) -> i64 {
        self.h
    }

    /// Returns the minute part of the duration.
    pub fn m(&self) -> i64 {
        self.m
    }

    /// Returns the seconds part of the duration.
    pub fn s(&self) -> i64 {
        self.s
    }
}

impl fmt::Display for TimeHMS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.h, self.m, self.s)
    }
}

/// Takes two numbers as arguments and returns their quotient and remainder when using integer division.
fn divmod(x: i64, y: i64) -> (i64, i64) {
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
            (-8, 3, (-2, -2)),
            (8, -3, (-2, 2)),
            (-8, -3, (2, -2)),
            (i64::MIN, 1, (i64::MIN, 0)),
            (i64::MAX, 1, (i64::MAX, 0)),
            (i64::MIN, i64::MIN, (1, 0)),
            (i64::MAX, i64::MAX, (1, 0)),
        ];

        for (x, y, expected) in test_cases {
            assert_eq!(divmod(x, y), expected);
        }
    }

    #[test]
    fn invalid_arg() {
        let t = TimeHMS::new(-1);
        assert!(t.is_err());
    }

    #[test]
    fn valid_args() {
        let test_cases = vec![
            (0, (0, 0, 0)),
            (12345, (3, 25, 45)),
            (123456789, (34293, 33, 9)),
        ];

        for (input, expected) in test_cases {
            let t = TimeHMS::new(input).unwrap();
            assert_eq!(t.h, expected.0);
            assert_eq!(t.m, expected.1);
            assert_eq!(t.s, expected.2);
        }
    }

    #[test]
    fn equal() {
        let t1 = TimeHMS::new(3661).unwrap();
        let t2 = TimeHMS::new(3661).unwrap();
        assert_eq!(t1, t2);
    }

    #[test]
    fn not_equal() {
        let t1 = TimeHMS::new(3661).unwrap();
        let t2 = TimeHMS::new(3662).unwrap();
        assert_ne!(t1, t2);
    }
}
