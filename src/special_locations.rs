use crate::game::{master_menu, PewterCityLocations, Regions};
use crate::{GameState, type_text};
use crate::battle_logic::battle2;
use crate::enemy_trainers::Trainer;
use crate::game::PewterCityLocations::Outside;
use crate::game::Regions::{PewterCity, ViridianCity};
use crate::game::ViridianCityLocations::{Route2, ViridianForest};
use crate::lib::get_user_input;
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
                println!("You are at the entrance to the forest, there is a boy standing near-by:\
                \n1. Go along the West path\
                \n2. Go North, into the Tall Grass\
                \n3. Go along the East path\
                \n4. EXIT Viridian Forest\
                \n5. Talk to BOY")
            }
            Node2 => {
                println!("You are on the Path West of the entrance:\
                \n1. Go North along the path\
                \n2. Go East along the path")
            }
            Node3 => {
                println!("You are standing in front of a large tree. There is a path to your south and Long Grass to your East\
                \n1. Investigate the Large Tree\
                \n2. Go South along the path\
                \n3. Go East into the Tall Grass")
            }
            Node4 => {
                println!("You've reached a corner. There is long grass to your South and long grass to your West:\
                \n1. Go West\
                \n2. Go South")
            }
            Node5 => {
                println!("You've reach a corner. There is a boy who looks hard a work catching bugs.\
                \n1. Talk to BUGCATCHER.\
                \n2. Take the North Path\
                \n3. Take the West Path")
            }
            Node6 => {
                println!("...\
                \n1. Go North into the Tall Grass\
                \n2. Go East along the path\
                \n3. Go South along the path")
            }
            Node7 => {
                println!("You reach a corner. There is a trainer north of you on the path.\
                \n1. Go North along the path\
                \n2. Go West along the path")
            }
            Node8 => {
                println!("There is a trainer to the East of you on the path.\
                \n1. Go East along the path\
                \n2. Go West into the tall grass\
                \n3. Go South into the tall grass")
            }
            Node9 => {
                println!("You come to a fork in the trail, there is a trainer on the West path and the South Path\
                \n1. Go West along the path\
                \n2. Go North along the path\
                \n3. Go South along the path")
            }
            Node10 => {
                println!("You come to corner, to the west is long stretch of path with a trainer.\
                \n1. Go West along the path\
                \n2. Go South along the path")
            }
            Node11 => {
                println!("You reach a corner,\
                \n1. Go South into the tall grass\
                \n2. Go East along the long path")
            }
            Node12 => {
                println!("You can hear a gentle stream from where you are standing\
                \n1. Go South along the path\
                \n2. Go North into the tall grass\
                \n3. Go East into the tall grass")
            }
            Node13 => {
                println!("You have come to a corner with a calm pond\
                \n1. Investigate the pond\
                \n2. Go West along the path\
                \n3. Go North along the path")
            }
            Node14 => {
                println!("Your reach a corner, North you can see a break in the trees.\
                \n1. Head North towards the exit\
                \n2. Go East along the path")
            }
            Node15 => {
                println!("You are at the North Exit of the forest!\
                \n1. Exit the forest\
                \n2. Go South into the tall grass")
            }
        }
        println!("Enter Your Choice:");
        let mut choice = get_user_input(9);
        if choice == 9{
            println!("OPENING MENU");
            master_menu(game_state);
            continue;
        }
        match location{
            Node1 => match choice{
                1=>location = Node2,
                2=>location = Node3,
                3=>{
                    let alive = encounter_roll(ViridianCity(ViridianForest), game_state);
                    if alive {location = Node5}
                    else{break}
                },
                4=>{game_state.move_loc(ViridianCity(Route2));
                    break
                },
                5=>{
                    type_text("BOY: This forest is like a maze! Don't be afraid to draw \
                    yourself a map if you start to get lost! Also, look for your my friends, we all \
                     came here to practice battling our pokemon!")
                }
                _=>println!("Invalid Choice"),
            }
            Node2 => match choice{
                    1=>location = Node3,
                    2=>location = Node1,
                    _=>println!("Invalid Choice")
                }
            Node3 => match choice{
                1=>{todo!()}//Finds an item
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
                }
                2=>location=Node6,
                _=>println!("Invalid Choice")
            }
            Node8 => match choice{
                1=>{
                    type_text("\nYou've been spotted by another Trainer!\n");
                    type_text("\nBUGCATCHER: I've been having bad catching luck all day!\n");
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
                }
                3=>{
                    type_text("\nYou've been spotted by another Trainer!\n");
                    type_text("\nBUGCATCHER: Let me show you how strong my PokeMon are getting!\n");
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
                    }
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
                1=>{todo!()}
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