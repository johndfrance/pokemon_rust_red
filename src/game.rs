use std::io;
use std::io::Write;
use crate::{battle, GameState, Pokemon, read_user_input, Trainer, type_text};
use crate::mon_base_stats::PokemonSpecies::{Bulbasaur, Charamander, Squirtle};
use colored::Colorize;

pub fn rust_red_game(mut game_state: GameState){
    println!("POKEMON - RUST RED\n********************");
    let msg1 = "\nWhat is your name?\n";
    type_text(msg1);
    let player_name = read_user_input();
    game_state.player.enter_name(player_name.clone());
    let msg2 = format!("Welcome to the world of Pokemon {}!\n", player_name);
    let msg2 = msg2.as_str();
    type_text(msg2);

    let mut current_local = Regions::PalletTown(PalletTownLocations::RedsHouse);

    loop {
        match current_local {
            Regions::PalletTown(PalletTownLocations::RedsHouse) => {
                println!("\nYou are in your house.");
                println!("1. Go outside");
                println!("2. Talk to MOM");
            }
            Regions::PalletTown(PalletTownLocations::Outside) => {
                println!("\nYou are outside in Pallet Town.");
                println!("1. Go inside your house");
                println!("2. Go in Blue's house");
                println!("3. Go inside Oak's Lab");
                println!("4. Go to Route 1");
            }
            Regions::PalletTown(PalletTownLocations::OaksLab) => {
                println!("\nYou are in Oak's Lab.");
                println!("1. Go outside");
                println!("2. Talk to OAK");
            }
            Regions::PalletTown(PalletTownLocations::Route1) => {
                println!("\nYou are in Route 1.");
                println!("1. Return to Pallet Town");
                println!("2. Go to Viridian City");
            }
            Regions::PalletTown(PalletTownLocations::BluesHouse) => {
                println!("\nYou are in Blue's House.");
                println!("1. Go outside");
                println!("2. Talk to DAISY")
            }
            Regions::ViridianCity(ViridianCityLocations::Gym) => {
                println!("\n You are inside the Viridian City Gym.");
                println!("1. Start Gym Challenge.");
                println!("2. Go outside.");
            }
            Regions::ViridianCity(ViridianCityLocations::Mart) => {
                println!("\n You are in the Viridian City Mart.");
                println!("1. Shop");
                println!("2. Go outside.");
            }
            Regions::ViridianCity(ViridianCityLocations::Outside) => {
                println!("\nYou are in Viridian City.");
                println!("1. Go to Route 1");
                println!("2. Go to Gym");
                println!("3. Go to Mart");
                println!("4. Go to Route 2");
            }
            Regions::ViridianCity(ViridianCityLocations::Route2)=>{
                println!("\nYou are in Route 2.");
                println!("1. Go into Viridian Forest");
                println!("2. Go into Diglett's Cave");
                println!("3. Go to Viridian City");
            }
            Regions::ViridianCity(ViridianCityLocations::ViridianForest)=>{
                println!("\n You are in Viridian Forest");
                println!("1. Go to route 2.");
            }
        }

        // Read player input
        let mut input = String::new();
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<usize>().unwrap();

        match current_local {
            ///
            /// Pallet Town
            ///
            Regions::PalletTown(PalletTownLocations::RedsHouse) => match choice {
                1 => current_local = Regions::PalletTown(PalletTownLocations::Outside),
                2 => mom(),
                _ => println!("Invalid choice."),
            },
            Regions::PalletTown(PalletTownLocations::OaksLab) => match choice {
                1 => current_local = Regions::PalletTown(PalletTownLocations::Outside),
                2 => {
                    if game_state.player.party.mon1 == None {
                        current_local = starter_selection(&mut game_state)
                    } else {
                        type_text("OAK: Good Luck on your Adventure!")
                    }
                }
                _ => println!("Invalid choice."),
            },
            Regions::PalletTown(PalletTownLocations::Route1) => match choice {
                1 => current_local = Regions::PalletTown(PalletTownLocations::Outside),
                2 => current_local = Regions::ViridianCity(ViridianCityLocations::Outside),
                _ => println!("Invalid choice."),
            },
            Regions::PalletTown(PalletTownLocations::BluesHouse) => match choice {
                1 => current_local = Regions::PalletTown(PalletTownLocations::Outside),
                2 => type_text("DAISY: Thanks for visiting me! My Brother is at Professor Oak's Lab right now...\n"),
                _ => println!("Invalid choice."),
            },
            Regions::PalletTown(PalletTownLocations::Outside) => match choice {
                1 => current_local = Regions::PalletTown(PalletTownLocations::RedsHouse),
                2 => current_local = Regions::PalletTown(PalletTownLocations::BluesHouse),
                3 => current_local = Regions::PalletTown(PalletTownLocations::OaksLab),
                4 => {
                    if adventure_start_check(&game_state) {
                        current_local = Regions::PalletTown(PalletTownLocations::Route1)
                    }
                },
                _ => println!("Invalid choice."),
            },
            ///
            /// Viridian City
            ///
            Regions::ViridianCity(ViridianCityLocations::Outside) => match choice {
                1 => current_local = Regions::PalletTown(PalletTownLocations::Route1),
                2 => { println!("Closed! Come back later!"); } //TODO: Make this a conditional based on number of gym badges.
                3 => current_local = Regions::ViridianCity(ViridianCityLocations::Mart),
                4 => current_local = Regions::ViridianCity(ViridianCityLocations::Route2),
                _ => println!("Invalid choice."),
            },
            Regions::ViridianCity(ViridianCityLocations::Mart) => match choice {
                1 => println!("NOT YET IMPLIMENTED"),
                2 => current_local = Regions::ViridianCity(ViridianCityLocations::Outside),
                _ => println!("Invalid choice"),
            }
            Regions::ViridianCity(ViridianCityLocations::Route2) => match choice {
                1 => current_local = Regions::ViridianCity(ViridianCityLocations::ViridianForest),
                2 => println!("Blocked by a strange looking tree..."),
                3 => current_local = Regions::ViridianCity(ViridianCityLocations::Outside),
                _ => println!("Invalid choice"),
            }
            Regions::ViridianCity(ViridianCityLocations::ViridianForest) => match choice {
                1 => current_local = Regions::ViridianCity(ViridianCityLocations::Route2),
                _ => println!("Invalid choice"),
            }
            _ => {}

        }
    }
}
enum Regions {
    PalletTown(PalletTownLocations),
    ViridianCity(ViridianCityLocations),
}
#[derive(Clone, Copy)]
enum PalletTownLocations {
    RedsHouse,
    BluesHouse,
    OaksLab,
    Route1,
    Outside,
}
#[derive(Clone, Copy)]
enum ViridianCityLocations {
    Gym,
    Mart,
    Outside,
    Route2,
    ViridianForest,
}


