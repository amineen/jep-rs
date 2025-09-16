use actix_web::{HttpResponse, Result, web};
use chrono::{DateTime, NaiveDate, Utc};
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

/// Parse flexible date formats (YYYY-MM-DD or ISO 8601 datetime)
fn parse_flexible_date(date_str: &str, is_end_date: bool) -> Result<DateTime<Utc>, String> {
    // Try parsing as full ISO 8601 datetime first
    if let Ok(datetime) = date_str.parse::<DateTime<Utc>>() {
        return Ok(datetime);
    }

    // Try parsing as date-only format (YYYY-MM-DD)
    if let Ok(naive_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        if is_end_date {
            // For end dates, set time to 23:59:59.999 to include the entire day
            let naive_datetime = naive_date
                .and_hms_milli_opt(23, 59, 59, 999)
                .ok_or("Invalid time")?;
            return Ok(DateTime::from_naive_utc_and_offset(naive_datetime, Utc));
        } else {
            // For start dates, set time to 00:00:00
            let naive_datetime = naive_date.and_hms_opt(0, 0, 0).ok_or("Invalid time")?;
            return Ok(DateTime::from_naive_utc_and_offset(naive_datetime, Utc));
        }
    }

    Err(format!(
        "Invalid date format: '{}'. Use YYYY-MM-DD or ISO 8601 format (YYYY-MM-DDTHH:MM:SSZ)",
        date_str
    ))
}

/// Get vending records with optional date range filtering
pub async fn get_vending_records(
    db: web::Data<Database>,
    query: web::Query<DateRangeQuery>,
) -> Result<HttpResponse> {
    // Parse date range or use defaults
    let start_date = match &query.start_date {
        Some(date_str) => parse_flexible_date(date_str, false)
            .map_err(|e| actix_web::error::ErrorBadRequest(e))?,
        None => Utc::now() - chrono::Duration::days(30), // Default: last 30 days
    };

    let end_date = match &query.end_date {
        Some(date_str) => {
            parse_flexible_date(date_str, true).map_err(|e| actix_web::error::ErrorBadRequest(e))?
        }
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

/// Get vending summary with aggregated statistics
pub async fn get_vending_summary(
    db: web::Data<Database>,
    query: web::Query<DateRangeQuery>,
) -> Result<HttpResponse> {
    // Parse date range or use defaults
    let start_date = match &query.start_date {
        Some(date_str) => parse_flexible_date(date_str, false)
            .map_err(|e| actix_web::error::ErrorBadRequest(e))?,
        None => Utc::now() - chrono::Duration::days(30), // Default: last 30 days
    };

    let end_date = match &query.end_date {
        Some(date_str) => {
            parse_flexible_date(date_str, true).map_err(|e| actix_web::error::ErrorBadRequest(e))?
        }
        None => Utc::now(), // Default: now
    };

    // Create repository
    let collection = db.collection::<VendingRecord>("vending_records");
    let repo = MongoDbVendingRecordRepository::from_collection(collection);

    // Get summary
    match repo.get_vending_summary(start_date, end_date).await {
        Ok(summary) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: format!(
                "Retrieved vending summary for period {} to {} ({} total transactions)",
                summary.period_start, summary.period_end, summary.total_transactions
            ),
            data: Some(summary),
        })),
        Err(e) => {
            eprintln!("Error fetching vending summary: {}", e);
            eprintln!("Error details: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: "Failed to fetch vending summary due to data processing issues. Check server logs for details.".to_string(),
                data: None,
            }))
        }
    }
}

/// Configure vending records routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/vending-records")
            .route("", web::get().to(get_vending_records))
            .route("/summary", web::get().to(get_vending_summary)),
    );
}
