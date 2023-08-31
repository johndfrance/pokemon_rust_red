/*
Title: game.rs

Desc: Contains the core structure of the 'game', the moving around from location to location, calling
battles, wild encounters, and so on.
 */

use crate::battle_logic::battle2;
use crate::mon_base_stats::PokemonSpecies::{Bulbasaur, Caterpie, Charamander, Pidgey, Squirtle};
use crate::{read_user_input, type_text, GameState, Pokemon, Trainer, save_temp};
use crate::game::Regions::{PewterCity, ViridianCity, PalletTown};
use crate::game::PalletTownLocations::*;
use crate::game::PewterCityLocations::*;
use crate::game::ViridianCityLocations::*;
use crate::lib::{DIGLETT, get_user_input, GYM, MART, MOM, NORTH, OAK, PCENTRE, PEACH, PEWTER, RIVALBLUE, SOUTH, travelling, VIRIDIAN, YELLOW};
use crate::region_groups::get_wild_encounter;
use crate::wild_battle_logic::wild_encounter;
use crate::gym_challenges::viridian_gym;
use crate::items::OAKPARCEL;
use crate::special_locations::viridian_forest;

use std::{io, result, thread};
use std::io::Write;
use std::cmp::Ordering;
use std::fmt::format;
use std::time::Duration;
use colored::Colorize;
use crossterm::style::Stylize;
use serde::{Serialize, Deserialize};

