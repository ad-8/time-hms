/// SimpleTime represents a duration in hours, minutes and seconds.
#[derive(Debug)]
pub struct TimeHMS {
    h: i32,
    m: i32,
    s: i32,
}

impl TimeHMS {
    /// Converts a duration from a representation in seconds
    /// into a representation in hours, minutes and seconds.
    pub fn new(seconds: i32) -> Result<TimeHMS, &'static str> {
        if seconds < 0 {
            return Err("seconds must be >= 0");
        }

        let (m, s) = divmod(seconds, 60);
        let (h, m) = divmod(m, 60);

        Ok(TimeHMS { h, m, s })
    }
}

/// Takes two numbers as arguments and returns their quotient and remainder when using integer division.
fn divmod(x: i32, y: i32) -> (i32, i32) {
    let quotient = x / y;
    let remainder = x % y;
    (quotient, remainder)
}

#[cfg(test)]
mod tests {
    use crate::TimeHMS;

    #[test]
    fn invalid_arg() {
        let t = TimeHMS::new(-1);
        assert!(t.is_err());
    }

    #[test]
    fn t0() {
        let t = TimeHMS::new(0).unwrap();
        assert_eq!(t.h, 0);
        assert_eq!(t.m, 0);
        assert_eq!(t.s, 0);
    }

    #[test]
    fn t123() {
        let t = TimeHMS::new(3723).unwrap();
        assert_eq!(t.h, 1);
        assert_eq!(t.m, 2);
        assert_eq!(t.s, 3);
    }

    #[test]
    fn t12345() {
        let t = TimeHMS::new(12345).unwrap();
        assert_eq!(t.h, 3);
        assert_eq!(t.m, 25);
        assert_eq!(t.s, 45);
    }

    #[test]
    fn t123456789() {
        let t = TimeHMS::new(123456789).unwrap();
        assert_eq!(t.h, 34293);
        assert_eq!(t.m, 33);
        assert_eq!(t.s, 9);
    }
}
