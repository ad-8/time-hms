fn main() {
    let foo = TimeHMS::new(12345);

    println!("{:?}", foo);
}

#[derive(Debug)]
struct TimeHMS {
    h: i32,
    m: i32,
    s: i32,
}

impl TimeHMS {
    fn new(seconds: i32) -> TimeHMS {
        let (m, s) = divmod(seconds, 60);
        let (h, m) = divmod(m, 60);
        TimeHMS { h, m, s }
    }
}

fn divmod(x: i32, y: i32) -> (i32, i32) {
    let quotient = x / y;
    let remainder = x % y;
    (quotient, remainder)
}
