use std::any::type_name;

use log::{error, info};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tauri::{EventHandler, Manager, Runtime};

use crate::{id::Id, state::Entity};

const ENTITY_UPDATED: &str = "entity-updated";
const ENTITY_DELETED: &str = "entity-deleted";

pub(crate) fn send_entity_updated<M, R>(manager: &M, entity: &Entity) -> Result<(), tauri::Error>
where
    M: Manager<R> + Sized,
    R: Runtime,
{
    send(manager, ENTITY_UPDATED, entity)
}

pub(crate) fn listen_entity_updated<M, R>(
    manager: &M,
    handler: impl Fn(Entity) -> () + Send + 'static,
) -> EventHandler
where
    M: Manager<R> + Sized,
    R: Runtime,
{
    listen(manager, ENTITY_UPDATED, handler)
}

pub(crate) fn send_entity_deleted<M, R>(manager: &M, id: Id) -> Result<(), tauri::Error>
where
    M: Manager<R> + Sized,
    R: Runtime,
{
    send(manager, ENTITY_DELETED, id)
}

pub(crate) fn listen_entity_deleted<M, R>(
    manager: &M,
    handler: impl Fn(Id) -> () + Send + 'static,
) -> EventHandler
where
    M: Manager<R> + Sized,
    R: Runtime,
{
    listen(manager, ENTITY_DELETED, handler)
}

fn send<P, M, R>(manager: &M, event_type: &'static str, payload: P) -> Result<(), tauri::Error>
 where P: Serialize + Clone, M: Manager<R>, R: Runtime {
    info!("Sending '{}'", event_type);
    let data = serde_json::to_string(&payload)?;
    manager.trigger_global(event_type, Some(data));
    manager.emit_all(event_type, payload)
}

fn listen<P, M, R>(
    manager: &M,
    event_type: &'static str,
    handler: impl Fn(P) -> () + Send + 'static
) -> EventHandler
where 
    P: DeserializeOwned,
    M: Manager<R> + Sized,
    R: Runtime,
{
    manager.listen_global(event_type, move | event| {
        info!("'{}' was received", event_type);
        let payload = match event.payload() {
            Some(s) => s,
            None => {
                error!(
                    "'{}' event missing payload - it should be an Id",
                    event_type
                );
                return;
            }
        };

        match serde_json::from_str(payload) {
            Ok(id) => handler(id),
            Err(e) => error!(
                "'{}' event failed to parse payload - it should be an {}\n{}",
                event_type, type_name::<P>(), e
            ),
        }
    })
}