pub fn rust_red_game(mut game_state: GameState) {

    loop {
        match &game_state.location{
            PalletTown(RedsHouse) => {
                println!("\nYou are in your house.");
                println!("1. Go outside");
                println!("2. Talk to {}", "MOM".color(MOM));
            }
            PalletTown(PalletTownLocations::Outside) => {
                println!("\nYou are outside in {}.", "Pallet Town".color(YELLOW));
                println!("1. Go inside your house");
                println!("2. Go in {}'s house", "BLUE".color(RIVALBLUE));
                println!("3. Go inside {}'s Lab", "OAK".color(OAK));
                println!("4. Go to Route 1");
            }
            PalletTown(OaksLab) => {
                println!("\nYou are in {}'s Lab.", "OAK".color(OAK));
                println!("1. Go outside");
                println!("2. Talk to {}", "OAK".color(OAK));
            }
            PalletTown(Route1) => {
                println!("\nYou are at the South end of Route 1.");
                println!("1. Return to {}", "Pallet Town".color(YELLOW));
                println!("2. Go {} along Route 1", "North".color(NORTH));
                println!("3. Talk to MAN.");
            }
            PalletTown(Route1pt2)=>{
                println!("\nYou are in the middle of Route 1.");
                println!("1. Head {} along Route 1 towards {}", "South".color(SOUTH), "Pallet Town".color(YELLOW));
                println!("2. Head {} along Route 1 towards {}", "North".color(NORTH), "Viridian City".color(VIRIDIAN));
            }
            PalletTown(Route1pt3)=>{
                println!("\nYou are at the North end of Route 1.");
                println!("1. Go to {}", "Viridian City".color(VIRIDIAN));
                println!("2. Head {} along Route 1", "South".color(SOUTH));
            }
            PalletTown(BluesHouse) => {
                println!("\nYou are in {}'s House.", "BLUE".color(RIVALBLUE));
                println!("1. Go outside");
                println!("2. Talk to DAISY")
            }
            ViridianCity(ViridianCityLocations::Gym) => {
                println!("\nYou are inside the {} {}.","Viridian City".color(VIRIDIAN), "Gym".color(GYM));
                println!("1. Start {} Challenge.", "Gym".color(GYM));
                println!("2. Go outside.");
            }
            ViridianCity(Mart) => {
                println!("\nYou are in the {} Mart.","Viridian City".color(VIRIDIAN));
                println!("1. Shop");
                println!("2. Go outside.");
            }
            ViridianCity(PokeCentre)=>{
                println!("\nYou are in the {} PokeCentre", "Viridian City".color(VIRIDIAN));
                println!("1. Heal Pokemon");
                println!("2. Go Outside");
                println!("3. Use PC")
            }
            ViridianCity(ViridianCityLocations::Outside) => {
                println!("\nYou are in {}.", "Viridian City".color(VIRIDIAN));
                println!("1. Go to Route 1");
                println!("2. Go to {}", "Gym".color(GYM));
                println!("3. Go to {}", "Mart".color(MART));
                println!("4. Go to {}", "PokeCentre".color(PCENTRE));
                println!("5. Go to Route 2");
            }
            ViridianCity(Route2) => {
                println!("\nYou are at the {} end of Route 2.", "South".color(SOUTH));
                println!("1. Go {} along Route 2", "North".color(NORTH));
                println!("2. Go into {}", "Diglett's Cave".color(DIGLETT));
                println!("3. Go to {}", "Viridian City".color(VIRIDIAN));
            }
            ViridianCity(Route2pt2)=>{
                println!("\nYou are at the {} end of Route 2.", "North".color(NORTH));
                println!("1. Go into {}", "Viridian Forest".color(VIRIDIAN));
                println!("2. Go {} along Route 2", "South".color(SOUTH));
            }
            ViridianCity(ViridianForest) => {
                println!("\n You are in Viridian Forest");
                println!("1. Go to Route 2.");
                println!("2. Go to Pewter City.");
            }

            PewterCity(PewterCityLocations::Outside)=>{
                println!("\nYou are in {}", "Pewter City".color(PEWTER));
                println!("1. Go to {}", "Gym".color(GYM));
                println!("2. Go to {}", "PokeCentre".color(PCENTRE));
                println!("3. Go to {}", "Mart".color(MART));
                println!("4. Go to Route 3");
                println!("5. Go to {}", "Viridian Forest".color(VIRIDIAN));
            }
            PewterCity(PewterPokeCentre)=>{
                println!("\nYou are in the {} {}", "Pewter City".color(PEWTER), "PokeCentre".color(PCENTRE));
                println!("1. Heal Pokemon");
                println!("2. Go Outside");
                println!("3. Use PC")
            }
            PewterCity(PewterMart)=>{
                println!("\nYou are in the {} {}","Pewter City".color(PEWTER),"Mart".color(MART));
                println!("1. Talk to SHOPKEEPER");
                println!("2. Go Outside");
            }
            PewterCity(PewterCityLocations::Gym)=>{
                println!("\nYou are in the {} {}", "Pewter City".color(PEWTER), "Gym".color(GYM));
                println!("1. Challenge {}", "Gym".color(GYM));
                println!("2. Go Outside")
            }
            PewterCity(Route3)=>{
                println!("\nYou are in Route 3");
                println!("1. Go To Mt.Moon");
                println!("2. Go to {}", "Pewter City".color(PEWTER));
            }
            /*
            PewterCity(Route3pt2)=>{
                println!("");
            }
             */
        }

        // Read player input
        let mut input = String::new();

        /*
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<usize>().unwrap();
         */
        println!("Enter your choice: ");
        let mut choice = get_user_input(9);
        if choice == 9{
            println!("OPENING MENU");
            master_menu(&mut game_state);
            //choice = get_user_input(9);

            continue;
        }

        match game_state.location {

            // PALLET TOWN

            PalletTown(RedsHouse) => match choice {
                1 => game_state.move_loc(PalletTown(PalletTownLocations::Outside)),
                2 => mom(&game_state),
                _ => println!("Invalid choice."),
            },
            PalletTown(OaksLab) => match choice {
                1 => game_state.move_loc(PalletTown(PalletTownLocations::Outside)),
                2 => {
                    if game_state.player.party.mon[0] == None {
                        let current_local = starter_selection(&mut game_state);
                        game_state.move_loc(current_local);
                    }else if game_state.event.starter_received == true && game_state.event.oaks_parcel_delivered == false && game_state.bag.contains(&OAKPARCEL){
                        type_text("\nOAK: Oh! It looks you have my parcel! Thank you very much!");
                        thread::sleep(Duration::from_millis(600));
                        type_text("\n Inside this parcel is a set of new PokeBalls! I Ordered some to give to you\n \
                        and my grandson to help you both with your adventure. Next time you are in battle\n \
                        against a wild pokemon, if you select ITEM from the battle menu you will throw \n\
                        a PokeBall and attempt to catch the wild pokemon!");
                        thread::sleep(Duration::from_millis(1000));
                        type_text("\n You received PokeBalls!");
                        thread::sleep(Duration::from_millis(1000));
                        type_text("\n One last thing, don't forgot that you can enter '9' in order to access the MENU \n\
                        from which you can do things like look ar your team and SAVE your game.");
                        game_state.bag.pop();
                        game_state.event.oaks_parcel_delivered = true;
                    }

                    else {
                        type_text("OAK: Good Luck on your Adventure!")
                    }
                }
                _ => println!("Invalid choice."),
            },
            PalletTown(Route1) => match choice {
                1 => {travelling("Pallet Town");
                    travelling_encounter(PalletTown(Route1), PalletTown(PalletTownLocations::Outside), &mut game_state);
                    },
                2 => {
                    travelling_encounter(PalletTown(Route1), PalletTown(Route1pt2), &mut game_state);
                    },
                3=>{
                    type_text("\nMAN: Wild pokemon can attack you on the Routes between cities, make sure you have a Pokemon \nstrong enough to protect you!\n");
                    thread::sleep(Duration::from_millis(1000));
                }
                _ => println!("Invalid choice."),
            },
            PalletTown(Route1pt2)=>match choice {
                1=>{
                    travelling_encounter(PalletTown(Route1), PalletTown(Route1), &mut game_state);
                },
                2=>{
                    travelling_encounter(PalletTown(Route1), PalletTown(Route1pt3), &mut game_state);
                }
                _=>println!("Invalid choice."),

            }
            PalletTown(Route1pt3)=>match choice {
                1=>{
                    travelling("Viridian City");
                    travelling_encounter(PalletTown(Route1), ViridianCity(ViridianCityLocations::Outside), &mut game_state);
                }
                2=>{
                    travelling_encounter(PalletTown(Route1), PalletTown(Route1pt2), &mut game_state);
                }
                _=>println!("Invalid choice."),
            }
            PalletTown(BluesHouse) => match choice {
                1 => game_state.move_loc(PalletTown(PalletTownLocations::Outside)),
                2 => {
                    if game_state.event.starter_received == false {
                        type_text("DAISY: Thanks for visiting me! My Brother is at Professor Oak's Lab right now...\n");
                    }else{
                        type_text("DAISY: Don't forget you can save your game! Enter '9' to open the MENU and then select '4' to SAVE. Good Luck!")
                    }
                },
                _ => println!("Invalid choice."),
            },
            PalletTown(PalletTownLocations::Outside) => match choice {
                1 => game_state.move_loc(PalletTown(RedsHouse)),
                2 => game_state.move_loc(PalletTown(BluesHouse)),
                3 => game_state.move_loc(PalletTown(OaksLab)),
                4 => {
                    if adventure_start_check(&game_state) {
                        travelling("Route 1");
                        game_state.move_loc(PalletTown(Route1))
                    }
                },
                _ => println!("Invalid choice."),
            },

            // VIRIDIAN CITY

            ViridianCity(ViridianCityLocations::Outside) => match choice {
                1 => {
                    travelling("Route 1");
                    game_state.move_loc(PalletTown(Route1pt3))
                },
                2 => { println!("\nThere's a sign that says: 'Closed! Come back later!'\n"); thread::sleep(Duration::from_millis(1000)); } //TODO: Make this a conditional based on number of gym badges.
                3 => game_state.move_loc(ViridianCity(Mart)),
                4 => game_state.move_loc(ViridianCity(PokeCentre)),
                5 => {
                    if game_state.event.oaks_parcel_delivered == false{
                        type_text("\n You see an old man laying across the road. \n \
                        OLD MAN: I'm not moving until I'm done my nap, why don't you check out the \
                        MART.")
                    }
                    else {
                        travelling("Route 2");
                        game_state.move_loc(ViridianCity(Route2))
                    }
                },
                _ => println!("Invalid choice."),
            },
            ViridianCity(PokeCentre)=>match choice{
                1=>{
                    game_state.player.party.pokecentre_heal();
                    game_state.last_used_pcentre = ViridianCity(PokeCentre);
                },
                2=>game_state.move_loc(ViridianCity(ViridianCityLocations::Outside)),
                3=> billpc(&mut game_state),
                _=>println!("Invalid choice."),
            }
            ViridianCity(Mart) => match choice {
                1 => {
                    if game_state.event.oaks_parcel_delivered == false && game_state.bag.is_empty(){
                        type_text("\nSHOP KEEPER: Hi there! You are from Pallet Town right? \
                        Would you might delivering this parcel to PROFESSOR OAK?");
                        game_state.bag.push(OAKPARCEL);
                        type_text("\nYou Received OAK's Parcel");
                    }else if game_state.event.oaks_parcel_delivered == false{
                        type_text("\nSHOP KEEPER: You still have that parcel I gave you right?\
                        \n Please bring that to PROFESSOR OAK as soon as you can!");
                    }else{
                        type_text("\nSHOP KEEPER: John hasn't yet implemented items yet! ")
                    }
                },
                2 => game_state.move_loc(ViridianCity(ViridianCityLocations::Outside)),
                _ => println!("Invalid choice"),
            }
            ViridianCity(Route2) => match choice {
                1 => {
                    thread::sleep(Duration::from_millis(600));
                    travelling_encounter(ViridianCity(Route2),ViridianCity(Route2pt2), &mut game_state);
                    thread::sleep(Duration::from_millis(600));
                },
                2 => {type_text("\nBlocked by a strange looking tree... \n");
                    thread::sleep(Duration::from_millis(400));
                    type_text("\nA pokemon might be able to learn how to cut this...\n");
                    thread::sleep(Duration::from_millis(800));}
                3 => {travelling("Viridian City");
                    travelling_encounter(ViridianCity(Route2),ViridianCity(ViridianCityLocations::Outside), &mut game_state);

                },
                _ => println!("Invalid choice"),
            }
            ViridianCity(Route2pt2)=>match choice{
                1=>{ travelling("Viridian Forest");
                    viridian_forest(&mut game_state);
                }
                2=>{thread::sleep(Duration::from_millis(600));
                    travelling_encounter(ViridianCity(Route2), ViridianCity(Route2), &mut game_state);
                    thread::sleep(Duration::from_millis(800));
                }
                _ => println!("Invalid choice"),
            }
            ViridianCity(ViridianForest) => match choice {
                1 => {travelling("Route 2");
                    game_state.move_loc(ViridianCity(Route2pt2))
                },
                2 =>{
                    travelling("Pewter City");
                    println!("You've been spotted by another Trainer and they want to Battle!");
                    let trainer_id = 8;
                    let mut enemy_trainer  = Trainer::get(trainer_id);
                    let result = battle2(&mut game_state, &mut enemy_trainer);
                    if result{
                        game_state.move_loc(PewterCity(PewterCityLocations::Outside))
                    }else{
                        println!("You blacked out!");
                        game_state.move_loc(ViridianCity(PokeCentre));
                        game_state.player.party.pokecentre_heal();
                    }

                }
                _ => println!("Invalid choice"),
            }

            // PEWTER CITY

            PewterCity(PewterCityLocations::Outside)=> match choice {
                1=> game_state.move_loc(PewterCity(PewterCityLocations::Gym)),
                2=>game_state.move_loc(PewterCity(PewterPokeCentre)),
                3=>game_state.move_loc(PewterCity(PewterMart)),
                4=>{
                    if game_state.badge_box.boulder==true {
                        travelling("Route 3");
                        game_state.move_loc(PewterCity(Route3));
                    }else{
                        type_text("MAN: Hey, wait! BROCK is looking for strong trainers to \
                        challenge him. \n You should try to face him before heading off to MT. MOON");
                    }
                },
                5=>{travelling("Viridian Forest");
                    viridian_forest(&mut game_state);
                    //game_state.move_loc(ViridianCity(ViridianForest))
                },
                _=>println!("Invalid choice"),
            }
            PewterCity(PewterPokeCentre)=>match choice{
                1=>{
                    game_state.player.party.pokecentre_heal();
                    game_state.last_used_pcentre = PewterCity(PewterPokeCentre);
                },
                2=>game_state.move_loc(PewterCity(PewterCityLocations::Outside)),
                3=> billpc(&mut game_state),
                _=>println!("Invalid choice"),
            }
            PewterCity(PewterCityLocations::Gym)=> match choice{
                1=>{viridian_gym(&mut game_state)},
                2=>game_state.move_loc(PewterCity(PewterCityLocations::Outside)),
                _=>println!("Invalid choice"),
            }
            PewterCity(PewterMart)=>match choice {
                1=>{type_text("SHOPKEEPER: Ha Ha, this is embarrassing but we don't have any stock yet!\n")}
                2=>game_state.move_loc(PewterCity(PewterCityLocations::Outside)),
                _=>println!("Invalid choice"),
            }
            PewterCity(Route3)=>match choice{
                1=>{todo!()},
                2=>{
                    travelling(format!("{}", "Pewter City".color(PEWTER)).as_str());
                    game_state.move_loc(PewterCity(PewterCityLocations::Outside));
                }
                _=>println!("Invalid choice"),
            }
            _ => {}
        }
    }
}
#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Regions {
    PalletTown(PalletTownLocations),
    ViridianCity(ViridianCityLocations),
    PewterCity(PewterCityLocations),
}
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum PalletTownLocations {
    RedsHouse,
    BluesHouse,
    OaksLab,
    Route1,
    Route1pt2,
    Route1pt3,
    Outside,
}
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum ViridianCityLocations {
    Gym,
    Mart,
    PokeCentre,
    Outside,
    Route2,
    Route2pt2,
    ViridianForest,
}
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum PewterCityLocations{
    Gym,
    Outside,
    PewterPokeCentre,
    PewterMart,
    Route3,
}
fn billpc(mut game_state: &mut GameState){
    loop{
        println!("\nYou're accessing Bill's PC, what would you like to do?");
        println!("1. Deposit a Pokemon");
        println!("2. Withdraw a Pokemon");
        println!("3. Exit\n");

        let choice = get_user_input(3);

        match choice {
            1=>game_state.move_mon_party_to_pc(),
            2=>game_state.move_mon_pc_to_party(),
            3=>break,
            _=>unreachable!(),
        }
    }
}

