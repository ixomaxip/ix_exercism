use duplicate::duplicate;

const EARTH_YEAR: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration {
    seconds: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {seconds: s as f64}
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

#[duplicate(
    planet      period;
    [Mercury]   [0.2408467];
    [Venus]     [0.61519726];
    [Earth]     [1.0];
    [Mars]      [1.8808158];
    [Jupiter]   [11.862615];
    [Saturn]    [29.447498];
    [Uranus]    [84.016846];
    [Neptune]   [164.79132];
)]
impl Planet for planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / period / EARTH_YEAR
    }
}
