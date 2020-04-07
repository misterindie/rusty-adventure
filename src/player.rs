use itemlist::ItemList;
use room::Room;

pub struct Player {
    pub name: String,
    pub health: i32,
    pub inventory: ItemList,
    pub current_room: Room
}

impl Player {
    pub fn new(name: String, health: i32, items: ItemList, room: Room ) -> Player {
        Player {
            name: name,
            health: health,
            inventory: items,
            current_room: room,
        }
    }

    pub fn from(&self) -> Player {
        let mut new_items = ItemList::new();
        for i in self.inventory.get_list().iter() {
            new_items.add(i, None);
        }
        let result = Player::new(self.name.to_string(), self.health, new_items, Room::from(&self.current_room));
        result
    }

    pub fn get_health(&self) -> &i32 {
        return &self.health;
    }

    pub fn get_inventory(&self) -> &ItemList {
        return &self.inventory;
    }
}
