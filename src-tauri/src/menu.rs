use log::{info, warn};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

use crate::window::Window;

pub fn build() -> Menu {
    let file = Submenu::new(
        "File",
        Menu::new()
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Quit),
    );

    let view = Submenu::new(
        "View",
        Menu::new().add_item(CustomMenuItem::new("view-character", "Character Sheet")),
    );

    Menu::new().add_submenu(file).add_submenu(view)
}

pub fn on_menu_event(event: WindowMenuEvent) {
    let window = event.window();
    info!(
        "Menu event {:?} on window {:?}",
        event.menu_item_id(),
        window.label()
    );

    match event.menu_item_id() {
        "quit" => {
            //TODO check app state
            std::process::exit(0);
        }
        "close" => {
            window.close().unwrap();
        }
        "view-character" => {
            crate::window::Character.open_or_focus(window).unwrap();
        }
        other => warn!("Unhandled menu event id: {}", other),
    };
}
