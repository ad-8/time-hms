use time_hms::TimeHMS;

fn main() {
    let yes = TimeHMS::new(12345);
    let no = TimeHMS::new(-1);
    println!("{:?}", yes);
    println!("{:?}", no);
}
