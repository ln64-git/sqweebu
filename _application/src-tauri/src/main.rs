// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use _core::io::{process_input, ChatEntry};
// region: --- imports
use _core::playback::{init_playback_channel, PlaybackCommand};
use _core::utils::{check_empty_sink, listen_audio_database, AudioEntry};
use _core::AppState;
use std::sync::Arc;
use surrealdb::engine::local::{Mem, RocksDb};
use surrealdb::Surreal;
use tauri::api::path::data_dir;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::{CustomMenuItem, SystemTrayMenu};
use tokio::sync::{mpsc, Mutex};
use tokio::task;
// endregion: --- imports

#[tokio::main]
async fn main() {
    // region: --- System Tray

    let show = CustomMenuItem::new("show".to_string(), "Show");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    // endregion: --- System Tray
    // region: --- Database

    let data_dir = data_dir().unwrap(); // This assumes the operation won't fail
    let db_path = data_dir.join("sqweebu");

    let chat_db: Surreal<surrealdb::engine::local::Db> =
        Surreal::new::<RocksDb>(db_path.to_str().unwrap())
            .await
            .unwrap();
    let _ = chat_db.use_ns("user7").use_db("chat").await;

    let audio_db = Surreal::new::<Mem>(()).await.unwrap();
    let _ = audio_db.use_ns("user7").use_db("audio").await;

    // endregion: --- Database

    let (entry_send, entry_recv) = mpsc::channel::<Option<AudioEntry>>(32);

    let playback_send = init_playback_channel(entry_send).await;

    let playback_send_clone = playback_send.clone();
    let nexus = Arc::new(Mutex::new(AppState {
        playback_send,
        current_entry: Arc::new(Mutex::new(None)),
        chat_db,
        audio_db,
    }));

    let entry_recv_arc = Arc::new(Mutex::new(entry_recv));
    let nexus_clone = nexus.clone();
    tokio::spawn(async move {
        while let Some(entry) = entry_recv_arc.lock().await.recv().await {
            let nexus = nexus_clone.lock().await;
            let mut current_entry = nexus.current_entry.lock().await;
            *current_entry = entry.clone();
        }
    });

    let nexus_clone = nexus.clone();
    tokio::spawn(async move {
        if let Err(e) = listen_audio_database(nexus_clone).await {
            eprintln!("Error listening to audio database: {:?}", e);
        }
    });

    // Clone `playback_send` before moving it into the async block.
    let playback_send_clone = playback_send_clone.clone();
    tokio::spawn(async move {
        // Use a reference to the cloned `Sender` here, as required by the function signature.
        if let Err(e) = check_empty_sink(&playback_send_clone).await {
            eprintln!("Error: {:?}", e);
        }
    });

    // region: --- Tauri Build
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| handle_system_tray_event(app, event))
        .invoke_handler(tauri::generate_handler![
            pause_playback_from_frontend,
            resume_playback_from_frontend,
            stop_playback_from_frontend,
            process_input_from_frontend,
            get_chat_updates,
            get_current_entry,
            read_from_sentence_frontend,
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
    // endregion: --- Tauri Build
}

#[tauri::command]
async fn get_chat_updates(app: tauri::AppHandle) -> Result<String, String> {
    let nexus = get_nexus(app).await;
    let chat_db = &nexus.chat_db;
    let chat_entries_result = chat_db.select("chat").await;
    let chat_entries: Vec<ChatEntry> = match chat_entries_result {
        Ok(entries) => entries,
        Err(error) => return Err(format!("Database error: {:?}", error)),
    };
    serde_json::to_string(&chat_entries).map_err(|e| format!("Serialization error: {}", e))
}

#[tauri::command]
async fn read_from_sentence_frontend(
    app: tauri::AppHandle,
) -> Result<Option<AudioEntry>, Option<AudioEntry>> {
    let nexus = get_nexus(app).await;
    let current_entry = nexus.current_entry.lock().await;
    Ok(current_entry.clone())
}

#[tauri::command]
async fn process_input_from_frontend(text: String, app: tauri::AppHandle) -> Result<(), String> {
    let nexus = get_nexus(app).await;
    let chat_db = nexus.chat_db;
    let audio_db = nexus.audio_db;
    let _ = process_input(&text, chat_db, audio_db.clone()).await;
    Ok(())
}

#[tauri::command]
async fn get_current_entry(
    app: tauri::AppHandle,
) -> Result<Option<AudioEntry>, Option<AudioEntry>> {
    let app_state = get_nexus(app).await;
    let current_entry = app_state.current_entry.lock().await;
    Ok(current_entry.clone())
}

#[tauri::command]
async fn pause_playback_from_frontend(app: tauri::AppHandle) -> Result<(), Option<AudioEntry>> {
    let nexus = get_nexus(app).await;
    let playback_send = nexus.playback_send.clone();
    task::spawn(async move {
        let _ = playback_send.send(PlaybackCommand::Pause).await;
    });
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

async fn get_nexus(app: tauri::AppHandle) -> AppState {
    let nexus_lock = app.state::<Arc<Mutex<AppState>>>();
    let nexus = nexus_lock.lock().await;
    nexus.clone()
}

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
