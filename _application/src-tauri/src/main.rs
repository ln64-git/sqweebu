// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// region: --- imports

use _core::{playback, speak_ollama, speak_text};
use _interface::{AppState, PlaybackCommand};
use tauri::Manager;
// use tauri::SystemTray;
// use tauri::SystemTrayEvent;
// use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
// endregion: --- imports

#[tokio::main]
async fn main() {
    // let show = CustomMenuItem::new("show".to_string(), "Show");
    // let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let tray_menu = SystemTrayMenu::new()
    //     .add_item(show)
    //     .add_item(hide)
    //     .add_item(quit);
    // let system_tray = SystemTray::new().with_menu(tray_menu);

    let playback_send = playback::init_playback_channel().await;

    let nexus = Arc::new(Mutex::new(AppState {
        playback_send: playback_send.clone(),
    }));

    tauri::Builder::default()
        // .system_tray(system_tray)
        // .on_system_tray_event(|app, event| handle_system_tray_event(app, event))
        .invoke_handler(tauri::generate_handler![
            speak_text_from_frontend,
            speak_ollama_from_frontend,
            pause_playback_from_frontend,
            resume_playback_from_frontend,
            stop_playback_from_frontend,
            fast_forward_playback_from_frontend
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

// region: --- Main Commands

#[tauri::command]
async fn speak_text_from_frontend(text: String, app: tauri::AppHandle) -> Result<(), String> {
    let playback_send = {
        let nexus_lock = app.state::<Arc<Mutex<AppState>>>();
        let nexus = nexus_lock.lock().await;
        nexus.playback_send.clone()
    };
    task::spawn(async move {
        let _ = speak_text(&text, &playback_send).await;
    });
    Ok(())
}

#[tauri::command]
async fn speak_ollama_from_frontend(prompt: String, app: tauri::AppHandle) -> Result<(), String> {
    let playback_send = {
        let nexus_lock = app.state::<Arc<Mutex<AppState>>>();
        let nexus = nexus_lock.lock().await;
        nexus.playback_send.clone()
    };
    task::spawn(async move {
        let _ = speak_ollama(prompt, &playback_send).await;
    });
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

// fn handle_system_tray_event(app: &tauri::AppHandle, event: SystemTrayEvent) {
//     match event {
//         SystemTrayEvent::LeftClick {
//             position: _,
//             size: _,
//             ..
//         } => {
//             println!("system tray received a left click");
//         }
//         SystemTrayEvent::RightClick {
//             position: _,
//             size: _,
//             ..
//         } => {
//             println!("system tray received a right click");
//         }
//         SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
//             "quit" => {
//                 std::process::exit(0);
//             }
//             "show" => {
//                 let window = app.get_window("main").unwrap();
//                 window.show().unwrap();
//             }
//             "hide" => {
//                 let window = app.get_window("main").unwrap();
//                 window.hide().unwrap();
//             }
//             _ => {}
//         },
//         _ => {}
//     }
// }
