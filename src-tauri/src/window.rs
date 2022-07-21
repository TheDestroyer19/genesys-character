use tauri::{Manager, WindowBuilder};

use crate::id::Id;

pub(crate) const CHARACTER_WINDOW: &str = "character";
pub(crate) const EDITOR_WINDOW_PREFIX: &str = "edit";

/// Creates or raises the character window
pub(crate) fn open_or_focus_character<M, R>(
    manager: &M,
) -> Result<tauri::Window<R>, tauri::Error>
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
    id: Id,
) -> Result<tauri::Window<R>, tauri::Error>
where
    M: Manager<R>,
    R: tauri::Runtime,
{
    let label = format!("{}-{}", EDITOR_WINDOW_PREFIX, id);
    if let Some(window) = manager.windows().get(&label) {
        window.set_focus()?;
        Ok(window.clone())
    } else {
        WindowBuilder::new(
            manager,
            &label,
            tauri::WindowUrl::App(format!("edit-item.html?id={}",id).into()),
        )
        .title("Edit")
        .fullscreen(false)
        .inner_size(800., 600.)
        .resizable(true)
        .build()
    }
}
