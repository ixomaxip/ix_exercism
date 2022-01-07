use std::fmt;

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
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Clock {hours: hours,
                           minutes: minutes};
        clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {hours: self.hours + minutes / 60,
               minutes: self.minutes + minutes % 60}
    }
}
