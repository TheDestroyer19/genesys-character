use std::convert::Infallible;
use std::path::PathBuf;

use log::error;
use tauri::{Manager, Runtime, WindowBuilder, WindowUrl};

use crate::event::Event;
use crate::id::Id;

pub(crate) trait Window {
    fn label(&self) -> String;

    fn url(&self) -> PathBuf;

    fn configure<R>(&self, builder: WindowBuilder<R>) -> Result<tauri::Window<R>, tauri::Error>
    where
        R: Runtime;

    fn open_or_focus<M, R>(&self, manager: &M) -> Result<tauri::Window<R>, tauri::Error>
    where
        M: Manager<R> + Sized,
        R: Runtime,
    {
        let label = self.label();
        if let Some(window) = manager.windows().get(&label) {
            window.set_focus()?;
            Ok(window.clone())
        } else {
            self.configure(WindowBuilder::new(
                manager,
                label,
                WindowUrl::App(self.url()),
            ))
        }
    }
}

pub(crate) struct Character;

impl Window for Character {
    fn label(&self) -> String {
        "index".into()
    }

    fn url(&self) -> PathBuf {
        "index.html".into()
    }

    fn configure<R>(&self, builder: WindowBuilder<R>) -> Result<tauri::Window<R>, tauri::Error>
    where
        R: Runtime,
    {
        builder
            .title("Genesys Character")
            .fullscreen(false)
            .inner_size(800., 600.)
            .resizable(true)
            .build()
    }
}

pub(crate) struct Edit(pub Id);

impl Window for Edit {
    fn label(&self) -> String {
        format!("edit-item-{}", self.0)
    }
    fn url(&self) -> PathBuf {
        format!("edit-item.html?id={}", self.0).into()
    }

    fn configure<R>(&self, builder: WindowBuilder<R>) -> Result<tauri::Window<R>, tauri::Error>
    where
        R: Runtime,
    {
        builder
            .title("Edit")
            .fullscreen(false)
            .inner_size(400., 300.)
            .resizable(true)
            .build()
    }
}

pub(crate) struct EditCharacter;

impl Window for EditCharacter {
    fn label(&self) -> String {
        "edit-character".into()
    }

    fn url(&self) -> PathBuf {
        "edit-character.html".into()
    }

    fn configure<R>(&self, builder: WindowBuilder<R>) -> Result<tauri::Window<R>, tauri::Error>
    where
        R: Runtime,
    {
        builder
            .title("Edit Character")
            .fullscreen(false)
            .inner_size(400., 300.)
            .resizable(true)
            .build()
    }
}

pub(crate) fn setup(app: &mut tauri::App) -> Result<(), Infallible> {
    //Update editor window titles when their entity updates
    // let manager = app.handle();
    // crate::event::EntityUpdated::listen(app, move |entity| {
    //     let window = manager.get_window(&format!("{}-{}", EDITOR_WINDOW_PREFIX, entity.id));
    //     if let Some(window) = window {
    //         if let Err(e) = window.set_title(&format!("Edit {}", entity.name)) {
    //             error!("Failed to update window title: {}", e);
    //         }
    //     }
    // });

    //Close editor windows when their associated entity is deleted
    let manager = app.handle();
    crate::event::EntityDeleted::listen(app, move |id| {
        let window = manager.get_window(&Edit(id).label());
        if let Some(window) = window {
            if let Err(e) = window.close() {
                error!("Failed to close a window: {}", e);
            }
        }
    });

    Ok(())
}
