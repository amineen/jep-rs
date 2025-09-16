use crate::model::{DailySummary, VendingRecord, VendingStationSummary, VendingSummary};
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

    async fn get_vending_summary(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<VendingSummary, Box<dyn Error>> {
        // Convert chrono DateTime to MongoDB DateTime for filtering
        let start_bson = mongodb::bson::DateTime::from_millis(start_date.timestamp_millis());
        let end_bson = mongodb::bson::DateTime::from_millis(end_date.timestamp_millis());

        // Build aggregation pipeline
        let pipeline = vec![
            // Match documents within date range
            doc! {
                "$match": {
                    "timestamp": {
                        "$gte": start_bson,
                        "$lte": end_bson
                    }
                }
            },
            // Add computed fields for date processing
            doc! {
                "$addFields": {
                    "date": {
                        "$dateToString": {
                            "format": "%Y-%m-%d",
                            "date": "$timestamp"
                        }
                    },
                    "safeAmount": { "$ifNull": ["$amount", 0.0] },
                    "safeKwh": { "$ifNull": ["$kwh", 0.0] },
                    "safeVendingStation": { "$ifNull": ["$vendingStation", "Unknown"] }
                }
            },
            // Group by vending station and date for daily summaries
            doc! {
                "$group": {
                    "_id": {
                        "vendingStation": "$safeVendingStation",
                        "date": "$date"
                    },
                    "dailyTransactions": { "$sum": 1 },
                    "dailyAmount": { "$sum": "$safeAmount" },
                    "dailyKwh": { "$sum": "$safeKwh" }
                }
            },
            // Group by vending station to create station summaries
            doc! {
                "$group": {
                    "_id": "$_id.vendingStation",
                    "totalTransactions": { "$sum": "$dailyTransactions" },
                    "totalAmount": { "$sum": "$dailyAmount" },
                    "totalKwh": { "$sum": "$dailyKwh" },
                    "dailySummaries": {
                        "$push": {
                            "date": "$_id.date",
                            "total_transactions": "$dailyTransactions",
                            "total_amount": "$dailyAmount",
                            "total_kwh": "$dailyKwh"
                        }
                    }
                }
            },
            // Sort daily summaries by date
            doc! {
                "$addFields": {
                    "dailySummaries": {
                        "$sortArray": {
                            "input": "$dailySummaries",
                            "sortBy": { "date": 1 }
                        }
                    }
                }
            },
            // Group all stations together for final summary
            doc! {
                "$group": {
                    "_id": null,
                    "grandTotalTransactions": { "$sum": "$totalTransactions" },
                    "grandTotalAmount": { "$sum": "$totalAmount" },
                    "grandTotalKwh": { "$sum": "$totalKwh" },
                    "stationSummaries": {
                        "$push": {
                            "vending_station": "$_id",
                            "total_transactions": "$totalTransactions",
                            "total_amount": "$totalAmount",
                            "total_kwh": "$totalKwh",
                            "daily_summaries": "$dailySummaries"
                        }
                    }
                }
            },
        ];

        // Execute aggregation
        let mut cursor = self.collection.aggregate(pipeline).await?;

        use futures_util::stream::StreamExt;
        if let Some(result) = cursor.next().await {
            let doc = result?;

            // Extract values from the aggregation result
            let grand_total_transactions =
                doc.get_i32("grandTotalTransactions").unwrap_or(0) as u32;
            let grand_total_amount = doc.get_f64("grandTotalAmount").unwrap_or(0.0);
            let grand_total_kwh = doc.get_f64("grandTotalKwh").unwrap_or(0.0);

            // Parse station summaries
            let mut station_summaries = Vec::new();
            if let Ok(stations_array) = doc.get_array("stationSummaries") {
                for station_doc in stations_array {
                    if let mongodb::bson::Bson::Document(station) = station_doc {
                        let vending_station = station
                            .get_str("vending_station")
                            .unwrap_or("Unknown")
                            .to_string();
                        let total_transactions =
                            station.get_i32("total_transactions").unwrap_or(0) as u32;
                        let total_amount = station.get_f64("total_amount").unwrap_or(0.0);
                        let total_kwh = station.get_f64("total_kwh").unwrap_or(0.0);

                        // Parse daily summaries
                        let mut daily_summaries = Vec::new();
                        if let Ok(daily_array) = station.get_array("daily_summaries") {
                            for daily_doc in daily_array {
                                if let mongodb::bson::Bson::Document(daily) = daily_doc {
                                    let daily_summary = DailySummary {
                                        date: daily.get_str("date").unwrap_or("").to_string(),
                                        total_transactions: daily
                                            .get_i32("total_transactions")
                                            .unwrap_or(0)
                                            as u32,
                                        total_amount: daily.get_f64("total_amount").unwrap_or(0.0),
                                        total_kwh: daily.get_f64("total_kwh").unwrap_or(0.0),
                                    };
                                    daily_summaries.push(daily_summary);
                                }
                            }
                        }

                        let station_summary = VendingStationSummary {
                            vending_station,
                            total_transactions,
                            total_amount,
                            total_kwh,
                            period_start: start_date.format("%Y-%m-%d").to_string(),
                            period_end: end_date.format("%Y-%m-%d").to_string(),
                            daily_summaries,
                        };
                        station_summaries.push(station_summary);
                    }
                }
            }

            // Sort station summaries by vending station name
            station_summaries.sort_by(|a, b| a.vending_station.cmp(&b.vending_station));

            let summary = VendingSummary {
                total_transactions: grand_total_transactions,
                total_amount: grand_total_amount,
                total_kwh: grand_total_kwh,
                period_start: start_date.format("%Y-%m-%d").to_string(),
                period_end: end_date.format("%Y-%m-%d").to_string(),
                vending_station_summaries: station_summaries,
            };

            Ok(summary)
        } else {
            // No data found, return empty summary
            Ok(VendingSummary {
                total_transactions: 0,
                total_amount: 0.0,
                total_kwh: 0.0,
                period_start: start_date.format("%Y-%m-%d").to_string(),
                period_end: end_date.format("%Y-%m-%d").to_string(),
                vending_station_summaries: Vec::new(),
            })
        }
    }
}
