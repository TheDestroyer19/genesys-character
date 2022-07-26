use std::{collections::HashMap, sync::Mutex};

use anyhow::bail;
use log::warn;
use serde::{Deserialize, Serialize};

use crate::id::Id;

pub(crate) type WorldState = Mutex<World>;

#[derive(Default)]
pub(crate) struct World {
    elements: HashMap<Id, Entity>,
}

impl World {
    pub fn create(&mut self) -> Entity {
        let entity = Entity {
            id: Id::new(),
            name: "New Entity".into(),
            description: "".into(),
        };
        self.elements.insert(entity.id, entity.clone());
        entity
    }

    pub fn get_all(&self) -> impl Iterator<Item = &'_ Entity> {
        self.elements.values()
    }

    pub fn get(&self, id: Id) -> Option<&Entity> {
        self.elements.get(&id)
    }

    pub fn update(&mut self, entity: Entity) -> Result<&Entity, anyhow::Error> {
        if let Some(old_entity) = self.elements.get_mut(&entity.id) {
            *old_entity = entity;
            Ok(old_entity)
        } else {
            bail!("Tried to update non-existant entity {:?}", entity.id)
        }
    }

    pub fn delete(&mut self, id: Id) {
        if self.elements.remove(&id).is_none() {
            warn!("Tried to delete a non-exitant entity {:?}", id);
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Entity {
    pub id: Id,
    pub name: String,
    pub description: String,
}
