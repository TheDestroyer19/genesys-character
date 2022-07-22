import { CreateEntity, GetEntities, ListenForElementUpdated } from "./rs-api";

const list = document.getElementById("list") as HTMLElement;//Assume that element exists
const add_item = document.getElementById("new-entity") as HTMLElement;//Assume that element exists

GetEntities().then((entities) => {
  entities.forEach(entity => {
    const element = document.createElement('li');
    element.setAttribute('data-entity', entity.id.toString());
    element.textContent = entity.id + ": " + entity.name;
    list.append(element);
  })
}); 

add_item.addEventListener('click', _ => {
  console.log("adding item");
  CreateEntity().then(entity => {
    const element = document.createElement('li');
    element.setAttribute('data-entity', entity.id.toString());
    element.textContent = entity.id + ": " + entity.name;
    list.append(element);
  });
});

ListenForElementUpdated(event => {
  let entity = event.payload;
  let element = document.querySelector(`[data-entity="${entity.id}"]`);
  if (element !== null) {
    element.textContent = entity.id + ": " + entity.name;
  }
});