fn adventure_start_check(game_state: &GameState)->bool{
    return if game_state.player.party.mon1 == None {
        type_text("OAK: Wait! It's dangerous to go out there without a Pokemon!\n");
        false
    } else {
        true
    }
}

fn starter_selection(game_state: &mut GameState)->Regions{
    type_text("OAK: Welcome to my lab!\n\
    Today is the start of a great adventure for you and my grandson Blue.\n\
    Please pick which Pokemon you want as your companion:\n\
    1. Bulbasaur\n\
    2. Charmander\n\
    3. Squirtle\n");
    let mut choice = true;
    let bulbasaur = Pokemon::new(Bulbasaur, 5);
    let charmander = Pokemon::new(Charamander, 5);
    let squirtle = Pokemon::new(Squirtle, 5);
    while choice {
        let starter_choice = read_user_input();
        let starter_choice = starter_choice.as_str();
        match starter_choice {
            "1" => {
                &game_state.player.party.add_party_member(bulbasaur.clone());
                choice = false;
            }
            "2" => {
                &game_state.player.party.add_party_member(charmander.clone());
                choice = false;
            }
            "3" => {
                &game_state.player.party.add_party_member(squirtle.clone());
                choice = false;
            }
            _ => println!("Sorry, that wasn't a valid choice."),
        }
    }
    type_text("OAK: Great Choice!\n");
    type_text("BLUE: Wait a minute, Lets Battle!\n");
    let mut trainer_blue = Trainer::new();

    let battle_result =  battle(game_state, &mut trainer_blue);
    game_state.player.party.mon1.as_mut().unwrap().current_hp = game_state.player.party.mon1.clone().unwrap().max_hp.value;

    if battle_result == false{
        type_text("BLUE: Smell Ya Later!\n");
        println!("EVERYTHING GOES BLACK.");
        return Regions::PalletTown(PalletTownLocations::RedsHouse);
    }else {
        type_text("BLUE: Ugh, I'm going to keep training and show you how strong I am!");
        return Regions::PalletTown(PalletTownLocations::OaksLab);
    }

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