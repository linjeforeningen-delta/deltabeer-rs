use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};

#[async_trait]
pub trait Clock {
    fn now(&self) -> DateTime<Utc>;

    fn today(&self) -> NaiveDate;
}
