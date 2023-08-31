use std::thread;
use std::time::Duration;
use crate::battle_logic::battle2;
use crate::enemy_trainers::Trainer;
use crate::{GameState, type_text};
use crate::game::master_menu;
use crate::lib::get_user_input;

pub fn viridian_gym(game_state: &mut GameState){
    println!();
    if game_state.badge_box.boulder == false {
        let result = game_state.trainer_battle(11);
        if result {
            type_text("\nTRAINER: You may have been able to beat me, but you'll never be able to be beat BROCK!\n");
            loop {
                println!("\nWhat will you do:");
                println!("1. Fight on");
                println!("2. Return to door\n");
                let choice = get_user_input(9);
                match choice {
                    1 => {
                        type_text("\nBROCK: Let me show you how a real Rock Star fights!\n");
                        let result = game_state.trainer_battle(501);
                        if result {
                            type_text("\nYou've beat Gym Leader Brock!\n He gives you the BOULDER BADGE!\n");
                            thread::sleep(Duration::from_millis(1600));
                            type_text("\nBROCK: You've got real talent kid! I'm looking forward to seeing what you make of it!\n");
                            game_state.badge_box.boulder = true;
                            return
                        } else {
                            game_state.move_loc(game_state.last_used_pcentre);
                            game_state.player.party.pokecentre_heal();
                            return
                        }
                    }
                    2 => {
                        return
                    }
                    9 => {
                        println!("OPENING MENU");
                        master_menu(game_state);
                        continue;
                    }
                    _ => println!("Invalid Choice")
                }
            }
        } else {
            game_state.move_loc(game_state.last_used_pcentre);
            game_state.player.party.pokecentre_heal();
        }
    }else{
        type_text("BROCK: You've already beat me, you should head east and challenge MISTY in Cerulean City")
    }
}

fn cerulean_gym(mut game_state: &mut GameState){
    if game_state.badge_box.cascade == false{

    }
}