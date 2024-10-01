use std::env;
use dotenvy::dotenv;
use chrono::Utc;

use sqlx::{
    migrate::MigrateDatabase,
    migrate::Migrator,
    migrate::MigrateError,
    Sqlite,
    SqlitePool,
    FromRow,
    Row,
};
use tauri::App;

use crate::models::{ NewJournalEntry, NewPersonalMetrics };

use crate::{ ApplicationError, MigrateErr, VarErr };
use crate::SqlxErr;

#[tokio::main]
pub async fn initialize_database(db_url: &str) -> Result<(), ApplicationError> {
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating database {}", &db_url);
        match Sqlite::create_database(&db_url).await {
            Ok(_) => println!("Create db success"),
            Err(e) => {
                return Err(ApplicationError::SqlxErr(SqlxErr(e)));
            }
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
            return Err(ApplicationError::SqlxErr(SqlxErr(e)));
        }
    }
}

// Takes in CARGO_MANIFEST_DIR env and sources existing migrations folder to run contained migrations
pub async fn run_migrations(crate_dir: &str, db: SqlitePool) -> Result<(), ApplicationError> {
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results: Result<(), MigrateError> = match
        sqlx::migrate::Migrator::new(migrations).await.unwrap().run(&db).await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(ApplicationError::MigrationErr(MigrateErr(e)));
        }
    };

    println!("migration: {:?}", migration_results);
    Ok(())
}

#[tauri::command]
pub async fn add_entry(
    personal_metrics: NewPersonalMetrics,
    journal_entry: NewJournalEntry
) -> Result<String, ApplicationError> {
    let db_url = match std::env::var("DB_URL") {
        Ok(url) => url,
        Err(e) => {
            return Err(ApplicationError::VarErr(VarErr(e)));
        }
    };

    let db = match connect_db(&db_url).await {
        Ok(pool) => pool,
        Err(e) => {
            return Err(e);
        }
    };
    let new_entry_id = add_journal_entry(journal_entry, &db).await?;

    add_personal_metric_entry(personal_metrics, &new_entry_id, &db).await?;

    Ok(String::from("200"))
}

// This will insert journal entry portion of "whole entry", returns id of new entry to use
// with personal metric entry insertion
async fn add_journal_entry(
    journal_entry: NewJournalEntry,
    db: &SqlitePool
) -> Result<String, ApplicationError> {
    // Create statement for inserting data into journal entry table, use ? to collect result
    let result = match
        sqlx
            ::query("INSERT INTO journal_entry (date, text) VALUES (?)")
            .bind(Utc::now().to_string())
            .bind(journal_entry.text)
            .execute(db).await
    {
        Ok(r) => r,
        Err(e) => {
            return Err(ApplicationError::SqlxErr(SqlxErr(e)));
        }
    };

    let new_journal_entry_id = result.last_insert_rowid().to_string();

    Ok(new_journal_entry_id)
}

// This will insert the person metrics portion of the "whole entry" using the parameter new journal entry ID
// from recent journal entry insertion
async fn add_personal_metric_entry(
    personal_metrics: NewPersonalMetrics,
    journal_entry_id: &String,
    db: &SqlitePool
) -> Result<(), ApplicationError> {
    // Create and execute sql query on db
    let _result = match
        sqlx
            ::query(
                "INSERT INTO personal_metrics (journal_entry_id, financial, fitness, mental, dietary, social, professional) VALUES (?)"
            )
            .bind(journal_entry_id)
            .bind(personal_metrics.financial)
            .bind(personal_metrics.fitness)
            .bind(personal_metrics.mental)
            .bind(personal_metrics.dietary)
            .bind(personal_metrics.social)
            .bind(personal_metrics.professional)
            .execute(db).await
    {
        Ok(r) => r,
        Err(e) => {
            return Err(ApplicationError::SqlxErr(SqlxErr(e)));
        }
    };

    Ok(())
}
