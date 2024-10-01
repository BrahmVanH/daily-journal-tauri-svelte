#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

// use state::{ AppState, ServiceAccess };
use tauri::{ State, Manager, AppHandle };
use thiserror::Error;

use std::env::{ self, VarError };
use dotenvy::dotenv;

mod models;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Failed to get environment variable: {0}")] VarErr(VarError),
    #[error("io error: {0}")] IoErr(std::io::Error),
    #[error("Tauri application error: {0}")] TauriErr(tauri::Error),
    #[error("Failed to get app data directory")]
    AppDataDirNotFound,
}

fn main() {}
