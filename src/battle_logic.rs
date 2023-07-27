/*
Title: battle_logic.rs

Desc: Contains the core engine for battles. Currently only for trainer to trainer battles.
 */
use crate::Status::Fainted;
use crate::{
    enemy_move_select, read_user_input, type_text, GameState, PartyOperations, Pokemon, Trainer,
};

use colored::Colorize;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use crate::lib::get_user_input;
use crate::move_data::Moves;

pub fn battle2(game_state: &mut GameState, enemy: &mut Trainer)-> bool {
    let mut winner = false;
    // Creating Aliases for easier access later
    let player_name = &game_state.player.name;
    let enemy_name = enemy.name;

    type_text(format!("\n{} has challenged you to a battle!\n", enemy_name.red()).as_str());

    let player_starting_mon_index = game_state
        .player
        .party
        .return_first_healthy()
        .expect("Somethings Gone Wrong");
    let enemy_starting_mon_index = enemy.return_first_healthy().expect("Somethings Gone Wrong");

    type_text(
        format!(
            "{} sends out {}\n",
            enemy_name, enemy.poke_team[enemy_starting_mon_index].name
        )
        .as_str(),
    );
    type_text(
        format!(
            "{} sends out {}\n",
            player_name,
            game_state.player.party.mon[player_starting_mon_index]
                .clone()
                .unwrap()
                .name
        )
        .as_str(),
    );
    let mut player_mon_index = player_starting_mon_index.clone();
    let mut enemy_mon_index = enemy_starting_mon_index.clone();

    loop {

        // At the start of each turn, check if the opponent fainted on the last turn.
        if enemy.poke_team[enemy_mon_index.clone()].status == Fainted {
            println!("ENEMY POKEMON AT INDEX {} IS FAINTED", enemy_mon_index);
            game_state.player.party.mon[player_mon_index.clone()]
                .as_mut()
                .unwrap()
                .gain_exp(&enemy.poke_team[enemy_mon_index.clone()]);
            // If the opponent fainted then check if they have any healthy pokemon remaining.
            println!("NOW CHECKING IF ENEMY HAS REMAINING POKEMON");
            let enemy_not_all_fainted = enemy.check_all_fainted();
            if enemy_not_all_fainted {
                println!("ENEMY DOES");
                // If they do, get the index for the first healthy pokemon, and send it out.
                let next_healthy_index =
                    enemy.return_first_healthy().expect("Somethings Gone Wrong");
                println!("NEXT HEALTHY POKEMON AT {} INDEX", next_healthy_index);
                enemy_mon_index = next_healthy_index;
                type_text(
                    format!(
                        "{} sends out {}\n",
                        enemy_name, enemy.poke_team[enemy_mon_index].name
                    )
                    .as_str(),
                );
            }
            // If there are no more healthy pokemon remaining, then end the loop and move to 'post-battle' section.
            else {
                println!("ENEMY DOES NOT");
                winner = true;
                break;
            }
        }
        if game_state.player.party.mon[player_mon_index.clone()]
            .as_ref()
            .unwrap()
            .status
            == Fainted
        {
            let player_not_all_fainted = game_state.player.party.check_all_fainted();
            if player_not_all_fainted {
                // TODO In future the player should be allowed to choice which pokemon they want to send in.
                let next_healthy_index = game_state
                    .player
                    .party
                    .return_first_healthy()
                    .expect("Somethings Gone Wrong");
                player_mon_index = next_healthy_index;
                type_text(
                    format!(
                        "{} sends out {}\n",
                        player_name,
                        game_state.player.party.mon[player_mon_index]
                            .clone()
                            .unwrap()
                            .name
                    )
                    .as_str(),
                );
            } else {
                winner = false;
                break;
            }
        }
        // Presents both fighting Pokemon with name, level, and HP info
        battle_display_names(&game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap(),);
        battle_display_names(&enemy.poke_team[enemy_mon_index.clone()]);

        let (menu_choice, sub_menu_choice) = battle_display_menu(&game_state, player_mon_index.clone());

        /*
        io::stdout().flush().unwrap();
        let mut valid_move_picked = false;
        let mut player1_input;
        player1_input = read_user_input();
        let player1_input = player1_input.as_str();
         */

        let mut player_selected_move: Moves;
        match menu_choice{
            MainMenuOptions::Fight =>{
                player_selected_move = match sub_menu_choice.unwrap() {
                    1 => { game_state.player.party.mon[player_mon_index.clone()].clone().unwrap().moves[0] }
                    2 => { game_state.player.party.mon[player_mon_index.clone()].clone().unwrap().moves[1] }
                    3 => { game_state.player.party.mon[player_mon_index.clone()].clone().unwrap().moves[3] }
                    4 => { game_state.player.party.mon[player_mon_index.clone()].clone().unwrap().moves[4] }
                    _ => unreachable!()
                }
            },
            MainMenuOptions::Item=>{todo!()}
            MainMenuOptions::Change=>{todo!()}
            MainMenuOptions::Run=>{todo!()}
        }

        println!("YOUVE SELECTED MOVE: {}", player_selected_move.move_stats().name);


        let enemy_move_selection =
            enemy_move_select(&enemy.poke_team[enemy_mon_index.clone()]).to_string();
        let enemy_move_selection = enemy_move_selection.as_str();
        let enemy_move_selection = match enemy_move_selection {
            "1" => enemy.poke_team[enemy_mon_index.clone()].moves[0],
            "2" => enemy.poke_team[enemy_mon_index.clone()].moves[1],
            "3" => enemy.poke_team[enemy_mon_index.clone()].moves[2],
            "4" => enemy.poke_team[enemy_mon_index.clone()].moves[3],
            _ => enemy.poke_team[enemy_mon_index.clone()].moves[0],
        };
        println!("ENEMY HAS SELECTED MOVE: {}", enemy_move_selection.move_stats().name);

        let speed_order = game_state.player.party.mon[player_mon_index.clone()]
            .clone()
            .unwrap()
            .spd
            .value
            .cmp(&enemy.poke_team[enemy_mon_index.clone()].spd.value);

        println!("DEBUG SPEED: {:?}", speed_order);

        match speed_order {
            Ordering::Greater => {
                println!("{} used {}!",
                         &game_state.player.party.mon[player_mon_index].as_ref().unwrap().name,
                         player_selected_move.move_stats().name);
                enemy.poke_team[enemy_mon_index.clone()].damage(
                    &game_state.player.party.mon[player_mon_index.clone()]
                        .clone()
                        .unwrap(),
                    &player_selected_move,
                );
                if enemy.poke_team[enemy_mon_index.clone()].current_hp == 0 {
                    type_text("Enemy Fainted!");
                } else {
                    println!("{} used {}",
                             enemy.poke_team[enemy_mon_index].name,
                             enemy_move_selection.move_stats().name);

                    game_state.player.party.mon[player_mon_index.clone()]
                        .as_mut()
                        .unwrap()
                        .damage(
                            &enemy.poke_team[enemy_mon_index.clone()],
                            &enemy_move_selection,
                        );
                    if game_state.player.party.mon[player_mon_index.clone()]
                        .clone()
                        .unwrap()
                        .current_hp
                        == 0
                    {
                        type_text("You Fainted!");
                    }
                }
            }
            Ordering::Less => {
                println!("{} used {}",
                         enemy.poke_team[enemy_mon_index].name,
                         enemy_move_selection.move_stats().name);
                game_state.player.party.mon[player_mon_index.clone()]
                    .as_mut()
                    .unwrap()
                    .damage(
                        &enemy.poke_team[enemy_mon_index.clone()],
                        &enemy_move_selection,
                    );
                if game_state.player.party.mon[player_mon_index.clone()]
                    .clone()
                    .unwrap()
                    .current_hp
                    == 0
                {
                    type_text("You Fainted!");
                } else {
                    println!("{} used {}!",
                             game_state.player.party.mon[player_mon_index].as_ref().unwrap().name,
                             player_selected_move.move_stats().name);
                    enemy.poke_team[enemy_mon_index.clone()].damage(
                        &game_state.player.party.mon[player_mon_index.clone()]
                            .clone()
                            .unwrap(),
                        &player_selected_move,
                    );
                    if enemy.poke_team[enemy_mon_index.clone()].current_hp == 0 {
                        type_text("Enemy Fainted!");
                    }
                }
            }
            Ordering::Equal => { //TODO: This should be 50/50 but for now I have it favour the player.
                println!("{} used {}!",
                         game_state.player.party.mon[player_mon_index].as_ref().unwrap().name,
                         player_selected_move.move_stats().name);

                enemy.poke_team[enemy_mon_index.clone()].damage(
                    &game_state.player.party.mon[player_mon_index.clone()]
                        .clone()
                        .unwrap(),
                    &player_selected_move,
                );
                if enemy.poke_team[enemy_mon_index.clone()].current_hp == 0 {
                    type_text("Enemy Fainted!");
                } else {
                    println!("{} used {}",
                             enemy.poke_team[enemy_mon_index].name,
                             enemy_move_selection.move_stats().name);
                    game_state.player.party.mon[player_mon_index.clone()]
                        .as_mut()
                        .unwrap()
                        .damage(
                            &enemy.poke_team[enemy_mon_index.clone()],
                            &enemy_move_selection,
                        );
                    if game_state.player.party.mon[player_mon_index.clone()]
                        .clone()
                        .unwrap()
                        .current_hp
                        == 0
                    {
                        type_text("You Fainted!");
                    }
                }
            }
        }
    }
    return winner
}

