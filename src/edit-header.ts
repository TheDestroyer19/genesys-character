import { Entity, GetCharacter, UpdateEntity } from "./rs-api";

var openEntity: Entity | null = null;

function BuildInputConnection(info: [string, (e: Entity) => string, (e: Entity, newvalue: string) => void][]) {
    const elements = info.map(value => [document.getElementById(value[0]), value[1], value[2]] as [HTMLInputElement, (e: Entity) => string, (e: Entity, newvalue: string) => void]
    );
    
    GetCharacter().then(entity => {
        if (entity.character === null) {
            console.error("Missing Character component");
            return;
        }
    
        openEntity = entity;

        elements.forEach(e => e[0].value = e[1](entity));
    });

    document.addEventListener('change', _ => {
        if (openEntity === null) {
            console.error("Missing Entity!");
            return;
        } else if (openEntity.character === null) {
            console.error("Missing Character component");
            return;
        }

        elements.forEach(e => e[2](openEntity as Entity, e[0].value));

        UpdateEntity(openEntity);
    });
}

BuildInputConnection([
    ['name-input', e => e.name, (e, nv) => e.name = nv],
    ['player-input', e => e.character!.player, (e, nv) => e.character!.player = nv],
    ['xp', e => e.character!.xp.toString(), (e, nv) => e.character!.xp = parseInt(nv)],
    ['total-xp', e => e.character!.total_xp.toString(), (e, nv) => e.character!.total_xp = parseInt(nv)],
    ['archetype', e => e.character!.archetype, (e, nv) => e.character!.archetype = nv],
    ['career', e => e.character!.career, (e, nv) => e.character!.career = nv],
    ['specializations', e => e.character!.specializations, (e, nv) => e.character!.specializations = nv],
]);