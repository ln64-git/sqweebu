// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// region: --- imports
use crate::ws::start_websocket_server;
use _core::playback::{init_playback_channel, PlaybackCommand};
use _core::{process_input, AppState};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;
use tauri::api::path::data_dir;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::{CustomMenuItem, SystemTrayMenu};
use tokio::sync::{broadcast, Mutex};
use tokio::task;
// endregion: --- imports

pub mod ws;

#[tokio::main]
async fn main() {
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    let data_dir = data_dir().unwrap(); // This assumes the operation won't fail
    let db_path = data_dir.join("database");

    let db: Surreal<surrealdb::engine::local::Db> =
        Surreal::new::<RocksDb>(db_path.to_str().unwrap())
            .await
            .unwrap();
    let _ = db.use_ns("test").use_db("test").await;

    let db_clone = db.clone(); // Clone the db value
    let db_lock: Arc<Mutex<Surreal<surrealdb::engine::local::Db>>> = Arc::new(Mutex::new(db_clone)); // Use the cloned value

    tauri::async_runtime::spawn(start_websocket_server(db_lock.clone()));

    let playback_send = init_playback_channel().await;
    let nexus = Arc::new(Mutex::new(AppState {
        playback_send: playback_send.clone(),
        db,
    }));

    let (sender, _) = broadcast::channel(100);
    tokio::spawn(async move {
        loop {
            // Broadcast database changes to WebSocket clients
            let _ = broadcast_chat_entries(db_lock.clone(), sender.clone()).await;
            // Adjust the sleep duration based on your requirements
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| handle_system_tray_event(app, event))
        .invoke_handler(tauri::generate_handler![
            pause_playback_from_frontend,
            resume_playback_from_frontend,
            stop_playback_from_frontend,
            fast_forward_playback_from_frontend,
            process_input_from_frontend,
        ])
        .manage(nexus.clone())
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

async fn broadcast_chat_entries(
    db_lock: Arc<Mutex<Surreal<surrealdb::engine::local::Db>>>,
    sender: broadcast::Sender<String>,
) -> Result<(), String> {
    let db = db_lock.lock().await;
    // Simulated data for demonstration
    let chat_entries: Option<Vec<ChatEntry>> = match db.select("chat").await {
        Ok(entries) => Some(entries),
        Err(error) => return Err(format!("Database error: {:?}", error)),
    };
    // Serialize the data
    let serialized_data = serde_json::to_string(&chat_entries).map_err(|e| e.to_string())?;
    println!("{:#?}", serialized_data);
    // Broadcast the serialized data to connected WebSocket clients
    sender.send(serialized_data).unwrap();
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatEntry {
    timestamp: DateTime<Utc>,
    body: String,
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
    // let playback_send = nexus.playback_send;
    let db = nexus.db;
    // task::spawn(async move {
    let _ = process_input(&text, db).await;
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
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
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
        },
        _ => {}
    }
}
