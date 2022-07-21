#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use state::{Entities, WorldState};

mod command;
mod menu;
mod window;
mod state;
mod id;

fn main() {
    pretty_env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            window::open_or_focus_character(app)?;
            Ok(())
        })
        .manage(WorldState::new(Entities::default()))
        .menu(menu::build())
        .on_menu_event(menu::on_menu_event)
        .invoke_handler(command::commands())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
