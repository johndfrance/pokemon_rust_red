use std::cmp::Ordering;
use crate::{enemy_move_select, GameState, PartyOperations, Pokemon, type_text};
use crate::battle_logic::{battle_display_menu, battle_display_names, MainMenuOptions};
use crate::move_data::Moves;
use crate::Status::Fainted;

pub fn wild_encounter(game_state: &mut GameState, wild_mon: &mut Pokemon)->bool{
    let mut winner ;
    let player_name = &game_state.player.name;
    type_text(format!("\nYou are attacked by a wild {}", wild_mon.name).as_str());

    let player_starting_mon_index = game_state
        .player
        .party
        .return_first_healthy()
        .expect("Somethings Gone Wrong");

    type_text(format!("{} sends out {}\n",
            player_name,
            game_state.player.party.mon[player_starting_mon_index].clone().unwrap().name).as_str(),);

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
        battle_display_names(&game_state.player.party.mon[player_mon_index.clone()].as_ref().unwrap(),);
        battle_display_names(wild_mon);

        let (menu_choice, sub_menu_choice) = battle_display_menu(&game_state, player_mon_index.clone());

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
        println!("WILD HAS SELECTED: {}", wild_mon_move_select.move_stats().name);
        let speed_order = game_state.player.party.mon[player_mon_index.clone()]
            .clone()
            .unwrap()
            .spd
            .value
            .cmp(&wild_mon.spd.value);

        println!("DEBUG SPEED: {:?}", speed_order);

        match speed_order{
            Ordering::Greater=>{

                println!("{} used {}!",
                         &game_state.player.party.mon[player_mon_index].as_ref().unwrap().name,
                         player_selected_move.move_stats().name);

                wild_mon.damage(&game_state.player.party.mon[player_mon_index.clone()]
                    .clone()
                    .unwrap(),&player_selected_move,);

                if wild_mon.current_hp == 0{
                    type_text("Wild Pokemon Fainted!");
                    winner = true;
                    game_state.player.party.mon[player_mon_index.clone()]
                        .as_mut()
                        .unwrap()
                        .gain_exp(&wild_mon);
                    break
                }
                else{
                    println!("{} used {}",
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
                println!("{} used {}",
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
                    println!("{} used {}!",
                             &game_state.player.party.mon[player_mon_index].as_ref().unwrap().name,
                             player_selected_move.move_stats().name);

                    wild_mon.damage(&game_state.player.party.mon[player_mon_index.clone()]
                        .clone()
                        .unwrap(),&player_selected_move,);

                    if wild_mon.current_hp == 0{
                        type_text("Wild Pokemon Fainted!");
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
                println!("{} used {}!",
                         &game_state.player.party.mon[player_mon_index].as_ref().unwrap().name,
                         player_selected_move.move_stats().name);

                wild_mon.damage(&game_state.player.party.mon[player_mon_index.clone()]
                    .clone()
                    .unwrap(),&player_selected_move,);

                if wild_mon.current_hp == 0{
                    type_text("Wild Pokemon Fainted!");
                    winner = true;
                    game_state.player.party.mon[player_mon_index.clone()]
                        .as_mut()
                        .unwrap()
                        .gain_exp(&wild_mon);
                    break
                }
                else{
                    println!("{} used {}",
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
    }
    return winner
}

