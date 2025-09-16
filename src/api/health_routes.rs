use actix_web::{HttpResponse, Result, web};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: u64,
    pub service: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct ReadinessResponse {
    pub status: String,
    pub timestamp: u64,
    pub service: String,
    pub version: String,
    pub database: DatabaseHealth,
}

#[derive(Serialize)]
pub struct DatabaseHealth {
    pub status: String,
    pub connected: bool,
}

/// Basic liveness check - returns 200 if the service is running
pub async fn health_check() -> Result<HttpResponse> {
    let response = HealthResponse {
        status: "UP".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        service: "jep-rs".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Readiness check - returns 200 if the service is ready to handle requests
/// This includes checking database connectivity
pub async fn readiness_check() -> Result<HttpResponse> {
    // TODO: Add actual database connection check
    // For now, we'll assume the database is healthy
    let db_health = DatabaseHealth {
        status: "UP".to_string(),
        connected: true, // This should be checked against actual MongoDB connection
    };

    let response = ReadinessResponse {
        status: "READY".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        service: "jep-rs".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_health,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Configure health routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(health_check))
            .route("/live", web::get().to(health_check))
            .route("/ready", web::get().to(readiness_check)),
    );
}
