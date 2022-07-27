#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use state::World;

mod command;
mod event;
mod id;
mod menu;
mod state;
mod window;

fn main() {
    pretty_env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            window::setup(app)?;
            Ok(())
        })
        .manage(Mutex::new(World::new()))
        .menu(menu::build())
        .on_menu_event(menu::on_menu_event)
        .invoke_handler(command::commands())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