pub fn battle_display_names(mon: &Pokemon) {
    let mut current_hp = mon.current_hp.clone().to_string();
    let mut current_hp_for_display;
    if mon.current_hp > (&mon.max_hp.value / 2) {
        current_hp_for_display = current_hp.green();
    } else if mon.current_hp > (&mon.max_hp.value / 5) && mon.current_hp <= (&mon.max_hp.value / 2)
    {
        current_hp_for_display = current_hp.yellow();
    } else {
        current_hp_for_display = current_hp.red();
    }
    println!(
        "{}",
        format!(
            "\n{}(LVL {}) has {}/{}HP",
            mon.name.to_string().cyan(),
            mon.level.to_string().cyan(),
            current_hp_for_display,
            mon.max_hp.value.to_string().green()
        )
    );
}
pub enum MainMenuOptions{
    Fight,
    Item,
    Change,
    Run,
}
pub fn battle_display_menu(game_state: &GameState, poke_index: usize)->(MainMenuOptions, Option<u8>){
    println!("\nYour Turn! What will you do?");
    println!("1.Fight 2.Items\n3.Poke 4.Run");
    let menu_selection = get_user_input(4);
    match menu_selection{
        1=>{ //FIGHT
            let mut move_count = 1;
            println!("Fight - Select your Move:");
            for moves in &game_state.player.party.mon[poke_index.clone()]
                .clone()
                .unwrap()
                .moves
            {
                println!("{}. {} ", move_count, moves.move_stats().name);
                move_count += 1;
            }
            let move_selection = get_user_input(move_count);
            return (MainMenuOptions::Fight, Some(move_selection));
        },
        2=>{
            println!("ITEMS NOT YET IMPLIMENTED");
            return (MainMenuOptions::Item, None);
        },
        3=>{
            println!("SWAPPING NOT YET IMPLIMENTED");
            return (MainMenuOptions::Change, None);
        },
        4=>{
            println!("RUNNING NOT YET IMPLIMENTED");
            return(MainMenuOptions::Run, None);
        },
        _=>unreachable!(),
    }
}

