import { invoke } from '@tauri-apps/api';
import { EventCallback, listen, UnlistenFn } from '@tauri-apps/api/event';

export type Id = number;
export interface Entity {
    id: Id,
    name: string,
    description: string,
};

export async function GetEntities(): Promise<Entity[]> {
    return await invoke('get_entities');
}

export async function GetEntity(id: Id): Promise<Entity> {
    const raw = await invoke('get_entity', { id: id });
    return raw as Entity;
}

export async function CreateEntity() {
    return await invoke('create_entity');
}

export async function EditEntity(id: Id) {
    await invoke('edit_entity', { id: id });
}

export async function UpdateEntity(entity: Entity) {
    await invoke('update_entity', { entity: entity });
}

export async function DeleteEntity(id: Id) {
    await invoke('delete_entity', { id: id });
}

export async function ListenForEntityCreated(callback: EventCallback<Entity>): Promise<UnlistenFn> {
    return await listen('entity-created', callback);
}

export async function ListenForEntityUpdated(callback: EventCallback<Entity>): Promise<UnlistenFn> {
    return await listen('entity-updated', callback);
}

export async function ListenForEntityDeleted(callback: EventCallback<Id>): Promise<UnlistenFn> {
    return await listen('entity-deleted', callback);
}