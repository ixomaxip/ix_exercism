use std::fmt;


#[derive(PartialEq, Debug)]
pub struct Clock {
    minutes: i32
}

const HOUR: i32 = 60;
const DAY: i32 = 60 * 24;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}

impl Clock {

    fn transform_to_minutes(hours: i32, minutes: i32) -> i32 {
        (((hours * HOUR + minutes) % DAY) + DAY) % DAY
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {minutes: Clock::transform_to_minutes(hours, minutes)}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {minutes: Clock::transform_to_minutes(0, self.minutes + minutes)}
    }
}
