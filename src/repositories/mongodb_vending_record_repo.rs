use crate::model::VendingRecord;
use crate::repositories::VendingRecordRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mongodb::{Collection, bson::doc};
use serde::{Deserialize, Serialize};
use std::error::Error;

// Internal struct for MongoDB operations with BSON DateTime
#[derive(Debug, Serialize, Deserialize)]
struct MongoVendingRecord {
    #[serde(rename = "_id")]
    pub id: String,
    pub timestamp: mongodb::bson::DateTime,
    #[serde(rename = "meterNumber")]
    pub meter_number: String,
    pub address: String,
    pub community: String,
    #[serde(rename = "customerName")]
    pub customer_name: String,
    pub token: String,
    #[serde(default)]
    pub tariff: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub kwh: Option<f64>,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "vendingStation")]
    pub vending_station: String,
    #[serde(rename = "fixedCharge", default)]
    pub fixed_charge: Option<f64>,
    #[serde(rename = "transactionId", default)]
    pub transaction_id: Option<String>,
    #[serde(rename = "remainingCredit", default)]
    pub remaining_credit: Option<f64>,
}

impl From<MongoVendingRecord> for VendingRecord {
    fn from(mongo_record: MongoVendingRecord) -> Self {
        VendingRecord {
            id: mongo_record.id,
            timestamp: mongo_record.timestamp.to_chrono(),
            meter_number: Some(mongo_record.meter_number),
            address: Some(mongo_record.address),
            community: Some(mongo_record.community),
            customer_name: Some(mongo_record.customer_name),
            token: Some(mongo_record.token),
            tariff: mongo_record.tariff,
            amount: mongo_record.amount,
            kwh: mongo_record.kwh,
            user_id: Some(mongo_record.user_id),
            vending_station: Some(mongo_record.vending_station),
            fixed_charge: mongo_record.fixed_charge,
            transaction_id: mongo_record.transaction_id,
            remaining_credit: mongo_record.remaining_credit,
        }
    }
}

pub struct MongoDbVendingRecordRepository {
    collection: Collection<MongoVendingRecord>,
}

impl MongoDbVendingRecordRepository {
    pub fn from_collection(collection: Collection<VendingRecord>) -> Self {
        // Convert the collection type to work with our internal MongoVendingRecord
        let mongo_collection = collection.clone_with_type::<MongoVendingRecord>();
        Self {
            collection: mongo_collection,
        }
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

        // Collect results and convert to API format
        use futures_util::stream::StreamExt;
        while let Some(result) = cursor.next().await {
            match result {
                Ok(mongo_record) => {
                    let api_record = VendingRecord::from(mongo_record);
                    records.push(api_record);
                }
                Err(e) => return Err(Box::new(e)),
            }
        }

        Ok(records)
    }
}
