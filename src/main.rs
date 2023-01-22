fn main() {
    let foo = TimeHMS::new(12345);

    println!("{:?}", foo);
}

/// SimpleTime represents a duration in hours, minutes and seconds.
#[derive(Debug)]
struct TimeHMS {
    h: i32,
    m: i32,
    s: i32,
}

impl TimeHMS {
    /// Converts a duration from a representation in seconds
    /// into a representation in hours, minutes and seconds.
    fn new(seconds: i32) -> TimeHMS {
        // TODO return Result (err if seconds < 0)
        let (m, s) = divmod(seconds, 60);
        let (h, m) = divmod(m, 60);
        TimeHMS { h, m, s }
    }
}

/// Takes two numbers as arguments and returns their quotient and remainder when using integer division.
fn divmod(x: i32, y: i32) -> (i32, i32) {
    let quotient = x / y;
    let remainder = x % y;
    (quotient, remainder)
}
