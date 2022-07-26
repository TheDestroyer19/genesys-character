use std::collections::HashMap;

use log::{info, warn, error};
use tauri::{api::dialog, command, generate_handler, Invoke, Manager, State};

use crate::event::{EntityDeleted, EntityUpdated, Event};
use crate::id::Id;
use crate::state::{Entity, WorldState};

pub(crate) fn commands() -> impl Fn(Invoke) {
    generate_handler![
        create_entity,
        get_entities,
        get_entity,
        edit_entity,
        update_entity,
        delete_entity
    ]
}

#[command]
fn create_entity(world: State<WorldState>, window: tauri::Window) {
    info!("Creating entity");
    let entity = world.lock().unwrap().create();
    crate::window::open_or_focus_editor(&window, &entity).unwrap();
    if let Err(e) = crate::event::EntityCreated::send(&window, &entity) {
        error!("Failed to create entity: {}", e);
    }
}

#[command]
fn get_entities(world: State<WorldState>) -> HashMap<Id, Entity> {
    info!("Fetching all entities");
    world.lock().unwrap().elements.clone()
}

#[command]
fn get_entity(world: State<WorldState>, id: Id) -> Option<Entity> {
    info!("Fetching entity {:?}", id);
    world.lock().unwrap().elements.get(&id).cloned()
}

#[command]
fn edit_entity(window: tauri::Window, world: State<WorldState>, id: Id) {
    info!("Opening editor for entity {:?}", id);
    let world = world.lock().unwrap();
    if let Some(entity) = world.get(id) {
        crate::window::open_or_focus_editor(&window, entity).unwrap();
    } else {
        warn!("Cannot open editor for nonexistant entity {:?}", id);
    }
}

#[command]
fn update_entity(window: tauri::Window, world: State<WorldState>, entity: Entity) {
    info!("Updating entity {:?}", entity.id);
    if let Some(old_e) = world.lock().unwrap().elements.get_mut(&entity.id) {
        *old_e = entity;
        EntityUpdated::send(&window, old_e).unwrap();
    } else {
        warn!("Tried to update nonexistant entity {:?}", entity.id);
    }
}

#[command]
fn delete_entity(window: tauri::Window, world: State<WorldState>, id: Id) {
    info!("Deleting entity {:?}", id);
    let (title, message) = {
        let world = world.lock().unwrap();
        let entity = match world.get(id) {
            Some(entity) => entity,
            None => return,
        };

        (
            format!("Deleting {}", entity.name),
            format!(
                "Are you sure you want to delete {}?\nIt will be gone forever",
                entity.name
            ),
        )
    };

    dialog::confirm(Some(&window.clone()), title, message, move |confirmed| {
        if confirmed {
            let world = window.state::<WorldState>();
            world.lock().unwrap().delete(id);
            EntityDeleted::send(&window, &id).unwrap();
        }
    })
}
