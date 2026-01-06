use crate::ports::Clock;
use chrono::NaiveDate;

pub struct SystemClock;
impl Clock for SystemClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }

    fn today(&self) -> NaiveDate {
        chrono::Utc::now().date_naive()
    }
}
