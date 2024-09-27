// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;

use state::{ AppState, ServiceAccess };
use tauri::{ State, Manager, AppHandle };

// mod models;

// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(app_handle: AppHandle, name: &str) -> String {
//     // Should handle errors instead of unwrapping here
//     app_handle.db(|db| database::add_item(name, db)).unwrap();

//     let items = app_handle.db(|db| database::get_all(db)).unwrap();

//     let items_string = items.join(" | ");

//     format!("Your name log: {}", items_string)
// }

// #[tauri::command]
// fn get_all(app_handle: AppHandle) -> String {
//     let items = app_handle.db(|db| database::get_all(db)).unwrap();

//     items.join(" | ")

// }

fn main() {
    //     #[cfg(debug_assertions)]
    let devtools = devtools::init();

    //     #[cfg(debug_assertions)]
    let builder = tauri::Builder
        ::default()
        .plugin(devtools)
        .manage(AppState { db: Default::default() })
        // .invoke_handler(tauri::generate_handler![greet])
        // .invoke_handler(tauri::generate_handler![get_all])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = database
                ::initialize_database(&handle)
                .expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        });
    //      #[cfg(not(debug_assertions))]

    //         let builder = tauri::Builder::default()
    //         .manage(AppState { db: Default::default() })
    //         .invoke_handler(tauri::generate_handler![greet])
    //         .invoke_handler(tauri::generate_handler![get_all])
    //         .setup(|app| {
    //             let handle = app.handle();

    //             let app_state: State<AppState> = handle.state();
    //             let db = database::initialize_database(&handle).expect("Database initialize should succeed");
    //             *app_state.db.lock().unwrap() = Some(db);

    //             Ok(())
    //         });

    let _ = builder.run(tauri::generate_context!());
}
