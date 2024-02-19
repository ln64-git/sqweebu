// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::Arc;

use app::AppState;
use app::PlaybackCommand;
use app::_utils::azure::__cmd__speak_text;
use app::_utils::azure::speak_text;
use app::_utils::playback;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tokio::sync::Mutex;
use tokio::task;

#[tokio::main]
async fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    let playback_send = playback::init_playback_channel().await;

    // Create the AppState instance
    let nexus = Arc::new(Mutex::new(AppState {
        playback_send: playback_send.clone(),
    }));

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| handle_system_tray_event(app, event))
        .invoke_handler(tauri::generate_handler![speak_text_from_frontend])
        .manage(nexus.clone()) // Manage the state with Tauri
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
async fn speak_text_from_frontend(text: String, app: tauri::AppHandle) -> Result<(), String> {
    println!("Received text: {}", text);

    // Access the playback_send from the app handle
    let playback_send = {
        let nexus_lock = app.state::<Arc<Mutex<AppState>>>();
        let nexus = nexus_lock.lock().await;
        nexus.playback_send.clone()
    };

    task::spawn(async move {
        speak_text(&text, &playback_send).await;
    });
    Ok(())
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
