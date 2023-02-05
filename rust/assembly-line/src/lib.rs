// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let units_per_hour = 221.0;

    speed as f64
        * units_per_hour
        * match speed {
            1..=4 => 1.0,
            5..=8 => 0.9,
            9..=10 => 0.77,
            _ => 0.0,
        }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let units = production_rate_per_hour(speed);
    let per_minute = units / 60.0;

    per_minute.floor() as u32
}
