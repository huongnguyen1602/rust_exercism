use time::PrimitiveDateTime as DateTime;
use time::{Duration, Date, Time};
// const GIGA_SECOND: Duration = Duration::new(1_000_000_000,0);
const GIGA_SECOND: Duration = Duration::seconds(1_000_000_000);


fn after(start: DateTime) -> DateTime {
    start + GIGA_SECOND
}

fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) ->DateTime{
    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}

pub fn test() {
    assert_eq!(after(dt(2011, 4, 25, 0, 0, 0)), dt(2043, 1, 1, 1, 46, 40));
    assert_eq!(after(dt(1977, 6, 13, 0, 0, 0)), dt(2009, 2, 19, 1, 46, 40));
    println!("it is ok");
}


