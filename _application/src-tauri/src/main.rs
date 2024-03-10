// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// region: --- imports

use _core::playback::{ init_playback_channel, PlaybackCommand };
use _core::{ process_input, AppState };
use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;
use tauri::api::path::data_dir;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::{ CustomMenuItem, SystemTrayMenu };

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
// endregion: --- imports

#[tokio::main]
async fn main() {
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(show).add_item(hide).add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    // Create a RocksDB database connection
    let data_dir = data_dir().unwrap(); // This assumes the operation won't fail
    println!("Data directory: {:?}", data_dir);
    let db_path = data_dir.join("database");
    println!("Database path: {:?}", db_path);
    let db: Surreal<surrealdb::engine::local::Db> = Surreal::new::<RocksDb>(
        db_path.to_str().unwrap()
    ).await.unwrap();
    println!("Database connection established");

    let playback_send = init_playback_channel().await;

    let nexus = Arc::new(
        Mutex::new(AppState {
            playback_send: playback_send.clone(),
            db,
        })
    );

    tauri::Builder
        ::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| handle_system_tray_event(app, event))
        .invoke_handler(
            tauri::generate_handler![
                pause_playback_from_frontend,
                resume_playback_from_frontend,
                stop_playback_from_frontend,
                fast_forward_playback_from_frontend,
                process_input_from_frontend
            ]
        )
        .manage(nexus.clone())
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            match event {
                tauri::RunEvent::ExitRequested { api, .. } => {
                    api.prevent_exit();
                }
                _ => {}
            }
        });
}

async fn get_nexus(app: tauri::AppHandle) -> AppState {
    let nexus_lock = app.state::<Arc<Mutex<AppState>>>();
    let nexus = nexus_lock.lock().await;
    nexus.clone()
}

// region: --- Main Commands

#[tauri::command]
async fn process_input_from_frontend(text: String, app: tauri::AppHandle) -> Result<(), String> {
    let nexus = get_nexus(app).await;
    let playback_send = nexus.playback_send;
    let db = nexus.db;
    // task::spawn(async move {
    let _ = process_input(&text, &playback_send, db).await;
    // });
    Ok(())
}

// endregion: --- Main Commands

// region: --- Playback Commands

#[tauri::command]
async fn pause_playback_from_frontend(app: tauri::AppHandle) -> Result<(), String> {
    let playback_send = {
        let nexus_lock = app.state::<Arc<Mutex<AppState>>>();
        let nexus = nexus_lock.lock().await;
        nexus.playback_send.clone()
    };
    task::spawn(async move { playback_send.send(PlaybackCommand::Pause).await });
    Ok(())
}

#[tauri::command]
async fn resume_playback_from_frontend(app: tauri::AppHandle) -> Result<(), String> {
    let playback_send = {
        let nexus_lock = app.state::<Arc<Mutex<AppState>>>();
        let nexus = nexus_lock.lock().await;
        nexus.playback_send.clone()
    };
    task::spawn(async move { playback_send.send(PlaybackCommand::Resume).await });
    Ok(())
}

#[tauri::command]
async fn stop_playback_from_frontend(app: tauri::AppHandle) -> Result<(), String> {
    let playback_send = {
        let nexus_lock = app.state::<Arc<Mutex<AppState>>>();
        let nexus = nexus_lock.lock().await;
        nexus.playback_send.clone()
    };
    task::spawn(async move { playback_send.send(PlaybackCommand::Stop).await });
    Ok(())
}

#[tauri::command]
async fn fast_forward_playback_from_frontend(app: tauri::AppHandle) -> Result<(), String> {
    let playback_send = {
        let nexus_lock = app.state::<Arc<Mutex<AppState>>>();
        let nexus = nexus_lock.lock().await;
        nexus.playback_send.clone()
    };
    task::spawn(async move { playback_send.send(PlaybackCommand::FastForward).await });
    Ok(())
}

// endregion: --- Playback Commands

fn handle_system_tray_event(app: &tauri::AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { position: _, size: _, .. } => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick { position: _, size: _, .. } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } =>
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                _ => {}
            }
        _ => {}
    }
}
