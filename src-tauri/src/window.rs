use std::convert::Infallible;

use log::error;
use tauri::{Manager, WindowBuilder};

use crate::state::Entity;

pub(crate) const CHARACTER_WINDOW: &str = "character";
pub(crate) const EDITOR_WINDOW_PREFIX: &str = "edit";

pub(crate) fn setup(app: &mut tauri::App) -> Result<(), Infallible> {
    //Update editor window titles when their entity updates
    let manager = app.handle();
    crate::event::listen_entity_updated(app, move |entity| {
        let window = manager.get_window(&format!("{}-{}", EDITOR_WINDOW_PREFIX, entity.id));
        if let Some(window) = window {
            if let Err(e) = window.set_title(&format!("Edit {}", entity.name)) {
                error!("Failed to update window title: {}", e);
            }
        }
    });

    //Close editor windows when their associated entity is deleted
    let manager = app.handle();
    crate::event::listen_entity_deleted(app, move |id| {
        let window = manager.get_window(&format!("{}-{}", EDITOR_WINDOW_PREFIX, id));
        if let Some(window) = window {
            if let Err(e) = window.close() {
                error!("Failed to close a window: {}", e);
            }
        }
    });

    Ok(())
}

/// Creates or raises the character window
pub(crate) fn open_or_focus_character<M, R>(manager: &M) -> Result<tauri::Window<R>, tauri::Error>
where
    M: Manager<R>,
    R: tauri::Runtime,
{
    if let Some(window) = manager.get_window(CHARACTER_WINDOW) {
        window.set_focus()?;
        Ok(window.clone())
    } else {
        WindowBuilder::new(
            manager,
            CHARACTER_WINDOW,
            tauri::WindowUrl::App("index.html".into()),
        )
        .title("Genesys Character")
        .fullscreen(false)
        .inner_size(800., 600.)
        .resizable(true)
        .build()
    }
}

/// Creates or focuses an editor window
pub(crate) fn open_or_focus_editor<M, R>(
    manager: &M,
    entity: &Entity,
) -> Result<tauri::Window<R>, tauri::Error>
where
    M: Manager<R>,
    R: tauri::Runtime,
{
    let label = format!("{}-{}", EDITOR_WINDOW_PREFIX, entity.id);
    if let Some(window) = manager.windows().get(&label) {
        window.set_focus()?;
        Ok(window.clone())
    } else {
        WindowBuilder::new(
            manager,
            &label,
            tauri::WindowUrl::App(format!("edit-item.html?id={}", entity.id).into()),
        )
        .title(format!("Edit {}", entity.name))
        .fullscreen(false)
        .inner_size(400., 300.)
        .resizable(true)
        .build()
    }
}
