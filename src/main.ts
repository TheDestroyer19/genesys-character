import { CreateEntity, EditEntity, Entity, GetEntities, ListenForEntityCreated, ListenForEntityDeleted, ListenForEntityUpdated } from "./rs-api";

const list = document.getElementById("list") as HTMLElement;//Assume that element exists
const add_item = document.getElementById("new-entity") as HTMLElement;//Assume that element exists

function CreateUnknownDisplay(entity: Entity) {
  const edit = document.createElement('button');
  edit.classList.add('edit-btn');
  edit.textContent = 'Edit';

  const name = document.createElement('b');
  name.textContent = entity.id + ": " + entity.name;

  const description = document.createElement('span');
  description.textContent = entity.description;

  const element = document.createElement('div');
  element.setAttribute('data-entity', entity.id.toString());
  element.append(name, edit, document.createElement('br'), description);

  list.append(element);
}
function UpdateUnknownDisplay(entity: Entity) {
  document.querySelectorAll(`[data-entity="${entity.id}"]`).forEach(element => {
    let text = element.querySelector('b');
    if (text !== null) {
      text.textContent = entity.id + ": " + entity.name;
    }
    let desc = element.querySelector('span');
    if (desc !== null) {
      desc.textContent = entity.description;
    }
  });
}

function UpdateCharacterDisplay(entity: Entity) {
  console.assert(entity.character !== null);
  if (entity.character === null) return;
  
  document.getElementById('player')!.textContent = entity.character.player;
  document.getElementById('name')!.textContent = entity.name;
  document.getElementById('xp')!.textContent = entity.character.xp.toString();
  document.getElementById('xp-total')!.textContent = entity.character.total_xp.toString();
  document.getElementById('career')!.textContent = entity.character.career;
  document.getElementById('specializations')!.textContent = entity.character.specializations;

  document.querySelector('#brawn>.value')!.textContent = entity.character.brawn.toString();
  document.querySelector('#agility>.value')!.textContent = entity.character.agility.toString();
  document.querySelector('#intellect>.value')!.textContent = entity.character.intellect.toString();
  document.querySelector('#cunning>.value')!.textContent = entity.character.cunning.toString();
  document.querySelector('#willpower>.value')!.textContent = entity.character.willpower.toString();
  document.querySelector('#presence>.value')!.textContent = entity.character.presence.toString();
  const force_rank = document.querySelector('#force-rank')!;
  if (entity.character.force_rank !== null) {
    force_rank.classList.remove('hidden');
    force_rank.querySelector(".value")!.textContent = entity.character.force_rank.toString();
  } else {
    force_rank.classList.add('hidden');
  }
}

GetEntities().then((entities) => {
  entities.forEach(entity => {
    if (entity.character !== null) {
      UpdateCharacterDisplay(entity);
    } else {
      CreateUnknownDisplay(entity);
    }
  });
}); 

add_item.addEventListener('click', _ => {
  console.log("adding item");
  CreateEntity();
});

ListenForEntityCreated(event => {
  let entity = event.payload;
  if (entity.character !== null) {
    console.warn!("There should only be one element with the character component");
  } else {
    CreateUnknownDisplay(entity);
  }
})

ListenForEntityUpdated(event => {
  let entity = event.payload;
  if (entity.character !== null) {
    UpdateCharacterDisplay(entity);
  } else {
    UpdateUnknownDisplay(entity);
  }
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