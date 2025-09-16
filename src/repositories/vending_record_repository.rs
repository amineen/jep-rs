use crate::model::VendingRecord;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::error::Error;

#[async_trait]
pub trait VendingRecordRepository: Send + Sync {
    //Get all vending records by date range
    async fn get_vending_records(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<VendingRecord>, Box<dyn Error>>;
}
