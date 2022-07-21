import { CreateEntity, Entity, GetEntities } from "./rs-api";

const list = document.getElementById("list") as HTMLElement;//Assume that element exists
const add_item = document.getElementById("new-entity") as HTMLElement;//Assume that element exists

GetEntities().then((entities) => {
  entities.forEach((value: Entity, key: number) => {
    const element = document.createElement('li');
    element.textContent = key + ": " + value;
    list.append(element);
  })
}); 

add_item.addEventListener('click', _ => {
  console.log("adding item");
  CreateEntity().then(_ => location.reload());
});