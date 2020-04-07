use room::Room;

pub struct RoomList {
    rooms: Vec<Room>
}

impl RoomList {
    pub fn new() -> RoomList {
        RoomList{rooms: vec![]}
    }

    pub fn add(&mut self, room: &Room) {
        self.rooms.push(Room::from(room));
    }

    pub fn get_list(&self) -> &Vec<Room> {
        &self.rooms
    }
    pub fn get_mut_list(&self) -> Vec<Room>{

        let mut new_list = Self::new();

        for  room in self.get_list().iter() {
            new_list.add(room);
        }
        return new_list.rooms;
    }

    pub fn get_index_of_room(&self, id: &u32) -> usize {
        let index = self.get_list().iter().position(|ref i| i.get_id() == id).unwrap();
        return index;
    }
}
