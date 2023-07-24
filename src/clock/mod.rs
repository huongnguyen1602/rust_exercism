use std::fmt::Display;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock{
    pub fn normalize(&mut self) {
        let summary_minutes = self.hours * 60 + self.minutes;
        self.hours = summary_minutes.div_euclid(60).rem_euclid(24);
        self.minutes = summary_minutes.rem_euclid(60);
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut instance = Clock{
            hours,
            minutes,
        };
        instance.normalize();
        instance
    }

    pub fn add_minutes(&self, minutes: i32) -> Self{
        let summary_minutes = self.hours * 60 + self.minutes + minutes;
        let instance = Clock{
            hours: summary_minutes.div_euclid(60).rem_euclid(24),
            minutes: summary_minutes.rem_euclid(60),
        };
        // instance.normalize();
        instance
    }
}

pub fn test() {
    assert_eq!(Clock::new(8, 0).to_string(), "08:00");
    assert_eq!(Clock::new(11, 9).to_string(), "11:09");
    assert_eq!(Clock::new(25, 0).to_string(), "01:00");
    assert_eq!(Clock::new(100, 0).to_string(), "04:00");
    assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
    assert_eq!(Clock::new(-1, 15).to_string(), "23:15");
    let clock = Clock::new(1, 1).add_minutes(3500);
    assert_eq!(clock.to_string(), "11:21");
    println!("it's ok");
}







// #[derive(Debug)]
// pub struct Clock{
//     hours: i32,
//     minutes: i32,
// }

// xử lí âm chưa tốt.
// impl Clock {
//     pub fn new(hours: i32, minutes: i32) -> Self {
//         let result_minutes = minutes.clone() % 60;
//         let mut result_hours = (minutes/60 + hours)%24;

//         Self { hours: result_hours, minutes: result_minutes}
//     }

//     pub fn add_minutes(&self, minutes: i32) -> Self {
//         let result_minutes = (self.minutes + minutes)%60;
//         let result_hours = (minutes/60 + self.hours)%24;
//         Self { hours: result_hours, minutes: result_minutes }
//     }
// }

// impl ToString for Clock {
//     fn to_string(&self) -> String {
//         let hours = if self.hours < 10{
//             "0".to_string()+&self.hours.to_string()
//         }else {
//             self.hours.to_string()
//         };

//         let minutes = if self.minutes < 10 {
//             "0".to_string()+&self.minutes.to_string()
//         }else {
//             self.minutes.to_string()
//         };
//         hours + &":".to_string() + &minutes
//     }
// }