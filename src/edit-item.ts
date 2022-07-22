import { Entity, GetEntity, UpdateEntity } from "./rs-api";

const NameInput = document.getElementById('name-input') as HTMLInputElement;

const ID = parseInt(new URLSearchParams(location.search).get('id') as string);

var openEntity: Entity | null = null;

GetEntity(ID).then(entity => {
    openEntity = entity;

    if (entity.name !== null) {
        NameInput.value = entity.name;
        NameInput.parentElement?.classList?.remove('hidden');
    }
});

//setup event handling
document.addEventListener('change', _event => {
    if (openEntity === null) {
        console.error("Missing Entity!");
        return;
    }

    openEntity.name = NameInput.value;
    UpdateEntity(openEntity);
})