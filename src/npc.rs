use itemlist::ItemList;
#[allow(dead_code)]
pub struct NPC {
    pub name: String,
    pub hp: i32,
    pub inventory: ItemList,
    pub id: u32
}

impl NPC {
    pub fn new(name: String, hp: i32, inventory: ItemList, id: u32) -> NPC {
        NPC {
            name: name,
            hp: hp,
            inventory: inventory,
            id: id
        }
    }

    pub fn from(&self) -> NPC {
        let mut new_items = ItemList::new();
        for i in self.inventory.get_list().iter(){
            new_items.add(i, None);
        }
        let result = NPC::new(self.name.to_string(), self.hp, new_items, self.id);
        result
    }

    pub fn get_id(&self) -> &u32 {
        return &self.id;
    }
}
