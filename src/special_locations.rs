use std::thread;
use std::time::Duration;
use colored::Colorize;
use rand::Rng;
use crate::game::{master_menu, PewterCityLocations, Regions};
use crate::{GameState, Pokemon, type_text};
use crate::battle_logic::battle2;
use crate::enemy_trainers::Trainer;
use crate::game::PewterCityLocations::Outside;
use crate::game::Regions::{PewterCity, ViridianCity};
use crate::game::ViridianCityLocations::{Route2, ViridianForest};
use crate::lib::{CINNABAR, EAST, get_user_input, NORTH, SOUTH, VIRIDIAN, WEST};
use crate::mon_base_stats::PokemonSpecies::{Jigglypuff, Lapras, Pidgey, Poliwag};
use crate::region_groups::get_wild_encounter;
use crate::special_locations::ViridianForestNodes::*;
use crate::wild_battle_logic::wild_encounter;

// At present I can't think of useful descriptions for these names, as they represent a hand-draw map
//
enum ViridianForestNodes{
    Node1,
    Node2,
    Node3,
    Node4,
    Node5,
    Node6,
    Node7,
    Node8,
    Node9,
    Node10,
    Node11,
    Node12,
    Node13,
    Node14,
    Node15,
}
pub fn viridian_forest(game_state: &mut GameState){
    let mut location = Node1;
    if game_state.location == PewterCity(Outside){
        location = Node15;
    }

    loop{
        match location{
            Node1 => {
                println!("\nYou are at the entrance to the forest, there is a boy standing near-by:\
                \n1. Go along the {} path\
                \n2. Go {}, into the Tall Grass\
                \n3. Go along the {} path\
                \n4. EXIT {}\
                \n5. Talk to BOY", "West".color(WEST), "North".color(NORTH), "East".color(EAST), "Viridian Forest".color(VIRIDIAN))
            }
            Node2 => {
                println!("\nYou are on the Path {} of the entrance:\
                \n1. Go {} along the path\
                \n2. Go {} along the path",
                "West".color(WEST), "North".color(NORTH), "East".color(EAST))
            }
            Node3 => {
                println!("\nYou are standing in front of a large tree. There is a path to your {} and Long Grass to your {}\
                \n1. Investigate the Large Tree\
                \n2. Go {} along the path\
                \n3. Go {} into the Tall Grass","South".color(SOUTH), "East".color(EAST),
                          "South".color(SOUTH), "East".color(EAST), )
            }
            Node4 => {
                println!("\nYou've reached a corner. There is long grass to your South and long grass to your West:\
                \n1. Go {}\
                \n2. Go {}",
                "West".color(WEST), "South".color(SOUTH))
            }
            Node5 => {
                println!("\nYou've reach a corner. There is a boy who looks hard a work catching bugs.\
                \n1. Talk to BUGCATCHER.\
                \n2. Take the {} Path\
                \n3. Take the {} Path",
                "North".color(NORTH), "West".color(WEST))
            }
            Node6 => {
                println!("\n...\
                \n1. Go {} into the Tall Grass\
                \n2. Go {} along the path\
                \n3. Go {} along the path",
                "North".color(NORTH), "East".color(EAST), "South".color(SOUTH))
            }
            Node7 => {
                println!("\nYou reach a corner. There is a trainer {} of you on the path.\
                \n1. Go {} along the path\
                \n2. Go {} along the path",
                "North".color(NORTH),"North".color(NORTH), "West".color(WEST))
            }
            Node8 => {
                println!("\nThere is a trainer to the {} of you on the path.\
                \n1. Go {} along the path\
                \n2. Go {} into the tall grass\
                \n3. Go {} into the tall grass",
                "East".color(EAST),"East".color(EAST),"West".color(WEST), "South".color(SOUTH))
            }
            Node9 => {
                println!("\nYou come to a fork in the trail, there is a trainer on the {} path and the {} Path\
                \n1. Go {} along the path\
                \n2. Go {} along the path\
                \n3. Go {} along the path",
                         "West".color(WEST), "South".color(SOUTH), "West".color(WEST), "North".color(NORTH),
                    "South".color(SOUTH))
            }
            Node10 => {
                println!("\nYou come to corner, to the {} is long stretch of path with a trainer.\
                \n1. Go {} along the path\
                \n2. Go {} along the path",
                         "west".color(WEST), "West".color(WEST), "South".color(SOUTH))
            }
            Node11 => {
                println!("\nYou reach a corner,\
                \n1. Go {} into the tall grass\
                \n2. Go {} along the long path",
                "South".color(SOUTH), "East".color(EAST))
            }
            Node12 => {
                println!("\nYou can hear a gentle stream from where you are standing\
                \n1. Go {} along the path\
                \n2. Go {} into the tall grass\
                \n3. Go {} into the tall grass",
                "South".color(SOUTH), "North".color(NORTH), "East".color(EAST))
            }
            Node13 => {
                println!("\nYou have come to a corner with a calm pond\
                \n1. Investigate the pond\
                \n2. Go {} along the path\
                \n3. Go {} along the path",
                          "West".color(WEST), "North".color(NORTH))
            }
            Node14 => {
                println!("\nYour reach a corner, {} you can see a break in the trees.\
                \n1. Head {} towards the exit\
                \n2. Go {} along the path",
                "North".color(NORTH), "North".color(NORTH), "East".color(EAST))
            }
            Node15 => {
                println!("\nYou are at the {} Exit of the forest!\
                \n1. Exit the forest\
                \n2. Go {} into the tall grass",
                "North".color(NORTH), "South".color(SOUTH))
            }
        }
        println!("Enter Your Choice:");
        let mut choice = get_user_input(9);
        if choice == 9{
            println!("OPENING MENU");
            master_menu(game_state);
            continue;
        }

        println!();
        thread::sleep(Duration::from_millis(250));
        match location{
            Node1 => match choice{
                1=>location = Node2,
                2=>location = Node4,
                3=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node5}
                    else{break}
                },
                4=>{game_state.move_loc(ViridianCity(Route2));
                    break
                },
                5=>{
                    type_text("\nBOY: This forest is like a maze! Don't be afraid to draw \
                    yourself a map if you start to get lost! Also, look for your my friends, we all \
                     came here to practice battling our pokemon!\n\n");
                    thread::sleep(Duration::from_millis(1000));
                }
                _=>println!("Invalid Choice"),
            }
            Node2 => match choice{
                    1=>location = Node3,
                    2=>location = Node1,
                    _=>println!("Invalid Choice")
                }
            Node3 => match choice{
                1=>{
                    if game_state.event.lee_or_chan == false{
                        type_text("\nYou found a pokeball!\n");
                        thread::sleep(Duration::from_millis(1000));
                        type_text("\nDo you pick it up?\n");
                        println!("1. Yes");
                        println!("2. No");
                        let choice = get_user_input(2);
                        match choice {
                            1=>{
                                let mon = Pokemon::new(Lapras, 10);
                                game_state.player.party.add_party_member(mon);
                                type_text("\nLapras joined your team!\n");
                                thread::sleep(Duration::from_millis(1000));
                                game_state.event.lee_or_chan = true;
                            }
                            2=>{
                                type_text("\nYou left the pokeball alone.\n");
                                thread::sleep(Duration::from_millis(1000));
                            }
                            _=>unreachable!(),
                        }
                    }else{
                        type_text("\nThe tree hollow is empty!\n");
                    }
                }//Finds an item
                2=>location=Node2,
                3=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node4}
                    else{break}
                }
                _=>println!("Invalid Choice")
            }
            Node4 => match choice {
                1=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node3}
                    else{break}
                }
                2=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node1}
                    else{break}
                }
                _=>println!("Invalid Choice")
            }
            Node5 => match choice{
                1=> {
                    type_text("\nHey what are you Bugging me for! Anyways, let's battle!\n");
                    let alive = game_state.trainer_battle(8);
                    if !alive{
                        break
                    }
                    /*
                    let trainer_id = 8;
                    let mut enemy_trainer  = Trainer::get(trainer_id);
                    let result = battle2(game_state, &mut enemy_trainer);
                    if result{
                        type_text("\n Stop Bothering me!")
                    }
                    else{
                        game_state.move_loc(game_state.last_used_pcentre);
                        game_state.player.party.pokecentre_heal();
                        break
                    }
                     */
                }
                2=> location = Node6,
                3=> location = Node1,
                _=>println!("Invalid Choice")
            }
            Node6 => match choice{
                1=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node8}
                    else{break}
                }
                2=>location = Node7,
                3=>location = Node5,
                _=>println!("Invalid Choice")
            }
            Node7 => match choice {
                1=>{
                    type_text("\nYou've been spotted by another Trainer!\n");
                    type_text("\nBUGCATCHER: Let me show you how strong my PokeMon are getting!\n");
                    let alive = game_state.trainer_battle(9);
                    if alive {
                        location = Node9;
                    }else{
                        break
                    }

                    /*
                    let trainer_id = 9;
                    let mut enemy_trainer  = Trainer::get(trainer_id);
                    let result = battle2(game_state, &mut enemy_trainer);
                    if result{
                        type_text("\nStop Bothering me!\n");
                        location = Node9;
                    }
                    else{
                        game_state.move_loc(game_state.last_used_pcentre);
                        game_state.player.party.pokecentre_heal();
                        break
                    }
                     */
                }
                2=>location=Node6,
                _=>println!("Invalid Choice")
            }
            Node8 => match choice{
                1=>{
                    type_text("\nYou've been spotted by another Trainer!\n");
                    type_text("\nBUGCATCHER: I've been having bad catching luck all day!\n");
                    let alive = game_state.trainer_battle(10);
                    if alive {
                        location = Node9;
                    }else{
                        break
                    }
                    /*
                    let trainer_id = 10;
                    let mut enemy_trainer  = Trainer::get(trainer_id);
                    let result = battle2(game_state, &mut enemy_trainer);
                    if result{
                        type_text("\nThe bad luck continues!\n");
                        location = Node9;
                    }
                    else{
                        game_state.move_loc(game_state.last_used_pcentre);
                        game_state.player.party.pokecentre_heal();
                        break
                    }

                     */
                }
                2=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node12}
                    else{break}
                }
                3=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node6}
                    else{break}
                }
                _=>println!("Invalid Choice")
            }
            Node9 => match choice{
                2=>location = Node10,
                1=>{
                    type_text("\nYou've been spotted by another Trainer!\n");
                    type_text("\nBUGCATCHER: I've been having bad catching luck all day!\n");
                    let alive = game_state.trainer_battle(10);
                    if alive {
                        location = Node8;
                    }else{
                        break
                    }/*
                    let trainer_id = 10;
                    let mut enemy_trainer  = Trainer::get(trainer_id);
                    let result = battle2(game_state, &mut enemy_trainer);
                    if result{
                        type_text("\nThe bad luck continues!\n");
                        location = Node8;
                    }
                    else{
                        game_state.move_loc(game_state.last_used_pcentre);
                        game_state.player.party.pokecentre_heal();
                        break
                    }
                    */

                }
                3=>{
                    type_text("\nYou've been spotted by another Trainer!\n");
                    type_text("\nBUGCATCHER: Let me show you how strong my PokeMon are getting!\n");
                    let alive = game_state.trainer_battle(9);
                    if alive{
                        location = Node7;
                    }else{
                        break
                    }/*
                    let trainer_id = 9;
                    let mut enemy_trainer  = Trainer::get(trainer_id);
                    let result = battle2(game_state, &mut enemy_trainer);
                    if result{
                        type_text("\nStop Bothering me!\n");
                        location = Node7;
                    }
                    else{
                        game_state.move_loc(game_state.last_used_pcentre);
                        game_state.player.party.pokecentre_heal();
                        break
                    }*/
                }
                _=>println!("Invalid Choice")
            }
            Node10 => match choice{
                1=>{
                    location = Node11; //TODO Make trainer to go here
                },
                2=>location=Node9,
                _=>println!("Invalid Choice")
            }
            Node11 => match choice{
                1=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node12}
                    else{break}
                }
                2=>{
                    location = Node10;
                }
                _=>println!("Invalid Choice")
            }
            Node12 => match choice{
                1=>location = Node13,
                2=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node11}
                    else{break}
                }
                3=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node8}
                    else{break}
                }
                _=>println!("Invalid Choice")
            }
            Node13 => match choice{
                1=>{
                    type_text("\nThe water looks very refreshing, take a drink?\n");
                    println!("1. Yes");
                    println!("2. No");
                    let choice = get_user_input(2);
                    match choice {
                        1=>{
                            let random_number = rand::thread_rng().gen_range(0..=4);
                            if random_number == 0{
                                let mut mon = Pokemon::new(Poliwag, 10);
                                wild_encounter(game_state, &mut mon);
                            }else {
                                type_text("\nThe water is good, and all your pokemon drink some\n");
                                game_state.player.party.pokecentre_heal();
                            }
                        }
                        2=>{
                            type_text("\nYou think decide not to take a drink.\n");
                        }
                        _=>unreachable!()
                    }
                }
                2=>{
                    location = Node14
                }
                3=>{
                    location = Node12
                }
                _=>println!("Invalid Choice")
            }
            Node14 => match choice {
                1=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node15}
                    else{break}
                }
                2=>{
                    location = Node13
                }
                _=>println!("Invalid Choice")
            }
            Node15 => match choice{
                1=>{
                    game_state.move_loc(PewterCity(Outside));
                    break
                }
                2=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node14}
                    else{break}
                }
                _=>println!("Invalid Choice")
            }
        }
    }
}

pub fn diglett_cave(){}

pub fn mt_moon(){}

fn encounter_roll(wild_mon_list: Regions, mut game_state: &mut GameState)->bool{

    let wild_mon = get_wild_encounter(wild_mon_list);
    if wild_mon != None{
        let result = wild_encounter(&mut game_state, &mut wild_mon.unwrap());
        return if result {
            true
        } else {
            game_state.move_loc(game_state.last_used_pcentre);
            game_state.player.party.pokecentre_heal();
            false
        }
    }
    return true
}