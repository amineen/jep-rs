use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VendingRecord {
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
    pub tariff: f64,
    pub amount: f64,
    pub kwh: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "vendingStation")]
    pub vending_station: String,

    #[serde(rename = "fixedCharge")]
    pub fixed_charge: f64,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "remainingCredit")]
    pub remaining_credit: f64,
}
