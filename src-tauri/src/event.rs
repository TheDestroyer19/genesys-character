use std::any::type_name;

use log::{error, info};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::{EventHandler, Manager, Runtime};

use crate::{id::Id, state::Entity};

pub(crate) trait Event<M, R>
where
    M: Manager<R> + Sized,
    R: Runtime,
{
    type Payload: Serialize + DeserializeOwned + Clone;
    fn name() -> &'static str;
    fn listen(manager: &M, handler: impl Fn(Self::Payload) -> () + Send + 'static) -> EventHandler {
        let name = Self::name();
        let type_name = type_name::<Self::Payload>();
        manager.listen_global(name, move |event| {
            info!("'{name}' was received");
            let payload = match event.payload() {
                Some(s) => s,
                None => {
                    error!("'{name}' event missing payload - it should be an {type_name}");
                    return;
                }
            };

            match serde_json::from_str(payload) {
                Ok(id) => handler(id),
                Err(e) => error!(
                    "'{name}' event failed to parse payload - it should be an {type_name}\n{e}"
                ),
            }
        })
    }
    fn send(manager: &M, payload: &Self::Payload) -> Result<(), tauri::Error> {
        let name = Self::name();
        info!("Sending '{name}'");
        let data = serde_json::to_string(&payload)?;
        manager.trigger_global(name, Some(data));
        manager.emit_all(name, payload)
    }
}

pub(crate) struct EntityUpdated;
impl<M: Manager<R>, R: Runtime> Event<M, R> for EntityUpdated {
    type Payload = Entity;

    fn name() -> &'static str {
        "entity-updated"
    }
}
pub(crate) struct EntityDeleted;
impl<M: Manager<R>, R: Runtime> Event<M, R> for EntityDeleted {
    type Payload = Id;
    fn name() -> &'static str {
        "entity-deleted"
    }
}
