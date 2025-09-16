use actix_web::{HttpResponse, Result, web};
use mongodb::Client;
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
pub async fn readiness_check(client: web::Data<Client>) -> Result<HttpResponse> {
    // Check actual database connectivity
    let db_health = match check_database_health(&client).await {
        Ok(is_connected) => DatabaseHealth {
            status: if is_connected {
                "UP".to_string()
            } else {
                "DOWN".to_string()
            },
            connected: is_connected,
        },
        Err(_) => DatabaseHealth {
            status: "DOWN".to_string(),
            connected: false,
        },
    };

    let is_ready = db_health.connected;
    let overall_status = if is_ready { "READY" } else { "NOT_READY" };

    let response = ReadinessResponse {
        status: overall_status.to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        service: "jep-rs".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_health,
    };

    if is_ready {
        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::ServiceUnavailable().json(response))
    }
}

/// Helper function to check database health
async fn check_database_health(client: &Client) -> Result<bool, Box<dyn std::error::Error>> {
    match client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1})
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => {
            eprintln!("Database health check failed: {}", e);
            Ok(false)
        }
    }
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
