import { CreateEntity, EditEntity, Entity, GetEntities, ListenForEntityCreated, ListenForEntityDeleted, ListenForEntityUpdated } from "./rs-api";

const list = document.getElementById("list") as HTMLElement;//Assume that element exists
const add_item = document.getElementById("new-entity") as HTMLElement;//Assume that element exists

function CreateDisplay(entity: Entity) {
  const edit = document.createElement('button');
  edit.classList.add('edit-btn');
  edit.textContent = 'Edit';

  const text = document.createElement('span');
  text.textContent = entity.id + ": " + entity.name;

  const element = document.createElement('li');
  element.setAttribute('data-entity', entity.id.toString());
  element.append(text, edit);

  list.append(element);
}

GetEntities().then((entities) => {
  entities.forEach(CreateDisplay);
}); 

add_item.addEventListener('click', _ => {
  console.log("adding item");
  CreateEntity();
});

ListenForEntityCreated(event => {
  let entity = event.payload;
  CreateDisplay(entity);
})

ListenForEntityUpdated(event => {
  let entity = event.payload;
  document.querySelectorAll(`[data-entity="${entity.id}"]`).forEach(element => {
    let text = element.querySelector('span');
    if (text !== null) {
      text.textContent = entity.id + ": " + entity.name;
    }
  });
});

ListenForEntityDeleted(event => {
  let id = event.payload;
  document.querySelectorAll(`[data-entity="${id}"]`).forEach(element => element.remove());
});

document.addEventListener('click', event => {
  if (!(event.target instanceof HTMLButtonElement)) {
    return;
  }

  let current: HTMLElement | null = event.target;
  while (current !== null && !current.hasAttribute('data-entity')) {
    current = current.parentElement;
  }

  if (current !== null && current.hasAttribute('data-entity')) {
    let id = parseInt(current.getAttribute('data-entity') as string);
    EditEntity(id);
  }
})