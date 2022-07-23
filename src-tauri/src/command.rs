use std::collections::HashMap;

use log::{info, warn};
use tauri::{api::dialog, command, generate_handler, Invoke, Manager, State};

use crate::{
    event::send_entity_deleted,
    event::send_entity_updated,
    id::Id,
    state::{Entity, WorldState},
};

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
fn create_entity(world: State<WorldState>, window: tauri::Window) -> Entity {
    info!("Creating entity");
    let entity = world.lock().unwrap().create();
    crate::window::open_or_focus_editor(&window, entity.id).unwrap();
    entity
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
    if world.contains(id) {
        crate::window::open_or_focus_editor(&window, id).unwrap();
    } else {
        warn!("Cannot open editor for nonexistant entity {:?}", id);
    }
}

#[command]
fn update_entity(window: tauri::Window, world: State<WorldState>, entity: Entity) {
    info!("Updating entity {:?}", entity.id);
    if let Some(old_e) = world.lock().unwrap().elements.get_mut(&entity.id) {
        *old_e = entity;
        send_entity_updated(&window, old_e).unwrap();
    } else {
        warn!("Tried to update nonexistant entity {:?}", entity.id);
    }
}

#[command]
fn delete_entity(window: tauri::Window, world: State<WorldState>, id: Id) {
    let (title, message) = {
        let world = world.lock().unwrap();
        let entity = match world.get(id) {
            Some(entity) => entity,
            None => return,
        };
        if let Some(name) = entity.name.as_ref() {
            (
                format!("Deleting {}", name),
                format!(
                    "Are you sure you want to delete {}?\nIt will be gone forever",
                    name
                ),
            )
        } else {
            (
                "Deleting".to_string(),
                "Are you sure you want to delete this?\nIt will be gone forever".to_string(),
            )
        }
    };

    dialog::confirm(Some(&window.clone()), title, message, move |confirmed| {
        if confirmed {
            let world = window.state::<WorldState>();
            world.lock().unwrap().delete(id);
            send_entity_deleted(&window, id).unwrap();
        }
    })
}
