pub mod actions {

    use item::Item;
    use room::Room;

    pub enum Action {
        PlayerAct(PlayerAction),
        RoomAct(RoomAction),
    }

    pub enum PlayerAction {
        //TODO: Add a talk action so that players can talk to NPC's
        //Talk(String, u32),
        Heal(i32),
        Hurt(i32),
        UpdateInventory(InventoryAction),
        ChangeRoom(Room),
    }

    pub enum NpcAction {
        //TODO: Add a reply action so that NPC's may reply to Players Talk message
        //Reply(String),
        Heal(i32, u32),
        Hurt(i32, u32),
        Kill(u32),
        UpdateInventory(InventoryAction, u32),
    }

    pub enum InventoryAction {
        Add(Item),
        Delete(Item),
        Display(),
    }

    pub enum RoomAction {
        UpdateInventory(InventoryAction, u32),
        UpdateNpcList(NpcAction, u32),
        UpdateDescription(String, u32)
    }
}
