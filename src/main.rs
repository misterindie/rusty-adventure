    //Imports
pub mod npc;
pub mod npclist;
pub mod item;
pub mod itemlist;
pub mod room;
pub mod roomlist;
pub mod player;
pub mod action;

pub mod state;

//Imports were using in this file
use std::io;
use std::io::prelude::*;
use state::State;
use action::actions::*;
use item::Item;


#[allow(dead_code)]
fn main() {

    println!("Text Adventure");

    //Inititalize the game state
    let mut game_state = init();

    //The main game loop
    while game_state.running() {
        let command: String = input();
        let processed_command: Vec<String> = process_command(command);
        let action = get_action(&processed_command);
        let new_state = game_state.reduce(&action);
        game_state = new_state;
        game_state.get_player().get_inventory().render();
        if game_state.running() {
            break
        }
    }
}

fn input() -> String {
    print!("Enter a Command: ");
    io::stdout().flush().unwrap();
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line");
    return command;
}

fn get_action(seperated_command: &Vec<String>) -> Action {
    let test_item:Item = Item::new("test".to_string(), "test".to_string(), 10);
    return Action::PlayerAct(PlayerAction::UpdateInventory(InventoryAction::Add(test_item)));
}

fn process_command(cmd: String) ->  Vec<String> {
    let mut split_command: Vec<String> = Vec::new();
    for i in cmd.trim().split(' ') {
        split_command.push(i.to_string());
    }
    return split_command;
}

fn init() -> State {
    let beggining_state = State::empty_state();
    return beggining_state;
}
