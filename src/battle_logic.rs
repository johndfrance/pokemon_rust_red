/*
Title: battle_logic.rs

Desc: Contains the core engine for battles. Currently only for trainer to trainer battles.
 */
use crate::Status::{Asleep, Burned, Fainted, Healthy, Poisoned};
use crate::{
    enemy_move_select, read_user_input, type_text, GameState, PartyOperations, Pokemon, Trainer,
};
use colored::Colorize;
use std::cmp::Ordering;
use std::{io, thread};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use crate::battle_logic::MainMenuOptions::Fight;
use crate::lib::{CERULEAN, get_user_input};
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
            "\n{} sends out {}\n",
            enemy_name, enemy.poke_team[enemy_starting_mon_index].name
        )
        .as_str(),
    );
    type_text(
        format!(
            "\n{} sends out {}\n",
            player_name,
            game_state.player.party.mon[player_starting_mon_index]
                .clone()
                .unwrap()
                .name
        )
        .as_str(),
    );
    game_state.player.party.mon[player_starting_mon_index.clone()].as_mut().unwrap().battle_stats_reset();
    let mut player_mon_index = player_starting_mon_index.clone();
    let mut enemy_mon_index = enemy_starting_mon_index.clone();

    loop {
        // At the start of each turn, check if the opponent fainted on the last turn.
        if enemy.poke_team[enemy_mon_index.clone()].status == Fainted {
            //println!("ENEMY POKEMON AT INDEX {} IS FAINTED", enemy_mon_index);
            println!();
            game_state.player.party.mon[player_mon_index.clone()]
                .as_mut()
                .unwrap()
                .gain_exp(&enemy.poke_team[enemy_mon_index.clone()]);
            // If the opponent fainted then check if they have any healthy pokemon remaining.
            //println!("NOW CHECKING IF ENEMY HAS REMAINING POKEMON");
            let enemy_not_all_fainted = enemy.check_all_fainted();
            if enemy_not_all_fainted {
                //println!("ENEMY DOES");
                // If they do, get the index for the first healthy pokemon, and send it out.
                let next_healthy_index =
                    enemy.return_first_healthy().expect("Somethings Gone Wrong");
                //println!("NEXT HEALTHY POKEMON AT {} INDEX", next_healthy_index);
                enemy_mon_index = next_healthy_index;
                type_text(
                    format!(
                        "\n{} sends out {}\n",
                        enemy_name, enemy.poke_team[enemy_mon_index].name
                    )
                    .as_str(),
                );
            }
            // If there are no more healthy pokemon remaining, then end the loop and move to 'post-battle' section.
            else {
                //println!("ENEMY DOES NOT");
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
                game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap().battle_stats_reset();
            } else {
                winner = false;
                break;
            }
        }
        // Presents both fighting Pokemon with name, level, and HP info
        println!("********************************");
        print!("\n{}  ", "Player".blue());
        battle_display_names(&game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap(),);
        print!("\n{}  ", "Enemy".red());
        battle_display_names(&enemy.poke_team[enemy_mon_index.clone()]);


        /*
        io::stdout().flush().unwrap();
        let mut valid_move_picked = false;
        let mut player1_input;
        player1_input = read_user_input();
        let player1_input = player1_input.as_str();
         */

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


        let mut player_selected_move: Moves = Moves::Agility;
        let (menu_choice, sub_menu_choice) = (MainMenuOptions::Fight, Some(0));

        loop {
            let (menu_choice, sub_menu_choice) = battle_display_menu(&game_state, player_mon_index.clone());
            match menu_choice {
                MainMenuOptions::Fight => {
                    player_selected_move = match sub_menu_choice.unwrap() {
                        1 => { game_state.player.party.mon[player_mon_index.clone()].clone().unwrap().moves[0] }
                        2 => { game_state.player.party.mon[player_mon_index.clone()].clone().unwrap().moves[1] }
                        3 => { game_state.player.party.mon[player_mon_index.clone()].clone().unwrap().moves[2] }
                        4 => { game_state.player.party.mon[player_mon_index.clone()].clone().unwrap().moves[3] }
                        _ => unreachable!()
                    };
                    break;
                },
                MainMenuOptions::Item => { println!("Can't capture a pokemon that already has a Trainer!") }
                MainMenuOptions::Change => {
                    player_mon_index = sub_menu_choice.unwrap() as usize;
                    println!("\nPlayer sends out {}\n", game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap().name);
                    enemy_attacks(game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap(), &mut enemy.poke_team[enemy_mon_index.clone()], enemy_move_selection);
                }
                MainMenuOptions::Run => { println!("Can't run from a battle with another Trainer!") }
            }
        }

        //println!("YOUVE SELECTED MOVE: {}", player_selected_move.move_stats().name);

        //println!("ENEMY HAS SELECTED MOVE: {}", enemy_move_selection.move_stats().name);
        let speed_order = game_state.player.party.mon[player_mon_index.clone()]
            .clone()
            .unwrap()
            .spd
            .value
            .cmp(&enemy.poke_team[enemy_mon_index.clone()].spd.value);
        if menu_choice == Fight {


            //println!("DEBUG SPEED: {:?}", speed_order);
            match speed_order {
                Ordering::Greater => {

                        player_attacks(game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap(), &mut enemy.poke_team[enemy_mon_index.clone()], player_selected_move);

                    //println!("WORKED");
                    /*
                type_text(format!("\n{} used {}!\n",
                         &game_state.player.party.mon[player_mon_index].as_ref().unwrap().name,
                         player_selected_move.move_stats().name).as_str());
                enemy.poke_team[enemy_mon_index.clone()].damage(
                    &game_state.player.party.mon[player_mon_index.clone()]
                        .clone()
                        .unwrap(),
                    &player_selected_move,
                );

                 */
                    if enemy.poke_team[enemy_mon_index.clone()].current_hp == 0 {
                        type_text("Enemy Fainted!\n");
                    } else {
                        thread::sleep(Duration::from_millis(500));
                        enemy_attacks(game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap(), &mut enemy.poke_team[enemy_mon_index.clone()], enemy_move_selection);
                        /*
                        type_text(
                            format!("\n{} used {}\n",
                                    enemy.poke_team[enemy_mon_index].name,
                                    enemy_move_selection.move_stats().name).as_str());

                        game_state.player.party.mon[player_mon_index.clone()]
                            .as_mut()
                            .unwrap()
                            .damage(
                                &enemy.poke_team[enemy_mon_index.clone()],
                                &enemy_move_selection,
                            );

                         */
                        if game_state.player.party.mon[player_mon_index.clone()]
                            .clone()
                            .unwrap()
                            .current_hp
                            == 0
                        {
                            type_text("You Fainted!\n");
                        }
                    }
                }
                Ordering::Less => {
                    enemy_attacks(game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap(), &mut enemy.poke_team[enemy_mon_index.clone()], enemy_move_selection);
                    /*
                    type_text(
                        format!("\n{} used {}!\n",
                                enemy.poke_team[enemy_mon_index].name,
                                enemy_move_selection.move_stats().name).as_str());
                    game_state.player.party.mon[player_mon_index.clone()]
                        .as_mut()
                        .unwrap()
                        .damage(
                            &enemy.poke_team[enemy_mon_index.clone()],
                            &enemy_move_selection,
                        );

                     */
                    if game_state.player.party.mon[player_mon_index.clone()]
                        .clone()
                        .unwrap()
                        .current_hp
                        == 0
                    {
                        type_text("You Fainted!\n");
                    } else {
                        thread::sleep(Duration::from_millis(500));
                        player_attacks(game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap(), &mut enemy.poke_team[enemy_mon_index.clone()], player_selected_move);
                        /*
                        type_text(
                            format!("\n{} used {}!\n",
                                    game_state.player.party.mon[player_mon_index].as_ref().unwrap().name,
                                    player_selected_move.move_stats().name).as_str());
                        enemy.poke_team[enemy_mon_index.clone()].damage(
                            &game_state.player.party.mon[player_mon_index.clone()]
                                .clone()
                                .unwrap(),
                            &player_selected_move,
                        );

                         */
                        if enemy.poke_team[enemy_mon_index.clone()].current_hp == 0 {
                            type_text("Enemy Fainted!\n");
                        }
                    }
                }
                Ordering::Equal => { //TODO: This should be 50/50 but for now I have it favour the player.
                    player_attacks(game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap(), &mut enemy.poke_team[enemy_mon_index.clone()], player_selected_move);
                    /*
                    type_text(
                        format!("{} used {}!\n",
                                game_state.player.party.mon[player_mon_index].as_ref().unwrap().name,
                                player_selected_move.move_stats().name).as_str());

                    enemy.poke_team[enemy_mon_index.clone()].damage(
                        &game_state.player.party.mon[player_mon_index.clone()]
                            .clone()
                            .unwrap(),
                        &player_selected_move,
                    );

                     */
                    if enemy.poke_team[enemy_mon_index.clone()].current_hp == 0 {
                        type_text("Enemy Fainted!\n");
                    } else {
                        thread::sleep(Duration::from_millis(500));
                        enemy_attacks(game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap(), &mut enemy.poke_team[enemy_mon_index.clone()], enemy_move_selection);
                        /*
                        type_text(
                            format!("\n{} used {}!\n",
                                    enemy.poke_team[enemy_mon_index].name,
                                    enemy_move_selection.move_stats().name).as_str());
                        game_state.player.party.mon[player_mon_index.clone()]
                            .as_mut()
                            .unwrap()
                            .damage(
                                &enemy.poke_team[enemy_mon_index.clone()],
                                &enemy_move_selection,
                            );
                         */
                        if game_state.player.party.mon[player_mon_index.clone()]
                            .clone()
                            .unwrap()
                            .current_hp
                            == 0
                        {
                            type_text("You Fainted!\n");
                        }
                    }
                }
            }
            // Status Damage
            if enemy.poke_team[enemy_mon_index.clone()].current_hp != 0 {
                //Leech Seed
                if enemy.poke_team[enemy_mon_index.clone()].special_conditions.leech_seeded == true {
                    enemy.poke_team[enemy_mon_index.clone()].leech_seed_effect(game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap());
                }
                //Burn or Poison Damage
                if enemy.poke_team[enemy_mon_index.clone()].status == Burned ||  enemy.poke_team[enemy_mon_index.clone()].status == Poisoned{
                    enemy.poke_team[enemy_mon_index.clone()].burn_poison_effect();
                }
            }
            if game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap().current_hp != 0 {
                //Leech Seed
                if game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap().special_conditions.leech_seeded == true {
                    game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap().leech_seed_effect(&mut enemy.poke_team[enemy_mon_index.clone()]);
                }
                //Burn or Poison Damage
                if game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap().status == Burned || game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap().status == Poisoned{
                    game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap().burn_poison_effect();
                }
            }
        }
    }
    game_state.player.party.status_reset();
    if winner{
        println!("\n{}", "You won the Battle!".green());
        game_state.set_trainer_defeated(enemy.id.clone());
    }else{
        println!("\n{}", "You lost the Battle!".red())
    }
    return winner
}

