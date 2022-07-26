import { Entity, GetEntity, UpdateEntity, DeleteEntity } from "./rs-api";

const NameInput = document.getElementById('name-input') as HTMLInputElement;
const DescInput = document.getElementById('description-input') as HTMLTextAreaElement;

const ID = parseInt(new URLSearchParams(location.search).get('id') as string);

var openEntity: Entity | null = null;

GetEntity(ID).then(entity => {
    openEntity = entity;

    NameInput.value = entity.name;
    DescInput.value = entity.description;
});

//setup event handling
document.addEventListener('change', _event => {
    if (openEntity === null) {
        console.error("Missing Entity!");
        return;
    }

    openEntity.name = NameInput.value;
    openEntity.description = DescInput.value;
    UpdateEntity(openEntity);
})

document.getElementById('delete')?.addEventListener('click', _ => DeleteEntity(ID));