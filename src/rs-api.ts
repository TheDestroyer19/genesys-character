import { invoke } from '@tauri-apps/api';
import { EventCallback, listen, UnlistenFn } from '@tauri-apps/api/event';

export type Id = number;
export interface Entity {
    id: Id,
    name: string | null,
};

export async function GetEntities(): Promise<Map<Id, Entity>> {
    const raw = await invoke('get_entities') as any;
    return new Map(Object.entries(raw).map(kv => [parseInt(kv[0]), kv[1] as Entity]));
}

export async function GetEntity(id: Id): Promise<Entity> {
    const raw = await invoke('get_entity', { id: id });
    return raw as Entity;
}

export async function CreateEntity(): Promise<Entity> {
    return await invoke('create_entity');
}

export async function EditEntity(id: Id) {
    return await invoke('edit_entity', { id: id });
}

export async function UpdateEntity(entity: Entity) {
    await invoke('update_entity', { entity: entity });
}

export async function ListenForElementUpdated(callback: EventCallback<Entity>): Promise<UnlistenFn> {
    return await listen('entity-updated', callback);
}