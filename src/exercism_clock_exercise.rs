#[derive(Debug)]
pub struct Clock {
    hour: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hour: hours,
            minutes: minutes,
            }
    }
}

#[test]
fn test_on_the_hour() {
    assert_eq!(Clock::new(8, 0).to_string(), "08:00");
}