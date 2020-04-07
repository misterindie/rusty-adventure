use itemlist::ItemList;
use npclist::NPCList;

pub struct Room {
    pub items: ItemList,
    pub npcs: NPCList,
    pub desc: String,
    pub id: u32
}

impl Room {
    pub fn new(items: ItemList, npcs:NPCList, desc: String, id: u32) -> Room {
        Room {
            items,
            npcs,
            desc,
            id
        }
    }

    pub fn new_empty_room(id: u32) -> Room {
        Room {
            items: ItemList::new(),
            npcs: NPCList::new(),
            desc: "Lorem Ipsum".to_string(),
            id: id
        }
    }

    pub fn get_items(&self) -> &ItemList {
        &self.items
    }

    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn from(&self) -> Room {
        let mut new_items = ItemList::new();
        let mut new_npcs = NPCList::new();
        for i in self.items.get_list().iter(){
            new_items.add(i, None);
        }
        for i in self.npcs.get_list().iter(){
            new_npcs.add(i);
        }
        let result = Room::new(new_items, new_npcs, self.desc.to_string(), self.id);
        result
    }
}
