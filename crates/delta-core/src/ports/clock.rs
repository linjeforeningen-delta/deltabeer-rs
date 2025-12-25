use chrono::{DateTime, NaiveDate, Utc};

pub trait Clock {
    fn now(&self) -> DateTime<Utc>;

    fn today(&self) -> NaiveDate;
}