fn enemy_attacks(player: &mut Pokemon, enemy: &mut Pokemon, selected_move: Moves){
    if enemy.status == Asleep{
        let random_number = rand::thread_rng().gen_range(0..=4);
        if random_number == 0{
            enemy.status == Healthy;
            type_text(format!("\n{} woke up!\n", enemy.name.clone().color(CERULEAN)).as_str());
        }
    }
    if enemy.status == Asleep{
        type_text(format!("\n{} is asleep and can't fight!\n", enemy.name.clone().color(CERULEAN)).as_str());
    }else {
        type_text(
            format!("{} used {}!\n",
                    enemy.name,
                    selected_move.move_stats().name).as_str());
        player.damage(&enemy, &selected_move);
    }
}
fn player_attacks(player: &mut Pokemon, enemy: &mut Pokemon, selected_move: Moves){
    if player.status == Asleep{
        let random_number = rand::thread_rng().gen_range(0..=4);
        if random_number == 0{
            player.status == Healthy;
            type_text(format!("\n{} woke up!\n", player.name.clone().color(CERULEAN)).as_str());
        }
    }
    if player.status == Asleep{
        type_text(format!("\n{} is asleep and can't fight!\n", player.name.clone().color(CERULEAN)).as_str());
    }else {
        type_text(
            format!("{} used {}!\n",
                    player.name,
                    selected_move.move_stats().name).as_str());
        enemy.damage(&player, &selected_move, );
    }
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
            "{}(LVL {}) has {}/{}HP",
            mon.name.to_string().cyan(),
            mon.level.to_string().cyan(),
            current_hp_for_display,
            mon.max_hp.value.to_string().green()
        )
    );
}
#[derive(PartialEq)]
pub enum MainMenuOptions{
    Fight,
    Item,
    Change,
    Run,
}
pub fn battle_display_menu(game_state: &GameState, poke_index: usize)->(MainMenuOptions, Option<u8>){
    println!("\nYour Turn! What will you do?");
    println!("1.Fight 2.Items\n3.Swap 4.Run");
    //println!("{}", "Items, Swap, and Run and not yet built for Trainer Battles\nSwap is not built for Wild Battles".red());
    let menu_selection = get_user_input(4);
    match menu_selection{
        1=>{ //FIGHT
            let mut move_count = 1;
            println!("\nFight - Select your Move:");
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

            return (MainMenuOptions::Item, None);
        },
        3=>{
            println!("Which pokemon would you like to swap for:");
            let mut counter = 1;
            for pokemon in &game_state.player.party.mon{
                if *pokemon != None {
                    println!("{}. {} ({:?})", counter, pokemon.as_ref().unwrap().name, pokemon.as_ref().unwrap().status);
                    counter += 1;
                }
            }
            let mut selection:u8;
            loop {
                selection = get_user_input(counter.clone())-1;

                if game_state.player.party.mon[selection as usize].as_ref().unwrap().status != Fainted{
                    break
                }else{
                    println!("That Pokemon cannot fight, select another!");
                }
            }
            return (MainMenuOptions::Change, Some(selection.clone()));
        },
        4=>{
            //println!("RUNNING NOT YET IMPLIMENTED");
            return(MainMenuOptions::Run, None);
        },
        _=>unreachable!(),
    }
}
