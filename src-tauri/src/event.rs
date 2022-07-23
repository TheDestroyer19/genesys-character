use log::{error, info};
use tauri::{EventHandler, Manager, Runtime};

use crate::{id::Id, state::Entity};

const ENTITY_UPDATED: &str = "entity-updated";
const ENTITY_DELETED: &str = "entity-deleted";

pub(crate) fn send_entity_updated<M, R>(manager: &M, entity: &Entity) -> Result<(), tauri::Error>
where
    M: Manager<R> + Sized,
    R: Runtime,
{
    info!("Sending '{}'", ENTITY_UPDATED);
    let data = serde_json::to_string(&entity)?;
    manager.trigger_global(ENTITY_UPDATED, Some(data));
    manager.emit_all(ENTITY_UPDATED, entity)
}

pub(crate) fn send_entity_deleted<M, R>(manager: &M, id: Id) -> Result<(), tauri::Error>
where
    M: Manager<R> + Sized,
    R: Runtime,
{
    info!("Sending '{}'", ENTITY_DELETED);
    let data = serde_json::to_string(&id)?;
    manager.trigger_global(ENTITY_DELETED, Some(data));
    manager.emit_all(ENTITY_DELETED, id)
}

pub(crate) fn listen_entity_deleted<M, R>(
    manager: &M,
    handler: impl Fn(Id) -> () + Send + 'static,
) -> EventHandler
where
    M: Manager<R> + Sized,
    R: Runtime,
{
    manager.listen_global(ENTITY_DELETED, move |event| {
        info!("'{}' was received", ENTITY_DELETED);
        let payload = match event.payload() {
            Some(s) => s,
            None => {
                error!(
                    "'{}' event missing payload - it should be an Id",
                    ENTITY_DELETED
                );
                return;
            }
        };

        match serde_json::from_str(payload) {
            Ok(id) => handler(id),
            Err(e) => error!(
                "'{}' event failed to parse payload - it should be an Id\n{}",
                ENTITY_DELETED, e
            ),
        }
    })
}
