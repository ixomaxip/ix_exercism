use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {

    fn get_time_from_minutes(minutes: i32) -> (i32, i32) {
        let hours = minutes / 60 % 24;
        let minutes = minutes % 60;
        (hours, minutes)
    }

    fn transform_to_minutes(hours: i32, minutes: i32) -> i32 {
        let min_in_day = 24 * 60;
        let in_minutes = hours * 60 + minutes;
        ((in_minutes % min_in_day) + min_in_day) % min_in_day
    }

    fn correct_time(hours: i32, minutes: i32) -> (i32, i32) {
        let in_minutes = Clock::transform_to_minutes(hours, minutes);
        Clock::get_time_from_minutes(in_minutes)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes)= Clock::correct_time(hours, minutes);
        let clock = Clock {hours: hours,
                           minutes: minutes};
        clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) = Clock::correct_time(self.hours, self.minutes + minutes);
        let clock = Clock {hours: hours,
                           minutes: minutes};
        clock
    }
}
