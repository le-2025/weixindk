use commands::{config, wechat};
use process::monitor;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

pub mod mutex;
pub mod process;
pub mod storage;
pub mod commands;

static MINIMIZE_TO_TRAY: AtomicBool = AtomicBool::new(false);

pub fn get_minimize_to_tray() -> bool {
    MINIMIZE_TO_TRAY.load(Ordering::Relaxed)
}

pub fn set_minimize_to_tray(enabled: bool) {
    MINIMIZE_TO_TRAY.store(enabled, Ordering::Relaxed);
}

pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            let minimize_to_tray = {
                match crate::storage::database::Database::new() {
                    Ok(db) => match db.get_config("minimize_to_tray") {
                        Ok(Some(v)) => v != "false",
                        _ => false,
                    },
                    _ => false,
                }
            };
            MINIMIZE_TO_TRAY.store(minimize_to_tray, Ordering::Relaxed);

            let show = MenuItemBuilder::with_id("show", "显示主窗口").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().unwrap())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .tooltip("械式微信多开器")
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }

            monitor::start_process_monitor(app.handle().clone());
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if MINIMIZE_TO_TRAY.load(Ordering::Relaxed) {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            wechat::launch_wechat,
            wechat::get_wechat_instances,
            wechat::update_instance_label,
            wechat::terminate_instance,
            wechat::get_wechat_path,
            wechat::sync_instances,
            wechat::relaunch_wechat,
            wechat::delete_instance,
            config::get_app_config,
            config::save_app_config,
            config::set_minimize_to_tray,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}