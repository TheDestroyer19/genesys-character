use std::{collections::HashMap, sync::Mutex};

use crate::id::Id;

pub(crate) type WorldState = Mutex<Entities>;

#[derive(Default)]
pub(crate) struct Entities {
    pub elements: HashMap<Id, Entity>,
}

impl Entities {
    pub fn create(&mut self) -> Entity {
        let id = Id::new();
        let entity = "New Entity".to_string();
        self.elements.insert(id, entity.clone());
        entity
    }
}

pub(crate) type Entity = String;