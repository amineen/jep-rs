use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VendingRecord {
    #[serde(rename = "_id")]
    pub id: String,
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "meterNumber", default)]
    pub meter_number: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub community: Option<String>,
    #[serde(rename = "customerName", default)]
    pub customer_name: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub tariff: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub kwh: Option<f64>,
    #[serde(rename = "userId", default)]
    pub user_id: Option<String>,
    #[serde(rename = "vendingStation", default)]
    pub vending_station: Option<String>,

    #[serde(rename = "fixedCharge", default)]
    pub fixed_charge: Option<f64>,
    #[serde(rename = "transactionId", default)]
    pub transaction_id: Option<String>,
    #[serde(rename = "remainingCredit", default)]
    pub remaining_credit: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DailySummary {
    pub date: String, //YYYY-MM-DD
    pub total_transactions: u32,
    pub total_amount: f64,
    pub total_kwh: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VendingStationSummary {
    pub vending_station: String,
    pub total_transactions: u32,
    pub total_amount: f64,
    pub total_kwh: f64,
    pub period_start: String, //YYYY-MM-DD
    pub period_end: String,   //YYYY-MM-DD
    pub daily_summaries: Vec<DailySummary>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VendingSummary {
    pub total_transactions: u32,
    pub total_amount: f64,
    pub total_kwh: f64,
    pub period_start: String, //YYYY-MM-DD
    pub period_end: String,   //YYYY-MM-DD
    pub vending_station_summaries: Vec<VendingStationSummary>,
}
