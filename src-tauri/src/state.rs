use std::{collections::HashMap, sync::Mutex};

use serde::{Deserialize, Serialize};

use crate::id::Id;

pub(crate) type WorldState<'r> = tauri::State<'r, Mutex<World>>;

#[derive(Default)]
pub(crate) struct World {
    pub elements: HashMap<Id, Entity>,
}

impl World {
    pub fn create(&mut self) -> Entity {
        let entity = Entity {
            id: Id::new(),
            name: Some("New Entity".into()),
        };
        self.elements.insert(entity.id, entity.clone());
        entity
    }

    pub fn contains(&self, id: Id) -> bool {
        self.elements.contains_key(&id)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Entity {
    pub id: Id,
    pub name: Option<String>,
}