/*
Output: IF Trainer battle EITHER {get money, game continues}
OR {lose money, change location to last (nearest?) PokeCentre, Restore all pokemon to  full health}
IF Wild battle EITHER {game continues}, {game continues, new pokemon added},
OR  {... All fainted condition}

Input: GameState, Enemy(EITHER Trainer or Wild Pokemon)

Process:
    Initial Considerations:

    -Battle will need to initialize some sort of temporary stats set to track modifications
    that are the result of moves, so this can be factored into the damage. Maybe this just needs to
    be part of the stats of the pokemon? Otherwise the DAMAGE function will need to intake this number as well.

    -Ideally there will be one function that can deal with both Trainer and Wild Battles. I hope to
    accomplish this with the use of Generics and Traits.

    -The first not fainted pokemon will be sent out first.
        -fn is_fainted(){}
    -When Pokemon's HP = 0 there status will turn to Fainted.
        -In impl Pokemon fn check_is_fainted(){} that will check if a Pokemon's HP is 0
        and if it is will turn status to Fainted.
        -That will be run within the Damage fn to ensure it's not missed.

     - fn battle () should probably not return anything, just call another function
     to deal with the fainting case, reviece_money(), or catch(). It could still return a bool
     but I just don't need to catch it if I'm not using it.

     -I'll probably need a fn wild_encounter() in which I make calls that deal with what wild mon
     are in what regions of the map, and then this function calls battle() once it picks which
     pokemon the trainer is battling.
        -This means I will need a way to store what pokemon are in what region.
        -fuck, more data handling.

    Should deal better with the Trainer::new()

        -Pokemon should be able to swap mid fight
        -items.
        -running.

 */
