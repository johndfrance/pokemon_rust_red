use std::cmp::Ordering;
use std::thread;
use std::time::Duration;
use colored::Colorize;
use crossterm::style::Color::{Blue, Green, Red};
use crossterm::style::Stylize;
use rand::Rng;
use crate::{enemy_move_select, GameState, PartyOperations, Pokemon, type_text};
use crate::battle_logic::{battle_display_menu, battle_display_names, MainMenuOptions};
use crate::battle_logic::MainMenuOptions::Fight;
use crate::move_data::Moves;
use crate::Status::{Burned, Fainted, Poisoned};

pub fn wild_encounter(game_state: &mut GameState, wild_mon: &mut Pokemon)->bool{
    let mut winner ;
    let player_name = &game_state.player.name;
    type_text(format!("\nYou are attacked by a wild {}", wild_mon.name).as_str());

    let player_starting_mon_index = game_state
        .player
        .party
        .return_first_healthy()
        .expect("Somethings Gone Wrong");

    type_text(format!("\n{} sends out {}\n",
            player_name,
            game_state.player.party.mon[player_starting_mon_index].clone().unwrap().name).as_str(),);
    game_state.player.party.mon[player_starting_mon_index.clone()].as_mut().unwrap().battle_stats_reset();
    let mut player_mon_index = player_starting_mon_index.clone();

    loop{
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
                        "\n{} sends out {}\n",
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
        println!("*******************************");
        print!("\n{}  ", "Player".stylize().blue());
        battle_display_names(&game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap(),);
        print!("\n{}  ", "Wild".stylize().red());
        battle_display_names(wild_mon);

        let (menu_choice, sub_menu_choice) = battle_display_menu(&game_state, player_mon_index.clone());
        let wild_mon_move_select = enemy_move_select(wild_mon);
        let wild_mon_move_select = wild_mon_move_select.to_string();
        let wild_mon_move_select = wild_mon_move_select.as_str();
        let wild_mon_move_select = match wild_mon_move_select {
            "1"=>wild_mon.moves[0],
            "2"=>wild_mon.moves[1],
            "3"=>wild_mon.moves[2],
            "4"=>wild_mon.moves[3],
            _=>unreachable!(),
        };

        let mut player_selected_move: Moves = Moves::Agility;

        match menu_choice{
            MainMenuOptions::Fight =>{
                player_selected_move = match sub_menu_choice.unwrap() {
                    1 => { game_state.player.party.mon[player_mon_index.clone()].clone().unwrap().moves[0] }
                    2 => { game_state.player.party.mon[player_mon_index.clone()].clone().unwrap().moves[1] }
                    3 => { game_state.player.party.mon[player_mon_index.clone()].clone().unwrap().moves[2] }
                    4 => { game_state.player.party.mon[player_mon_index.clone()].clone().unwrap().moves[3] }
                    _ => unreachable!()
                }
            },
            MainMenuOptions::Item=>{
                println!("Threw a pokeball!");
                let random_number = rand::thread_rng().gen_range(0..=1);
                if random_number == 0 {
                    game_state.player.party.add_party_member(wild_mon.clone());
                    winner = true;
                    type_text(format!("You caught wild {}\n", wild_mon.name.clone().stylize().green()).as_str());
                    thread::sleep(Duration::from_millis(600));
                    break
                }else{
                    type_text("Shoot! The wild pokemon broke free!\n");
                    thread::sleep(Duration::from_millis(600));
                }

            }
            MainMenuOptions::Change=>{
                player_mon_index = sub_menu_choice.unwrap() as usize + 1;
                type_text(format!("\nPlayer sends out {}\n", game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap().name).as_str());
                type_text(format!("\n{} used {}\n",
                         wild_mon.name,
                         wild_mon_move_select.move_stats().name).as_str());

                game_state.player.party.mon[player_mon_index.clone()]
                    .as_mut()
                    .unwrap()
                    .damage(
                        &wild_mon,
                        &wild_mon_move_select,
                    );
            }
            MainMenuOptions::Run=>{
                println!("You ran!");
                winner=true;
                break}
        }
        if menu_choice == Fight{
        //println!("YOUVE SELECTED MOVE: {}", player_selected_move.move_stats().name);


        //println!("WILD HAS SELECTED: {}", wild_mon_move_select.move_stats().name);
        let speed_order = game_state.player.party.mon[player_mon_index.clone()]
            .clone()
            .unwrap()
            .spd
            .value
            .cmp(&wild_mon.spd.value);

        //println!("DEBUG SPEED: {:?}", speed_order);

        match speed_order{
            Ordering::Greater=>{

                println!("\n{} used {}!",
                         &game_state.player.party.mon[player_mon_index].as_ref().unwrap().name,
                         player_selected_move.move_stats().name);

                wild_mon.damage(&game_state.player.party.mon[player_mon_index.clone()]
                    .clone()
                    .unwrap(),&player_selected_move,);

                if wild_mon.current_hp == 0{
                    type_text("Wild Pokemon Fainted!\n");
                    winner = true;
                    game_state.player.party.mon[player_mon_index.clone()]
                        .as_mut()
                        .unwrap()
                        .gain_exp(&wild_mon);
                    break
                }
                else{
                    println!("\n{} used {}",
                        wild_mon.name,
                        wild_mon_move_select.move_stats().name);

                    game_state.player.party.mon[player_mon_index.clone()]
                        .as_mut()
                        .unwrap()
                        .damage(
                            &wild_mon,
                            &wild_mon_move_select,
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
            Ordering::Less=>{
                println!("\n{} used {}",
                         wild_mon.name,
                         wild_mon_move_select.move_stats().name);

                game_state.player.party.mon[player_mon_index.clone()]
                    .as_mut()
                    .unwrap()
                    .damage(
                        &wild_mon,
                        &wild_mon_move_select,
                    );

                if game_state.player.party.mon[player_mon_index.clone()]
                    .clone()
                    .unwrap()
                    .current_hp
                    == 0
                {
                    type_text("You Fainted!");
                }
                else{
                    println!("\n{} used {}!",
                             &game_state.player.party.mon[player_mon_index].as_ref().unwrap().name,
                             player_selected_move.move_stats().name);

                    wild_mon.damage(&game_state.player.party.mon[player_mon_index.clone()]
                        .clone()
                        .unwrap(),&player_selected_move,);

                    if wild_mon.current_hp == 0{
                        type_text("Wild Pokemon Fainted!\n");
                        winner = true;
                        game_state.player.party.mon[player_mon_index.clone()]
                            .as_mut()
                            .unwrap()
                            .gain_exp(&wild_mon);
                        break
                    }
                }
            }
            Ordering::Equal=>{
                println!("\n{} used {}!",
                         &game_state.player.party.mon[player_mon_index].as_ref().unwrap().name,
                         player_selected_move.move_stats().name);

                wild_mon.damage(&game_state.player.party.mon[player_mon_index.clone()]
                    .clone()
                    .unwrap(),&player_selected_move,);

                if wild_mon.current_hp == 0{
                    type_text("Wild Pokemon Fainted!\n");
                    winner = true;
                    game_state.player.party.mon[player_mon_index.clone()]
                        .as_mut()
                        .unwrap()
                        .gain_exp(&wild_mon);
                    break
                }
                else {
                    println!("\n{} used {}",
                             wild_mon.name,
                             wild_mon_move_select.move_stats().name);

                    game_state.player.party.mon[player_mon_index.clone()]
                        .as_mut()
                        .unwrap()
                        .damage(
                            &wild_mon,
                            &wild_mon_move_select,
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
            if wild_mon.current_hp !=0{
                //Leech Seed
                if wild_mon.special_conditions.leech_seeded{
                    wild_mon.leech_seed_effect(game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap());
                }
                //Burn or Poison
                if wild_mon.status == Burned || wild_mon.status == Poisoned{
                    wild_mon.burn_poison_effect();
                }
            }
            if game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap().current_hp != 0 {
                //Leech Seed
                if game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap().special_conditions.leech_seeded == true {
                    game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap().leech_seed_effect(wild_mon);
                }
                //Burn or Poison
                if game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap().status == Burned || game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap().status == Poisoned{
                    game_state.player.party.mon[player_mon_index.clone()].as_mut().unwrap().burn_poison_effect();
                }
            }
        }
    }
    return winner
}

