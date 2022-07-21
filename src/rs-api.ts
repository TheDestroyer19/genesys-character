import { invoke } from '@tauri-apps/api';

export type Id = number;
export type Entity = string;

export async function GetEntities(): Promise<Map<Id, Entity>> {
    const raw = await invoke('get_entities') as any;
    console.log(raw);
    return new Map(Object.entries(raw).map(kv => [parseInt(kv[0]), kv[1] as Entity]));
}

export async function CreateEntity(): Promise<Entity> {
    return await invoke('create_entity');
}