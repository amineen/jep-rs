mod api;
mod model;
mod repositories;

use actix_web::{App, HttpServer, middleware::Logger};
use api::configure_health_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("🚀 Starting JEP-RS API Server...");
    println!("📍 Server will be available at: http://127.0.0.1:8092");
    println!("🏥 Health check endpoint: http://127.0.0.1:8092/health");
    println!("🔍 Ready check endpoint: http://127.0.0.1:8092/health/ready");

    HttpServer::new(|| {
        App::new()
            // Add logging middleware
            .wrap(Logger::default())
            // Configure health routes
            .configure(configure_health_routes)
    })
    .bind("127.0.0.1:8092")?
    .run()
    .await
}
