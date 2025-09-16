use mongodb::{Client, Database};
use std::env;

pub struct DatabaseConnection {
    pub client: Client,
    pub database: Database,
}

impl DatabaseConnection {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Get MongoDB connection string from environment variable
        let connection_string = env::var("MONGODB_URI").map_err(
            |_| "MONGODB_URI env variable is required. Please set it in your .env file.",
        )?;

        // Get database name from environment variable
        let database_name = env::var("DATABASE_NAME").map_err(
            |_| "DATABASE_NAME env variable is required. Please set it in your .env file.",
        )?;

        // Create MongoDB client
        let client = Client::with_uri_str(&connection_string).await?;

        // Test the connection by pinging the database
        client
            .database("admin")
            .run_command(mongodb::bson::doc! {"ping": 1})
            .await?;

        println!("✅ Successfully connected to MongoDB");

        let database = client.database(&database_name);

        Ok(DatabaseConnection { client, database })
    }
}
