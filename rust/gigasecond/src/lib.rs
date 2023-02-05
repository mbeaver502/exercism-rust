use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const GIGASECOND: i64 = 1_000_000_000;

    start
        .checked_add(Duration::seconds(GIGASECOND))
        .expect("Something went horribly wrong")
}
