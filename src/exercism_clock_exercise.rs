
use std::fmt;

const MINS_IN_HOUR: i32 = 60;
const MAX_MINUTES: i32 = 24* 60;



#[derive(Debug)]
pub struct Clock{
    hour: i32,
    minutes: i32
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "{:02}:{:02}", self.hour, self.minutes)
        
    }
}

impl Clock {
    pub fn new( hour: i32, minutes: i32) -> Self {
        let total_minutes: i32 = (((hour * 60 + minutes)% MAX_MINUTES) + MAX_MINUTES) % MAX_MINUTES;
        Clock {
            hour: total_minutes/MINS_IN_HOUR,
            minutes: total_minutes % MINS_IN_HOUR
        }
    }

    pub fn add_minutes(&self, minutes:i32 ) -> Self{
        Clock::new( self.hour, self.minutes + minutes)
    }

}


    
    


#[test]
fn test_on_the_hour() {
    // assert_eq!(format!("{}", Clock {hour: 12, minutes: 32}), "04:32");
    assert_eq!(Clock::new(8, 32).to_string(), "08:32");
}