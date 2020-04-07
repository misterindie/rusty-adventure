use player::Player;
use npclist::NPCList;
use roomlist::RoomList;
use itemlist::ItemList;
use room::Room;
use action::actions::*;

pub struct State {
    player: Player,
    npcs: NPCList,
    rooms: RoomList,
    running: bool
}

impl State {
    //make a new player
    pub fn new(player: Player, npcs: NPCList, rooms: RoomList, running: bool) -> State{
        State {
            player: player,
            npcs: npcs,
            rooms: rooms,
            running: running
        }
    }

    //return a reference to the player value of the state
    pub fn get_player(&self) -> &Player {
        return &self.player;
    }

    //return a new empty state
    pub fn empty_state() -> State {
        State{player: Player::new("TBA".to_string(), 100, ItemList::new(), Room::new_empty_room(0)), npcs: NPCList::new(), rooms: RoomList::new(), running: true}
    }

    pub fn running(&self) -> bool {
        return self.running;
    }

    //return a new state based on a reference state
    pub fn from(&self) -> State {
        let mut new_npc_list = NPCList::new();
        let mut new_room_list = RoomList::new();

        for i in self.npcs.get_list().iter(){
            new_npc_list.add(i);
        }

        for i in self.rooms.get_list().iter(){
            new_room_list.add(i);
        }

        let result = State::new(Player::from(&self.player), new_npc_list, new_room_list, self.running);
        return result;
    }

    //This function will perform actions on a clone of the old state and return it to update the state;
    pub fn reduce(&self, action: &Action) -> State {
        //Our new state
        let mut new_state = State::from(&self);

        //We will match the action with actions from our action file and change things accordiningly
        match action {
            //PLAYER ACTIONS
            Action::PlayerAct(ref player_action) => match player_action {
                //Heal the Player
                PlayerAction::Heal(ref amount) => {
                    new_state.player.health += amount;
                },
                //Hurt the Player
                PlayerAction::Hurt(ref amount) => {
                    new_state.player.health -= amount;
                },
                //Update the Player's Inventory
                PlayerAction::UpdateInventory(ref inventory_actions) => match inventory_actions {
                    //Add an Item to the players inventory
                    InventoryAction::Add(ref item) => {
                        new_state.player.inventory.add(item, Some("was added to your Inventory!".to_string()));
                    },
                    //Remove an Item from the players Invnetory
                    InventoryAction::Delete(ref item) => {
                        new_state.player.inventory.delete(item);
                    },
                    InventoryAction::Display() => {
                        new_state.player.inventory.render();
                    }
                },
                //Change the players current room
                PlayerAction::ChangeRoom(ref room) => {
                    new_state.player.current_room = Room::from(room);
                }
            },
            //ROOM ACTIONS
            Action::RoomAct(ref room_action) => match room_action {
                RoomAction::UpdateInventory(ref inventory_actions, ref id) => match inventory_actions {
                    InventoryAction::Add(ref item) => {
                        new_state.rooms.get_mut_list()[new_state.rooms.get_index_of_room(id)].
                        items.add(item, None);
                    },
                    InventoryAction::Delete(ref item) => {
                        new_state.rooms.get_mut_list()[new_state.rooms.get_index_of_room(id)].
                        items.delete(item);
                    },
                    InventoryAction::Display() => {
                        new_state.rooms.get_list()[new_state.rooms.get_index_of_room(id)].items.render();
                    }
                },
                RoomAction::UpdateDescription(ref description, ref room_id) => {
                    new_state.rooms.get_mut_list()[new_state.rooms.get_index_of_room(room_id)].desc = description.to_string();
                },
                RoomAction::UpdateNpcList(ref npc_actions, ref room_id) => match npc_actions {
                    NpcAction::Heal(ref amount, ref npc_id) => {
                        new_state.rooms.get_mut_list()[new_state.rooms.get_index_of_room(room_id)].
                        npcs.get_mut_list()[new_state.npcs.get_index_of_npc(npc_id)].hp += amount;
                    },
                    NpcAction::Hurt(ref amount, ref npc_id) => {
                        new_state.rooms.get_mut_list()[new_state.rooms.get_index_of_room(room_id)].
                        npcs.get_mut_list()[new_state.rooms.get_list()[new_state.rooms.get_index_of_room(room_id)].
                        npcs.get_index_of_npc(npc_id)].hp -= amount;
                    },
                    NpcAction::Kill(ref id) => {
                            new_state.rooms.get_list()[new_state.rooms.get_index_of_room(room_id)].
                            npcs.delete(id);
                    },
                    NpcAction::UpdateInventory(ref inventory_action, ref npc_id) => match inventory_action {
                        InventoryAction::Delete(ref item) => {
                            new_state.rooms.get_list()[new_state.rooms.get_index_of_room(room_id)].
                            npcs.get_mut_list()[new_state.rooms.get_list()[new_state.rooms.get_index_of_room(room_id)].
                            npcs.get_index_of_npc(npc_id)].inventory.delete(item);
                        },
                        InventoryAction::Add(ref item) => {
                            new_state.rooms.get_list()[new_state.rooms.get_index_of_room(room_id)].
                            npcs.get_mut_list()[new_state.npcs.get_index_of_npc(npc_id)].inventory.add(item, None);
                        },
                        InventoryAction::Display() => {
                            new_state.rooms.get_list()[new_state.rooms.get_index_of_room(room_id)].npcs.get_list()[new_state.npcs.get_index_of_npc(npc_id)].inventory.render();
                        }
                    }
                }
            }
        };
        return new_state;
    }
}
