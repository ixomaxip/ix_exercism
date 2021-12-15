// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    
    let rate = speed as f64 * 221.0;
    match speed {
        1..=4 => rate,
        5..=8 => 0.9 * rate,
        9..=10 => 0.77 * rate,
        _ =>  0.0
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    
    let rate_per_minute = production_rate_per_hour(speed) / 60.0;
    return rate_per_minute as u32
}
