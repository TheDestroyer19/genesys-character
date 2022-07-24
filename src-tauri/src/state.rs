use std::{collections::HashMap, sync::Mutex};

use serde::{Deserialize, Serialize};

use crate::id::Id;

pub(crate) type WorldState = Mutex<World>;

#[derive(Default)]
pub(crate) struct World {
    pub elements: HashMap<Id, Entity>,
}

impl World {
    pub fn create(&mut self) -> Entity {
        let entity = Entity {
            id: Id::new(),
            name: "New Entity".into(),
        };
        self.elements.insert(entity.id, entity.clone());
        entity
    }

    pub fn get(&self, id: Id) -> Option<&Entity> {
        self.elements.get(&id)
    }

    pub fn delete(&mut self, id: Id) {
        self.elements.remove(&id);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Entity {
    pub id: Id,
    pub name: String,
}
