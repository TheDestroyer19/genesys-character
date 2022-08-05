use log::{error, info, warn};
use tauri::{api::dialog, command, generate_handler, Invoke, Manager, State};

use crate::event::{EntityDeleted, EntityUpdated, Event};
use crate::id::Id;
use crate::state::{Entity, WorldState};
use crate::window::Window;

pub(crate) fn commands() -> impl Fn(Invoke) {
    generate_handler![
        create_entity,
        get_entities,
        get_entity,
        edit_entity,
        update_entity,
        delete_entity,
        get_character,
        edit_character,
    ]
}

#[command]
fn create_entity(world: State<WorldState>, window: tauri::Window) {
    info!("Creating entity");
    let entity = world.lock().unwrap().create();
    crate::window::Edit(entity.id)
        .open_or_focus(&window)
        .unwrap();
    if let Err(e) = crate::event::EntityCreated::send(&window, &entity) {
        error!("Failed to create entity: {}", e);
    }
}

#[command]
fn get_entities(world: State<WorldState>) -> Vec<Entity> {
    info!("Fetching all entities");
    world.lock().unwrap().get_all().cloned().collect()
}

#[command]
fn get_entity(world: State<WorldState>, id: Id) -> Option<Entity> {
    info!("Fetching entity {:?}", id);
    world.lock().unwrap().get(id).cloned()
}

#[command]
fn edit_entity(window: tauri::Window, world: State<WorldState>, id: Id) {
    info!("Opening editor for entity {:?}", id);
    let world = world.lock().unwrap();
    if let Some(entity) = world.get(id) {
        crate::window::Edit(entity.id)
            .open_or_focus(&window)
            .unwrap();
    } else {
        warn!("Cannot open editor for nonexistant entity {:?}", id);
    }
}

#[command]
fn update_entity(window: tauri::Window, world: State<WorldState>, entity: Entity) {
    info!("Updating entity {:?}", entity.id);
    let result = world
        .lock()
        .unwrap()
        .update(entity)
        .and_then(|e| EntityUpdated::send(&window, e));

    if let Err(e) = result {
        error!("Failed to update entity: {}", e)
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

#[command]
fn get_character(world: State<WorldState>) -> Entity {
    info!("Fetching Character");
    let world = world.lock().unwrap();
    world.get_character().clone()
}

#[command]
fn edit_character(window: tauri::Window) {
    info!("Opening character entity for editing");
    crate::window::EditCharacter.open_or_focus(&window).unwrap();
}
