#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use database::initialize_database;
use serde::{ Serialize, Serializer };
// use state::{ AppState, ServiceAccess };
use tauri::{ State, Manager, AppHandle };
use thiserror::Error;

use std::{ io, env::{ self, VarError } };
use dotenvy::dotenv;

mod models;
mod database;

#[derive(Debug)]
pub struct IoErr(io::Error);

// Manual implementation of Serialize for IoErrorWrapper
impl Serialize for IoErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Convert io::Error to a string representation
        serializer.serialize_str(&self.0.to_string())
    }
}

#[derive(Debug)]
pub struct VarErr(VarError);

// Manual implementation of Serialize for IoErrorWrapper
impl Serialize for VarErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Convert io::Error to a string representation
        serializer.serialize_str(&self.0.to_string())
    }
}

#[derive(Debug)]
pub struct TauriErr(tauri::Error);

// Manual implementation of Serialize for IoErrorWrapper
impl Serialize for TauriErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Convert io::Error to a string representation
        serializer.serialize_str(&self.0.to_string())
    }
}

#[derive(Debug)]
pub struct SqlxErr(sqlx::Error);

// Manual implementation of Serialize for IoErrorWrapper
impl Serialize for SqlxErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Convert io::Error to a string representation
        serializer.serialize_str(&self.0.to_string())
    }
}

#[derive(Debug)]
pub struct MigrateErr(sqlx::migrate::MigrateError);

// Manual implementation of Serialize for IoErrorWrapper
impl Serialize for MigrateErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Convert io::Error to a string representation
        serializer.serialize_str(&self.0.to_string())
    }
}

#[derive(Error, Debug, Serialize)]
pub enum ApplicationError {
    #[error("Failed to get environment variable:")] VarErr(VarErr),
    #[error("io error:")] IoErr(IoErr),
    #[error("Tauri application error: ")] TauriErr(TauriErr),
    #[error("sqlx error: ")] SqlxErr(SqlxErr),
    #[error("migration error: ")] MigrationErr(MigrateErr),

    #[error("Failed to get app data directory")]
    AppDataDirNotFound,
}
fn main() -> Result<(), ApplicationError> {
    let _ = dotenv();

    match
        tauri::Builder
            ::default()
            .invoke_handler(tauri::generate_handler![database::add_entry])
            .invoke_handler(tauri::generate_handler![database::run_migrations])
            .setup(|_app| {
                let db_url = match env::var("DB_URL") {
                    Ok(url) => url,
                    Err(e) => {
                        return Err(Box::new(ApplicationError::VarErr(VarErr(e))) as Box<dyn std::error::Error>);
                    }
                };
                tauri::async_runtime::block_on(async {
                    match initialize_database(&db_url).await {
                        Ok(_) => Ok(()),
                        Err(e) => {
                            return Err(Box::new(e) as Box<dyn std::error::Error>);
                        }
                    }
                })
            })
            .run(tauri::generate_context!())
    {
        Ok(_) => (),
        Err(e) => {
            return Err(ApplicationError::TauriErr(TauriErr(e)));
        }
    }

    Ok(())
}
