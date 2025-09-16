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
