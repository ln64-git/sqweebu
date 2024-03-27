// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// region: --- imports
use _core::playback::{init_playback_channel, PlaybackCommand};
use _core::utils::listen_audio_database;
use _core::{process_input, AppState, ChatEntry};
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
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    let data_dir = data_dir().unwrap(); // This assumes the operation won't fail
    let db_path = data_dir.join("sqweebu");

    let chat_db: Surreal<surrealdb::engine::local::Db> =
        Surreal::new::<RocksDb>(db_path.to_str().unwrap())
            .await
            .unwrap();
    let _ = chat_db.use_ns("user7").use_db("chat").await;

    let audio_db = Surreal::new::<Mem>(()).await.unwrap();
    let _ = audio_db.use_ns("user7").use_db("audio").await;

    let (sentence_send, sentence_recv) = mpsc::channel::<String>(32);
    let sentence_recv_arc = Arc::new(Mutex::new(sentence_recv));

    let playback_send = init_playback_channel(sentence_send).await;

    let nexus = Arc::new(Mutex::new(AppState {
        playback_send,
        current_sentence: Arc::new(Mutex::new("".to_string())),
        chat_db,
        audio_db,
    }));

    let nexus_clone = nexus.clone();
    let sentence_recv_clone = sentence_recv_arc.clone(); // Assuming you intended to share an Arc<Mutex<Receiver>>
    tokio::spawn(async move {
        while let Some(sentence) = sentence_recv_clone.lock().await.recv().await {
            let app_state = nexus_clone.lock().await;
            let mut current_sentence = app_state.current_sentence.lock().await;
            *current_sentence = sentence.clone();
        }
    });

    let nexus_clone = nexus.clone();
    tokio::spawn(async move {
        if let Err(e) = listen_audio_database(nexus_clone).await {
            eprintln!("Error listening to audio database: {:?}", e);
        }
    });

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| handle_system_tray_event(app, event))
        .invoke_handler(tauri::generate_handler![
            pause_playback_from_frontend,
            resume_playback_from_frontend,
            stop_playback_from_frontend,
            process_input_from_frontend,
            get_chat_updates,
            get_current_sentence,
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

#[tauri::command]
async fn get_current_sentence(app: tauri::AppHandle) -> Result<String, String> {
    let app_state = get_nexus(app).await;
    let current_sentence = app_state.current_sentence.lock().await;
    if *current_sentence == "" {
        Ok("No Currently Selected Sentence".to_string())
    } else {
        Ok(current_sentence.clone())
    }
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

// async fn broadcast_chat_entries(
//     db_lock: Arc<Mutex<Surreal<surrealdb::engine::local::Db>>>,
//     sender: broadcast::Sender<String>,
// ) -> Result<(), String> {
//     let db = db_lock.lock().await;
//     // Simulated data for demonstration
//     let chat_entries: Option<Vec<ChatEntry>> = match db.select("chat").await {
//         Ok(entries) => Some(entries),
//         Err(error) => return Err(format!("Database error: {:?}", error)),
//     };
//     // Serialize the data
//     let serialized_data = serde_json::to_string(&chat_entries).map_err(|e| e.to_string())?;
//     println!("{:#?}", serialized_data);
//     // Broadcast the serialized data to connected WebSocket clients
//     sender.send(serialized_data).unwrap();
//     Ok(())
// }

async fn get_nexus(app: tauri::AppHandle) -> AppState {
    let nexus_lock = app.state::<Arc<Mutex<AppState>>>();
    let nexus = nexus_lock.lock().await;
    nexus.clone()
}

// region: --- Main Commands

#[tauri::command]
async fn process_input_from_frontend(text: String, app: tauri::AppHandle) -> Result<(), String> {
    let nexus = get_nexus(app).await;
    let chat_db = nexus.chat_db;
    let audio_db = nexus.audio_db;
    let _ = process_input(&text, chat_db, audio_db.clone()).await;
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
