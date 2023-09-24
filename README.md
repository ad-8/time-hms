# About

Easily parse a duration represented in seconds to a more 
human-readable representation in hours, minutes and seconds.

# Why?
It's a simple functionality that I needed across different applications.
I also wanted to learn how to publish a library on [crates.io](https://crates.io/).

# Usage

```rust
use time_hms::TimeHms;

let t = TimeHms::new(3723);

// 3723 seconds equal 1 hour, 2 minutes and 3 seconds
assert_eq!(t.h(), 1);
assert_eq!(t.m(), 2);
assert_eq!(t.s(), 3);

// A default way to format! / println! is included
assert_eq!(format!("{}", t), "01:02:03")
```

