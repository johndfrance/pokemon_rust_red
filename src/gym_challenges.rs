use crate::battle_logic::battle2;
use crate::enemy_trainers::Trainer;
use crate::{GameState, type_text};

pub fn viridian_gym(game_state: &mut GameState){
    println!();
    let mut first_trainer = Trainer::get(11);
    let result = battle2(game_state, &mut first_trainer);
    if result{
        let mut gym_leader = Trainer::get(501);
        let result = battle2(game_state, &mut gym_leader);
        if result {
            type_text("You've beat Gym Leader Brock!\n He gives you the BOULDER BADGE!");
        }else {
            game_state.move_loc(game_state.last_used_pcentre);
            game_state.player.party.pokecentre_heal();
        }
    }else{
        game_state.move_loc(game_state.last_used_pcentre);
        game_state.player.party.pokecentre_heal();
    }


}