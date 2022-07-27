import { invoke } from '@tauri-apps/api';
import { EventCallback, listen, UnlistenFn } from '@tauri-apps/api/event';

export type Id = number;
export interface Entity {
    id: Id,
    name: string,
    description: string,
    character: Character | null,
};

export interface Character {
    player: string,
    archetype: string,
    career: string,
    specializations: string,
    xp: number,
    total_xp: number,

    brawn: number,
    agility: number,
    intellect: number,
    cunning: number,
    willpower: number,
    presence: number,
    force_rank: number | null,

    soak: number,
    wounds: number,
    wounds_threshold: number,
    strain: number,
    strain_threshold: number,
    defense_melee: number,
    defense_ranged: number,

    encumbrance: number,
    encumbrance_threshold: number,
}

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