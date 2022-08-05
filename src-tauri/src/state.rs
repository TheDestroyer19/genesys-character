use std::{collections::HashMap, sync::Mutex};

use anyhow::bail;
use log::warn;
use serde::{Deserialize, Serialize};

use crate::id::Id;

pub(crate) type WorldState = Mutex<World>;

pub(crate) struct World {
    elements: HashMap<Id, Entity>,
    character_id: Id,
}

impl World {
    pub fn new() -> Self {
        let mut elements = HashMap::new();

        let id = Id::new();
        elements.insert(
            id,
            Entity {
                id,
                name: "Unnamed Character".into(),
                description: "".into(),
                character: Some(Box::new(Character::default())),
            },
        );

        Self {
            elements,
            character_id: id,
        }
    }

    pub fn create(&mut self) -> Entity {
        let entity = Entity {
            id: Id::new(),
            name: "New Entity".into(),
            description: "".into(),
            character: None,
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

    pub fn get_character(&self) -> &Entity {
        self.get(self.character_id).unwrap()
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
    pub character: Option<Box<Character>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Character {
    pub player: String,
    pub archetype: String,
    pub career: String,
    pub specializations: String,
    pub xp: i32,
    pub total_xp: i32,

    pub brawn: i32,
    pub agility: i32,
    pub intellect: i32,
    pub cunning: i32,
    pub willpower: i32,
    pub presence: i32,
    pub force_rank: Option<i32>,

    pub soak: i32,
    pub wounds: i32,
    pub wounds_threshold: i32,
    pub strain: i32,
    pub strain_threshold: i32,
    pub defense_melee: i32,
    pub defense_ranged: i32,

    pub encumbrance: i32,
    pub encumbrance_threshold: i32,
}
impl Default for Character {
    fn default() -> Self {
        Self {
            player: Default::default(),
            archetype: Default::default(),
            career: Default::default(),
            specializations: Default::default(),
            xp: 0,
            total_xp: 0,
            brawn: 2,
            agility: 2,
            intellect: 2,
            cunning: 2,
            willpower: 2,
            presence: 2,
            force_rank: None,
            soak: 2,
            wounds: 0,
            wounds_threshold: 2,
            strain: 0,
            strain_threshold: 2,
            defense_melee: 0,
            defense_ranged: 0,
            encumbrance: 0,
            encumbrance_threshold: 7,
        }
    }
}
