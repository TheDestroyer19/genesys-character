use std::collections::HashMap;

use log::info;
use tauri::{command, State, generate_handler, Invoke};

use crate::{state::{Entity, WorldState}, id::Id};

pub(crate) fn commands() -> impl Fn(Invoke) {
    generate_handler![get_entities, create_entity]
}

#[command]
fn get_entities(world: State<WorldState>) -> HashMap<Id, Entity> {
    info!("Fetching all entities");
    world.lock().unwrap().elements.clone()
}

#[tauri::command]
fn create_entity(world: State<WorldState>, window: tauri::Window) -> Entity {
    info!("Creating entity");
    let entity = world.lock().unwrap().create();
    crate::window::open_or_focus_editor(&window, entity.id).unwrap();
    entity
}