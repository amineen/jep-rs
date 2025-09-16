use crate::model::VendingRecord;
use crate::repositories::VendingRecordRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mongodb::Client;
use std::error::Error;

pub struct MongoDbVendingRecordRepository {
    client: Client,
}

impl MongoDbVendingRecordRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl VendingRecordRepository for MongoDbVendingRecordRepository {
    async fn get_vending_records(
        &self,
        _start_date: DateTime<Utc>,
        _end_date: DateTime<Utc>,
    ) -> Result<Vec<VendingRecord>, Box<dyn Error>> {
        // TODO: Implement actual MongoDB query
        // For now, return empty vector to make it compile
        Ok(Vec::new())
    }
}