fn wild_encounter_chance()->Option<Pokemon>{
    todo!()
}
fn travelling_encounter(wild_mon_list: Regions, destination: Regions, mut game_state: &mut GameState){

    let wild_mon = get_wild_encounter(wild_mon_list);
    if wild_mon != None{
        let result = wild_encounter(&mut game_state, &mut wild_mon.unwrap());
        if result{
            game_state.move_loc(destination);
        }else{
            game_state.move_loc(game_state.last_used_pcentre);
            game_state.player.party.pokecentre_heal();
        }
    }
    game_state.move_loc(destination);
}

fn adventure_start_check(game_state: &GameState) -> bool {
    return if game_state.player.party.mon[0] == None {
        type_text("OAK: Wait! It's dangerous to go out there without a Pokemon!\n");
        false
    } else {
        true
    };
}

fn starter_selection(game_state: &mut GameState) -> Regions {
    type_text(format!(
        "{}: Welcome to my lab!\n\
    Today is the start of a great adventure for you and my grandson {}.\n\
    Please pick which {} you want as your companion:\n\
    1. {}\n\
    2. {}\n\
    3. {}\n",
        "OAK".color(OAK),"BLUE".color(RIVALBLUE), "POKEMON".color(YELLOW),
        Stylize::green("Bulbasaur"),
        Stylize::red("Charmander"),
        Stylize::blue("Squirtle")).as_str()
    );
    let mut choice = true;
    let bulbasaur = Pokemon::new(Bulbasaur, 5);
    let charmander = Pokemon::new(Charamander, 5);
    let squirtle = Pokemon::new(Squirtle, 5);
    let mut rival_id = 1;
    while choice {
        let starter_choice = read_user_input();
        let starter_choice = starter_choice.as_str();
        match starter_choice {
            "1" => {
                &game_state.player.party.add_party_member(bulbasaur.clone());
                //&game_state.player.party.add_party_member(charmander.clone());
                rival_id = 2001;
                choice = false;
            }
            "2" => {
                &game_state.player.party.add_party_member(charmander.clone());
                rival_id = 3001;
                choice = false;
            }
            "3" => {
                &game_state.player.party.add_party_member(squirtle.clone());
                rival_id = 1001;
                choice = false;
            }
            _ => println!("Sorry, that wasn't a valid choice."),
        }
    }
    game_state.event.starter_received = true;
    type_text(format!("{}: Great Choice!\n", "OAK".color(OAK)).as_str());
    type_text(format!("\n{}: Wait a minute, Lets Battle!\n", "BLUE".color(RIVALBLUE)).as_str());
    let mut trainer_blue = Trainer::get(rival_id);

    let battle_result = battle2(game_state, &mut trainer_blue);

    //game_state.player.party.mon[0].as_mut().unwrap().current_hp =
        //game_state.player.party.mon[0].clone().unwrap().max_hp.value;
    game_state.player.party.pokecentre_heal();

    return if battle_result == false {
        type_text("\nBLUE: Smell Ya Later!\n");
        println!("EVERYTHING GOES BLACK.");
        Regions::PalletTown(PalletTownLocations::RedsHouse)
    } else {
        type_text(format!("\n{}: Ugh, I'm going to keep training and show you how strong I am!\n", "BLUE".color(RIVALBLUE)).as_str());
        Regions::PalletTown(PalletTownLocations::OaksLab)
    };
}

