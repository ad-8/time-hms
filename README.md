# About

Easily parse a duration represented in seconds to a more 
human-readable representation in hours, minutes and seconds.

# Usage

```rust
use time_hms::TimeHMS;

let t = TimeHMS::new(3723).unwrap();

// 3723 seconds equal 1 hour, 2 minutes and 3 seconds
assert_eq!(t.h(), 1);
assert_eq!(t.m(), 2);
assert_eq!(t.s(), 3);

// A default way to format! / println! is included
assert_eq!(format!("{}", t), "01:02:03")
```

