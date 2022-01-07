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
    fn correct_time(hours: i32, minutes: i32) -> (i32, i32) {
        let hours = ((hours % 24) + 24) % 24;
        let whole_minutes = hours * 60 + minutes;
        println!("whole: {:?}", whole_minutes);
        let hours = whole_minutes / 60 % 24;
        let hours = ((hours % 24) + 24) % 24;
        
        // let minutes = whole_minutes % 60;
        let minutes = ((whole_minutes % 60) + 60) % 60;

        // println!("{:?}", (minutes / 60));
        (hours, minutes)
    }
    pub fn new(hours: i32, minutes: i32) -> Self {
        let correct_time = Clock::correct_time(hours, minutes);
        // let minutes = minutes % 60;
        let clock = Clock {hours: correct_time.0,
                           minutes: correct_time.1};
        clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hours = self.hours + minutes / 60;
        let minutes = self.minutes + minutes % 60;
        Clock {hours: hours % 24,
               minutes: minutes % 60}
    }
}
