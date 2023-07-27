/*
Title: game.rs

Desc: Contains the core structure of the 'game', the moving around from location to location, calling
battles, wild encounters, and so on.
 */
use std::cmp::Ordering;
use crate::battle_logic::battle2;
use crate::mon_base_stats::PokemonSpecies::{Bulbasaur, Charamander, Pidgey, Squirtle};
use crate::{read_user_input, type_text, GameState, Pokemon, Trainer};
use colored::Colorize;
use std::{io, result};
use std::io::Write;
use crate::game::Regions::{PewterCity, ViridianCity, PalletTown};
use crate::game::PalletTownLocations::*;
use crate::game::PewterCityLocations::PewterPokeCentre;
use crate::game::ViridianCityLocations::*;
use crate::lib::{get_user_input, PEWTER, travelling, VIRIDIAN};
use crate::wild_battle_logic::wild_encounter;

pub fn rust_red_game(mut game_state: GameState) {
    println!("{}\n********************", "POKEMON - RUST RED".red());
    let msg1 = "\nWhat is your name?\n";
    type_text(msg1);
    let player_name = read_user_input();
    game_state.player.enter_name(player_name.clone());
    let msg2 = format!("Welcome to the world of Pokemon {}!\n", player_name.cyan());
    let msg2 = msg2.as_str();
    type_text(msg2);

    //let mut current_local = Regions::PalletTown(PalletTownLocations::RedsHouse);

    loop {
        match &game_state.location{
            PalletTown(RedsHouse) => {
                println!("\nYou are in your house.");
                println!("1. Go outside");
                println!("2. Talk to MOM");
            }
            PalletTown(PalletTownLocations::Outside) => {
                println!("\nYou are outside in Pallet Town.");
                println!("1. Go inside your house");
                println!("2. Go in Blue's house");
                println!("3. Go inside Oak's Lab");
                println!("4. Go to Route 1");
            }
            PalletTown(OaksLab) => {
                println!("\nYou are in Oak's Lab.");
                println!("1. Go outside");
                println!("2. Talk to OAK");
            }
            PalletTown(Route1) => {
                println!("\nYou are in Route 1.");
                println!("1. Return to Pallet Town");
                println!("2. Go to Viridian City");
            }
            PalletTown(BluesHouse) => {
                println!("\nYou are in Blue's House.");
                println!("1. Go outside");
                println!("2. Talk to DAISY")
            }
            ViridianCity(Gym) => {
                println!("\n You are inside the Viridian City Gym.");
                println!("1. Start Gym Challenge.");
                println!("2. Go outside.");
            }
            ViridianCity(Mart) => {
                println!("\n You are in the Viridian City Mart.");
                println!("1. Shop");
                println!("2. Go outside.");
            }
            ViridianCity(PokeCentre)=>{
                println!("\nYou are in the Viridian City PokeCentre");
                println!("1. Heal Pokemon");
                println!("2. Go Outside");
            }
            ViridianCity(ViridianCityLocations::Outside) => {
                println!("\nYou are in {}.", "Viridian City".color(VIRIDIAN));
                println!("1. Go to Route 1");
                println!("2. Go to Gym");
                println!("3. Go to Mart");
                println!("4. Go to PokeCentre");
                println!("5. Go to Route 2");
            }
            ViridianCity(Route2) => {
                println!("\nYou are in Route 2.");
                println!("1. Go into Viridian Forest");
                println!("2. Go into Diglett's Cave");
                println!("3. Go to Viridian City");
            }
            ViridianCity(ViridianForest) => {
                println!("\n You are in Viridian Forest");
                println!("1. Go to route 2.");
                println!("2. Go to Pewter City.");
            }

            PewterCity(PewterCityLocations::Outside)=>{
                println!("\nYou are in {}", "Pewter City".color(PEWTER));
                println!("1. Go to Gym");
                println!("2. Go to PokeCentre");
                println!("3.");
                println!("4.");
                println!("5. Go to Viridian Forest");
            }
            PewterCity(PewterPokeCentre)=>{
                println!("\nYou are in the Pewter City PokeCentre");
                println!("1. Heal Pokemon");
                println!("2. Go Outside");
            }
            PewterCity(PewterCityLocations::Gym)=>{
                println!("\nYou are in the Pewter City Gym");
                println!("1. Challenge Gym");
                println!("2. Go Outside")
            }
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
            choice = get_user_input(9);
        }

        match game_state.location {

            // PALLET TOWN

            PalletTown(RedsHouse) => match choice {
                1 => game_state.move_loc(PalletTown(PalletTownLocations::Outside)),
                2 => mom(),
                _ => println!("Invalid choice."),
            },
            PalletTown(OaksLab) => match choice {
                1 => game_state.move_loc(PalletTown(PalletTownLocations::Outside)),
                2 => {
                    if game_state.player.party.mon[0] == None {
                        let current_local = starter_selection(&mut game_state);
                        game_state.move_loc(current_local);
                    } else {
                        type_text("OAK: Good Luck on your Adventure!")
                    }
                }
                _ => println!("Invalid choice."),
            },
            PalletTown(Route1) => match choice {
                1 => {travelling("Pallet Town");
                    game_state.move_loc(PalletTown(PalletTownLocations::Outside))},
                2 => {travelling("Viridian City");
                    let mut wild_mon = Pokemon::new(Pidgey, 3);
                    let result  = wild_encounter(&mut game_state, &mut wild_mon);
                    if result {
                        game_state.move_loc(ViridianCity(ViridianCityLocations::Outside))
                    }else{
                        game_state.move_loc(PalletTown(RedsHouse));
                        game_state.player.party.pokecentre_heal();
                    }

                    },
                _ => println!("Invalid choice."),
            },
            PalletTown(BluesHouse) => match choice {
                1 => game_state.move_loc(PalletTown(PalletTownLocations::Outside)),
                2 => type_text("DAISY: Thanks for visiting me! My Brother is at Professor Oak's Lab right now...\n"),
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
                    game_state.move_loc(PalletTown(Route1))
                },
                2 => { println!("Closed! Come back later!"); } //TODO: Make this a conditional based on number of gym badges.
                3 => game_state.move_loc(ViridianCity(Mart)),
                4 => game_state.move_loc(ViridianCity(PokeCentre)),
                5 => {
                    travelling("Route 2");
                    game_state.move_loc(ViridianCity(Route2))
                },
                _ => println!("Invalid choice."),
            },
            ViridianCity(PokeCentre)=>match choice{
                1=>{game_state.player.party.pokecentre_heal()},
                2=>game_state.move_loc(ViridianCity(ViridianCityLocations::Outside)),
                _=>println!("Invalid choice."),
            }
            ViridianCity(Mart) => match choice {
                1 => println!("NOT YET IMPLIMENTED"),
                2 => game_state.move_loc(ViridianCity(ViridianCityLocations::Outside)),
                _ => println!("Invalid choice"),
            }
            ViridianCity(Route2) => match choice {
                1 => {travelling("Viridian Forest");
                    game_state.move_loc(ViridianCity(ViridianForest))
                },
                2 => println!("Blocked by a strange looking tree..."),
                3 => {travelling("Viridian City");
                    game_state.move_loc(ViridianCity(ViridianCityLocations::Outside))
                },
                _ => println!("Invalid choice"),
            }
            ViridianCity(ViridianForest) => match choice {
                1 => {travelling("Route 2");
                    game_state.move_loc(ViridianCity(Route2))
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
                5=>{travelling("Viridian Forest");
                    game_state.move_loc(ViridianCity(ViridianForest))
                },
                _=>println!("Invalid choice"),
            }
            PewterCity(PewterPokeCentre)=>match choice{
                1=>game_state.player.party.pokecentre_heal(),
                2=>game_state.move_loc(PewterCity(PewterCityLocations::Outside)),
                _=>println!("Invalid choice"),
            }
            PewterCity(PewterCityLocations::Gym)=> match choice{
                1=>{todo!()},
                2=>game_state.move_loc(PewterCity(PewterCityLocations::Outside)),
                _=>println!("Invalid choice"),
            }
            _ => {}
        }
    }
}
pub enum Regions {
    PalletTown(PalletTownLocations),
    ViridianCity(ViridianCityLocations),
    PewterCity(PewterCityLocations),
}
#[derive(Clone, Copy)]
pub enum PalletTownLocations {
    RedsHouse,
    BluesHouse,
    OaksLab,
    Route1,
    Outside,
}
#[derive(Clone, Copy)]
pub enum ViridianCityLocations {
    Gym,
    Mart,
    PokeCentre,
    Outside,
    Route2,
    ViridianForest,
}
pub enum PewterCityLocations{
    Gym,
    Outside,
    PewterPokeCentre,
}
fn wild_encounter_chance()->Option<Pokemon>{
    todo!()
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
    type_text(
        "OAK: Welcome to my lab!\n\
    Today is the start of a great adventure for you and my grandson Blue.\n\
    Please pick which Pokemon you want as your companion:\n\
    1. Bulbasaur\n\
    2. Charmander\n\
    3. Squirtle\n",
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
    type_text("OAK: Great Choice!\n");
    type_text("\nBLUE: Wait a minute, Lets Battle!\n");
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
        type_text("\nBLUE: Ugh, I'm going to keep training and show you how strong I am!");
        Regions::PalletTown(PalletTownLocations::OaksLab)
    };
}

fn mom() {
    type_text("MOM: Goodluck today!\n");
}

/*
enum PalletTownLocations{
    RedsHouse,
    NeighboursHouse,
    OaksLab,
    Route1,
    Outside,
}
enum ViridianCityLocations{
    ViridianGym,
    PokeCentre,
    Mart,
    House,
    PokemonAcademy,
    Outside,
}
 */
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
