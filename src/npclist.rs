use npc::NPC;

pub struct NPCList {
    list: Vec<NPC>
}

impl NPCList {
    pub fn new() -> NPCList {
        NPCList{ list: vec![] }
    }

    pub fn add(&mut self, npc: &NPC) {
        self.list.push(NPC::from(npc));
    }

    pub fn from(npcs: &[&NPC]) -> NPCList {
        let mut result = Self::new();
        for npc in npcs {
            result.add(npc);
        }
        result
    }
    //TODO: delete a npc from the list
    pub fn delete(&self, id: &u32) {
        println!("{}",id );
    }

    pub fn get_list(&self) -> &Vec<NPC>{
        &self.list
    }

    pub fn get_mut_list(&self) -> Vec<NPC>{
        let mut new_list = Self::new();
        for npc in self.get_list().iter() {
            new_list.add(npc);
        }
        return new_list.list;
    }

    pub fn get_index_of_npc(&self, id: &u32) -> usize {
        let index = self.get_list().iter().position(|ref i| i.get_id() == id).unwrap();
        return index;
    }
}
