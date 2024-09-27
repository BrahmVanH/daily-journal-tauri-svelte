use rusqlite::{ Connection, named_params, Result as SqliteResult, Error as SqliteError };
use tauri::AppHandle;
use std::fs;
use std::env;
use dotenvy::dotenv;
use chrono::Utc;
// use crate::models::{ NewJournalEntry, NewPersonalMetrics };

const CURRENT_DB_VERSION: u32 = 1;

/// Initializes the database connection, creating the .sqlite file if needed, and upgrading the database
/// if it's out of date.
pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let _ = dotenv();

    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");

    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");

    let sqlite_path = app_dir.join(env::var("DB_FILE_NAME").expect("No db file name foundy"));

    let mut db = Connection::open(sqlite_path)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| { Ok(row.get(0)?) })?;
    drop(user_pragma);

    upgrade_database_if_needed(&mut db, existing_user_version)?;

    Ok(db)
}

/// Upgrades the database to the current version.
pub fn upgrade_database_if_needed(
    db: &mut Connection,
    existing_version: u32
) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        db.pragma_update(None, "journal_mode", "WAL")?;

        let tx = db.transaction()?;

        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

        tx.execute_batch(
            "
      CREATE TABLE personal_metrics (
        journal_entry_id INTEGER PRIMARY KEY NOT NULL,
        financial INTEGER NOT NULL,
        fitness INTEGER NOT NULL,
        mental INTEGER NOT NULL,
        dietary INTEGER NOT NULL,
        social INTEGER NOT NULL,
        professional INTEGER NOT NULL
        FOREIGN KEY(journal_entry_id) REFERENCES journal_entry(id)
       );

      CREATE TABLE journal_entry (
        id INTEGER PRIMARY KEY NOT NULL,
        date TEXT NOT NULL,
        text TEXT NOT NULL
      );"
        )?;

        tx.commit()?;
    }

    Ok(())
}

pub fn add_item(title: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement: rusqlite::Statement<'_> = db.prepare(
        "INSERT INTO items (title) VALUES (@title)"
    )?;
    statement.execute(named_params! { "@title": title })?;

    Ok(())
}

pub fn get_all(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM items")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        let title: String = row.get("title")?;

        items.push(title);
    }

    Ok(items)
}

// #[tauri::command]
// pub fn add_entry(
//     personal_metrics: NewPersonalMetrics,
//     journal_entry: NewJournalEntry,
//     db: &Connection
// ) -> Result<(), SqliteError> {
//     // Trying out mapping all errors to strings without explicit handling within the function... we'll see what happens
//     let new_entry_id = add_journal_entry(journal_entry, db)?;

//     match add_personal_metric_entry(personal_metrics, &new_entry_id, &db) {
//         Ok(_) => Ok(()),
//         Err(e) => Err(e),
//     }
// }

// This will insert journal entry portion of "whole entry", returns id of new entry to use
// with personal metric entry insertion
// fn add_journal_entry(journal_entry: NewJournalEntry, db: &Connection) -> SqliteResult<String> {
//     // Create statement for inserting data into journal entry table, use ? to collect result
//     let mut stmt: rusqlite::Statement<'_> = db.prepare(
//         "INSERT INTO journal_entry (date, text) VALUES (@date, @text)"
//     )?;

//     // Execute statement, propagate error if present
//     stmt.execute(named_params! { "@date": Utc::now().to_string(), "@text": journal_entry.text })?;

//     let new_journal_entry_id = db.last_insert_rowid().to_string();

//     Ok(new_journal_entry_id)
// }

// This will insert the person metrics portion of the "whole entry" using the parameter new journal entry ID
// from recent journal entry insertion
// fn add_personal_metric_entry(
//     personal_metrics: NewPersonalMetrics,
//     journal_entry_id: &String,
//     db: &Connection
// ) -> SqliteResult<()> {
//     // Create statement for insertion of personal_metrics entry
//     let mut stmt: rusqlite::Statement<'_> = db.prepare(
//         "INSERT INTO personal_metrics (journal_entry_id, financial, fitness, mental, dietary, social, professional) VALUES (@journal_entry_id, @financial, @fitness, @mental, @dietary, @social, @professional)"
//     )?;

//     stmt.execute(
//         named_params! {
//     "@journal_entry_id": journal_entry_id, "@financial": personal_metrics.financial, "@fitness": personal_metrics.fitness, "@mental": personal_metrics.mental, "@dietary": personal_metrics.dietary, "@social": personal_metrics.social, "@professional": personal_metrics.professional

//     }
//     )?;

//     Ok(())
// }
