use std::collections::HashMap;

use log::{info, warn};
use tauri::{command, generate_handler, Invoke, State};

use crate::{
    event::send_entity_updated,
    id::Id,
    state::{Entity, WorldState},
};

pub(crate) fn commands() -> impl Fn(Invoke) {
    generate_handler![create_entity, get_entities, get_entity, update_entity]
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
fn update_entity(window: tauri::Window, world: State<WorldState>, entity: Entity) {
    info!("Updating entity {:?}", entity.id);
    if let Some(old_e) = world.lock().unwrap().elements.get_mut(&entity.id) {
        *old_e = entity;
        send_entity_updated(&window, old_e).unwrap();
    } else {
        warn!("Tried to update nonexistant entity {:?}", entity.id);
    }
}
