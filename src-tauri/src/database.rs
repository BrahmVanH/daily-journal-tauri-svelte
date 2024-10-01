use std::env;
use dotenvy::dotenv;

use sqlx::{ migrate::MigrateDatabase, Sqlite, SqlitePool, FromRow, Row };

use crate::ApplicationError;

#[tokio::main]
pub async fn initialize_database(db_url: &str) -> Result<(), ApplicationError> {
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating database {}", &db_url);
        match Sqlite::create_database(&db_url).await {
            Ok(_) => println!("Create db success"),
            Err(e) => panic!("error: {}", e),
        }
    } else {
        println!("Database already exists");
    }

    Ok(())
}

pub async fn connect_db(db_url: &str) -> Result<SqlitePool, ApplicationError> {
    match SqlitePool::connect(&db_url).await {
        Ok(db) => Ok(db),
        Err(e) => {
            return Err(ApplicationError::SqlxErr(e));
        }
    }
}
