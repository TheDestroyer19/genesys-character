use tauri::{Manager, Runtime};

use crate::state::Entity;

pub(crate) fn send_entity_updated<M, R>(manager: &M, entity: &Entity) -> Result<(), tauri::Error>
where
    M: Manager<R> + Sized,
    R: Runtime,
{
    manager.emit_all("entity-updated", entity)
}
