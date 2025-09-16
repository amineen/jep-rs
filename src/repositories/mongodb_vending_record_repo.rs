use crate::model::VendingRecord;
use crate::repositories::VendingRecordRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mongodb::{Collection, bson::doc};
use std::error::Error;

pub struct MongoDbVendingRecordRepository {
    collection: Collection<VendingRecord>,
}

impl MongoDbVendingRecordRepository {
    pub fn from_collection(collection: Collection<VendingRecord>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl VendingRecordRepository for MongoDbVendingRecordRepository {
    async fn get_vending_records(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<VendingRecord>, Box<dyn Error>> {
        // Convert chrono DateTime to MongoDB DateTime
        let start_bson = mongodb::bson::DateTime::from_millis(start_date.timestamp_millis());
        let end_bson = mongodb::bson::DateTime::from_millis(end_date.timestamp_millis());

        // Create filter for date range
        let filter = doc! {
            "timestamp": {
                "$gte": start_bson,
                "$lte": end_bson
            }
        };

        // Execute query
        let mut cursor = self.collection.find(filter).await?;
        let mut records = Vec::new();

        // Collect results
        use futures_util::stream::StreamExt;
        while let Some(result) = cursor.next().await {
            match result {
                Ok(record) => records.push(record),
                Err(e) => return Err(Box::new(e)),
            }
        }

        Ok(records)
    }
}
