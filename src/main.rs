mod api;
mod database;
mod model;
mod repositories;

use actix_web::{App, HttpServer, middleware::Logger, web};
use api::{configure_health_routes, configure_vending_routes};
use database::DatabaseConnection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    if let Err(e) = dotenvy::dotenv() {
        println!("⚠️  Warning: Could not load .env file: {}", e);
        println!(
            "💡 Make sure you have a .env file in the project root with MONGODB_URI and DATABASE_NAME"
        );
    } else {
        println!("✅ Loaded environment variables from .env file");
    }

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initialize database connection
    let db_connection = match DatabaseConnection::new().await {
        Ok(connection) => {
            println!("✅ Database connection established");
            connection
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            eprintln!("💡 Make sure MongoDB is running or set MONGODB_URI environment variable");
            std::process::exit(1);
        }
    };

    println!("🚀 Starting JEP-RS API Server...");
    println!("📍 Server will be available at: http://127.0.0.1:8092");
    println!("🏥 Health check endpoint: http://127.0.0.1:8092/health");
    println!("🔍 Ready check endpoint: http://127.0.0.1:8092/health/ready");
    println!("📊 Vending records API: http://127.0.0.1:8092/api/vending-records");

    HttpServer::new(move || {
        App::new()
            // Add database connection to app data
            .app_data(web::Data::new(db_connection.client.clone()))
            .app_data(web::Data::new(db_connection.database.clone()))
            // Add logging middleware
            .wrap(Logger::default())
            // Configure health routes
            .configure(configure_health_routes)
            // Configure vending records routes
            .configure(configure_vending_routes)
    })
    .bind("127.0.0.1:8092")?
    .run()
    .await
}