fn mom(game_state: &GameState) {
    if game_state.event.starter_received == false {
        type_text("MOM: Goodluck today!\n");
    }else{
        type_text("MOM: Please remember to come visit!\n");
    }
}

pub fn master_menu(game_state: &mut GameState){
    loop {
        println!("MENU");
        println!("1. PokeDex {}", Stylize::red("TODO"));
        println!("2. Party");
        println!("3. Badges");
        println!("4. Save");
        println!("5. Exit Menu");

        let menu_select = get_user_input(5);

        match menu_select {
            1 => pokedex_display(),
            2 => {
                //party_display(game_state)
                game_state.player.party.party_menu();
            },
            3 => bag_display(game_state),
            4 => save_temp(game_state),
            5 => break,
            _ => unreachable!()
        }
    }
}
fn pokedex_display(){todo!()}
fn party_display(game_state:  &GameState){
    let mut counter = 1;
    println!("\n Your Party:");
    for pokemon in &game_state.player.party.mon{
        if *pokemon != None {
            println!("{} - {:?} {} ({}/{})\n", counter, pokemon.as_ref().unwrap().name, pokemon.as_ref().unwrap().level, pokemon.as_ref().unwrap().current_hp, pokemon.as_ref().unwrap().max_hp.value);
            counter +=1;
        }
    }
}

fn bag_display(game_state: &GameState){
    println!("Your Badges:");
    if game_state.badge_box.boulder{
        println!("{}\n", "BOULDER BADGE".color(PEWTER))
    }else{
        println!("No badges yet!")
    }
}


/*
enum Location {
    PalletTown,
    Route1,
    ViridianCity,
    PewterCity,
    CeruleanCity,
    VermilionCity,
    LavenderTown,
    CaladonCity,
    FuchiaCity,
    SaffronCity,
    CinnabarIsland,
}
 */
