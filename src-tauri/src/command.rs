use std::collections::HashMap;

use log::info;
use tauri::{command, State, generate_handler, Invoke, Runtime};

use crate::{state::{Entity, WorldState}, id::Id};

pub(crate) fn commands<R>() -> impl Fn(Invoke<R>) + Send + Sync + 'static
where R: Runtime {
    generate_handler![get_entities, create_entity]
}

#[command]
fn get_entities(entities: State<WorldState>) -> HashMap<Id, Entity> {
    info!("Fetching all entities");
    entities.lock().unwrap().elements.clone()
}

#[command]
fn create_entity(entities: State<WorldState>) -> Entity {
    info!("Creating entity");
    entities.lock().unwrap().create()
}