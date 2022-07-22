use std::{collections::HashMap, sync::Mutex};

use serde::{Deserialize, Serialize};

use crate::id::Id;

pub(crate) type WorldState = Mutex<Entities>;

#[derive(Default)]
pub(crate) struct Entities {
    pub elements: HashMap<Id, Entity>,
}

impl Entities {
    pub fn create(&mut self) -> Entity {
        let entity = Entity {
            id: Id::new(),
            name: Some("New Entity".into()),
        };
        self.elements.insert(entity.id, entity.clone());
        entity
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Entity {
    pub id: Id,
    pub name: Option<String>,
}
