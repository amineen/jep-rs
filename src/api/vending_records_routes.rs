use actix_web::{HttpResponse, Result, web};
use chrono::{DateTime, Utc};
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::model::VendingRecord;
use crate::repositories::{MongoDbVendingRecordRepository, VendingRecordRepository};

#[derive(Deserialize)]
pub struct DateRangeQuery {
    pub start_date: Option<String>, // ISO 8601 format: "2023-01-01T00:00:00Z"
    pub end_date: Option<String>,   // ISO 8601 format: "2023-12-31T23:59:59Z"
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

/// Get vending records with optional date range filtering
pub async fn get_vending_records(
    db: web::Data<Database>,
    query: web::Query<DateRangeQuery>,
) -> Result<HttpResponse> {
    // Parse date range or use defaults
    let start_date = match &query.start_date {
        Some(date_str) => date_str.parse::<DateTime<Utc>>().map_err(|_| {
            actix_web::error::ErrorBadRequest("Invalid start_date format. Use ISO 8601 format.")
        })?,
        None => Utc::now() - chrono::Duration::days(30), // Default: last 30 days
    };

    let end_date = match &query.end_date {
        Some(date_str) => date_str.parse::<DateTime<Utc>>().map_err(|_| {
            actix_web::error::ErrorBadRequest("Invalid end_date format. Use ISO 8601 format.")
        })?,
        None => Utc::now(), // Default: now
    };

    // Create repository
    let collection = db.collection::<VendingRecord>("vending_records");
    let repo = MongoDbVendingRecordRepository::from_collection(collection);

    // Get records
    match repo.get_vending_records(start_date, end_date).await {
        Ok(records) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: format!("Retrieved {} vending records", records.len()),
            data: Some(records),
        })),
        Err(e) => {
            eprintln!("Error fetching vending records: {}", e);
            eprintln!("Error details: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: "Failed to fetch vending records due to data format issues. Check server logs for details.".to_string(),
                data: None,
            }))
        }
    }
}

/// Configure vending records routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/vending-records").route("", web::get().to(get_vending_records)));
}
