/*
Title: game.rs

Desc: Contains the core structure of the 'game', the moving around from location to location, calling
battles, wild encounters, and so on.
 */
use crate::battle_logic::battle2;
use crate::mon_base_stats::PokemonSpecies::{Bulbasaur, Caterpie, Charamander, Pidgey, Squirtle, Wigglytuff};
use crate::{read_user_input, type_text, GameState, Pokemon, Trainer, save_temp, Starter};
use crate::game::Regions::{PewterCity, ViridianCity, PalletTown, Route3};
use crate::game::PalletTownLocations::*;
use crate::game::PewterCityLocations::*;
use crate::game::ViridianCityLocations::*;
use crate::game::Route3Loc::*;
use crate::lib::{CINNABAR, DIGLETT, EAST, FUCHSIA, get_user_input, GYM, MART, MOM, NORTH, OAK, PCENTRE, PEACH, PEWTER, RIVALBLUE, SOUTH, travelling, VIRIDIAN, WEST, YELLOW};
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
use crossterm::style::Color::Red;
use crossterm::style::{style, Stylize};
use serde::{Serialize, Deserialize};
use crate::Starter::{Bulb, Charm, Squirt};

pub fn rust_red_game(mut game_state: GameState) {

    loop {
        match &game_state.location{
            PalletTown(RedsHouse) => {
                println!("\nYou are in your house.");
                println!("1. Go outside");
                println!("2. Talk to {}", "MOM".color(MOM));
                println!("3. Go to bed");
            }
            PalletTown(PalletTownLocations::Outside) => {
                println!("\nYou are outside in {}.", "Pallet Town".color(YELLOW));
                println!("1. Go inside your house");
                println!("2. Go in {}'s house", "BLUE".color(RIVALBLUE));
                println!("3. Go inside {}'s Lab", "OAK".color(OAK));
                println!("4. Go to Route 1");
                println!("5. Read Sign");
            }
            PalletTown(OaksLab) => {
                println!("\nYou are in {}'s Lab.", "OAK".color(OAK));
                if game_state.event.starter_received == false{
                    print!("Professor Oak is standing there waiting next to a boy.");
                }
                println!("\n1. Go outside");
                println!("2. Talk to {}", "OAK".color(OAK));
                println!("3. Look around");
            }
            PalletTown(Route1) => {
                println!("\nYou are at the South end of Route 1.");
                println!("1. Return to {}", "Pallet Town".color(YELLOW));
                println!("2. Go {} along Route 1", "North".color(NORTH));
                println!("3. Talk to MAN.");
                println!("4. Read Sign");
            }
            PalletTown(Route1pt2)=>{
                println!("\nYou are in the middle of Route 1.");
                println!("1. Head {} along Route 1 towards {}", "South".color(SOUTH), "Pallet Town".color(YELLOW));
                println!("2. Head {} along Route 1 towards {}", "North".color(NORTH), "Viridian City".color(VIRIDIAN));
                println!("3. Look around");
            }
            PalletTown(Route1pt3)=>{
                println!("\nYou are at the North end of Route 1.");
                println!("1. Go to {}", "Viridian City".color(VIRIDIAN));
                println!("2. Head {} along Route 1", "South".color(SOUTH));
            }
            PalletTown(BluesHouse) => {
                println!("\nYou are in {}'s House.", "BLUE".color(RIVALBLUE));
                println!("1. Go outside");
                println!("2. Talk to DAISY");
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
                println!("4. Read Sign");
            }
            ViridianCity(Route2pt2)=>{
                println!("\nYou are at the {} end of Route 2.", "North".color(NORTH));
                println!("1. Go into {}", "Viridian Forest".color(VIRIDIAN));
                println!("2. Go {} along Route 2", "South".color(SOUTH));
                println!("3. Talk to OAK's AIDE");
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
            Route3(PewterConnection)=>{
                println!("\nYou are at the {} entrance of Route 3. There are two trails heading {};\n\
                 the {} trail looks well maintained, while the {} path cuts through a lot of tall grass ", "West".color(WEST), "East".color(EAST), "North".color(NORTH), "South".color(SOUTH));
                println!("1. Take the {} trail","North".color(NORTH));
                println!("2. Take the {} trail", "South".color(SOUTH));
                println!("3. Read Sign");
                println!("4. Return to {}", "Pewter City".color(PEWTER));
            }
            Route3(NorthWestNode)=>{
                println!("\nYou are at the {} end of the {} trail on Route 3", "West".color(WEST), "Northern".color(NORTH));
                println!("1. Continue {} along the trail", "East".color(EAST));
                println!("2. Cut through the long grass to the {} trail", "Southern".color(SOUTH));
                println!("3. Return to the trail head");
            }
            Route3(NorthCentralNode)=>{
                println!("\nYou are in the middle of the of the {} trail on Route 3",  "Northern".color(NORTH));
                println!("1. Continue {} along the trail", "East".color(EAST));
                println!("2. Cut through the long grass to the {} trail", "Southern".color(SOUTH));
                println!("3. Head {} along the trail", "West".color(WEST));
            }
            Route3(NorthEastNode)=>{
                println!("\nYou are at the {} of the of the {} trail on Route 3", "East".color(EAST), "Northern".color(NORTH));
                println!("1. Continue {} to the trail end", "East".color(EAST));
                println!("2. Cut through the long grass to the {} trail", "Southern".color(SOUTH));
                println!("3. Head {} along the trail", "West".color(WEST));
            }
            Route3(SouthWestNode)=>{
                println!("\nYou are at the {} end of the {} trail on Route 3", "West".color(WEST), "Southern".color(SOUTH));
                println!("1. Continue {} along the trail", "East".color(EAST));
                println!("2. Cut through the long grass to the {} trail", "Northern".color(NORTH));
                println!("3. Return to the trail head");
            }
            Route3(SouthCentralNode)=>{
                println!("\nYou are in the middle of the {} trail on Route 3", "Southern".color(SOUTH));
                println!("1. Continue {} along the trail", "East".color(EAST));
                println!("2. Cut through the long grass to the {} trail", "Northern".color(NORTH));
                println!("3. Head {} along the trail", "West".color(WEST));
            }
            Route3(SouthEastNode)=> {
                println!("\nYou are at the {} end of the {} trail on Route 3", "East".color(EAST), "Southern".color(SOUTH));
                println!("1. Continue {} along the trail", "East".color(EAST));
                println!("2. Cut through the long grass to the {} trail", "Northern".color(NORTH));
                println!("3. Head {} along the trail", "West".color(WEST));
            }
            Route3(MtMoonConnection)=>{
                println!("\nYou are at the trail merger at the {} end of Route 3. \n A little ways off you can see the towering Mt. Moon dominating the horizon", "East".color(EAST));
                println!("1. Head up the path towards Mt. Moon");
                println!("2. Go back along the {} trail", "North".color(NORTH));
                println!("3. Go back along the {} trail", "South".color(SOUTH));
            }
            Route3(MtMoonOpening)=>{
                println!("\nYou are at the entrance to a large cave in the side of Mt. Moon. Nearby is a small Pokemon Centre");
                println!("1. Enter Mt. Moon");
                println!("2. Go to {}", "PokeCentre".color(PCENTRE));
                println!("3. Head back along the path to Route 3");
            }
            Route3(MtMoonPC)=>{
                println!("\nYou are at the Mt.Moon {}.","PokeCentre".color(PCENTRE));
                println!("1. Heal Pokemon");
                println!("2. Go Outside");
                println!("3. Use PC");
                println!("4. Talk to JR. TRAINER");
            }
            Route3(MtMoonInside)=>{
                println!("\nYou are inside Mt.Moon. You can hear the chatter of thousands of zubat, but can see very little.\n\
                In front of you a large rock blocks your way.");
                println!("1. Try to push the rock");
                println!("2. Exit the cave");
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
                3=>{
                    thread::sleep(Duration::from_millis(600));
                    type_text(". . . Z Z Z . . .");
                    thread::sleep(Duration::from_millis(600));
                    type_text("\nMOM: Time to get up and go!");
                }
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
                        a PokeBall and attempt to catch the wild pokemon!\n");
                        thread::sleep(Duration::from_millis(1000));
                        type_text("\n You received PokeBalls!\n");
                        thread::sleep(Duration::from_millis(1000));
                        type_text("\n One last thing, don't forgot that you can enter '9' in order to access the MENU \n\
                        from which you can do things like look ar your team and SAVE your game.\n");
                        game_state.bag.pop();
                        game_state.event.oaks_parcel_delivered = true;
                    }
                    else {
                        type_text("OAK: Good Luck on your Adventure!")
                    }
                }
                3=>{
                    type_text("\nAround you are multiple book shelves filled with thick tomes. Some titles include: \n\
                    -Reproductive Strategies in Ghost Type Pokemon by Karloff von Geist.\n\
                    -Proposal for Recognition of Steel as Distinct Taxonomic Category by UofJohto Dept. of Cladistics.\n\
                    -Zubat as Intermediate Predator in Cave Ecosystems by Jess Fang.\n");
                    thread::sleep(Duration::from_millis(1000));
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
                4=>{
                    type_text("\nRoute 1: A modest and direct walking path that connects the small settlement of Pallet Town \n\
                    with larger crossroads centre of Viridian City. Occupied by many gentle but assertive wild pokemon.\n");
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
                3=>{
                    type_text("\nYou are walking a modest dirt pathway, dense forest lines the path on either side.\n\
                    You can hear the sounds of birds chirps and other pokemon noises in the distance \n\
                    between the trees. To your south you can distantly see the small group of buildings\n\
                    that make up Pallet Town. To the north Viridian City is still obscured by a gentle \n\
                    curve in the road. The sun is bright and the day is hot.\n");
                    thread::sleep(Duration::from_millis(1000));
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
                        type_text("DAISY: Don't forget you can save your game! Enter '9' to open the MENU and then select '4' to SAVE. Good Luck!\n")
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
                5=>{
                    type_text("\nPallet Town: Home of the famous professor Oak's Pokemon Laboratory!\n")
                }
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
                        type_text("\nYou see an old man laying across the road. \n \
                        OLD MAN: I'm not moving until I'm done my nap, why don't you check out the \
                        MART.\n")
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
                        Would you might delivering this parcel to PROFESSOR OAK?\n");
                        game_state.bag.push(OAKPARCEL);
                        type_text("\nYou Received OAK's Parcel\n");
                    }else if game_state.event.oaks_parcel_delivered == false{
                        type_text("\nSHOP KEEPER: You still have that parcel I gave you right?\
                        \n Please bring that to PROFESSOR OAK as soon as you can!\n");
                    }else{
                        type_text("\nSHOP KEEPER: This is embarrassing but we don't actually have any items in stock yet...\n")
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
                2 => {
                    type_text("\nBlocked by a strange looking tree... \n");
                    thread::sleep(Duration::from_millis(400));
                    type_text("\nA pokemon might be able to learn how to cut this...\n");
                    thread::sleep(Duration::from_millis(800));}
                3 => {travelling("Viridian City");
                    travelling_encounter(ViridianCity(Route2),ViridianCity(ViridianCityLocations::Outside), &mut game_state);

                },
                4=>{
                    type_text("\nRoute 2: Runs North/South between the cities of Viridian and Pewter, travellers should \n\
                    be prepared for a challenging hike through the Viridian Forest which occupies the \n\
                    North end of the Route.\n");
                    thread::sleep(Duration::from_millis(800));
                    type_text("\nAttached to the sign is a notice reading 'KANTO WILDLIFE DEPT: Viridian Forest is \n\
                    currently experiencing a higher than average outbreak of Bugs, travellers be aware.\n");
                    thread::sleep(Duration::from_millis(800));
                }
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
                3=>{
                    type_text("\nAIDE: Prof Oak sent me here because we heard reports of PIKACHU being spotted\n\
                    in this forest. Such a rare pokemon, we just had to see if we could get one for ourselves!\n");
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
                        game_state.move_loc(Route3(PewterConnection));
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

            // Route 3

            Route3(PewterConnection)=>match choice {
                1=>{
                    type_text("\n. . .Walking...\n");
                    thread::sleep(Duration::from_millis(400));
                    game_state.move_loc(Route3(NorthWestNode));
                    game_state.trainer_battle(1);
                    thread::sleep(Duration::from_millis(400));
                    type_text("\n...\n")
                }
                2=>{
                    travelling_encounter(Route3(PewterConnection), Route3(SouthWestNode), &mut game_state);
                }
                3=>{
                    type_text("\nRoute 3: The path between Pewter City and Cerulean City is difficult. Between the two cities \n\
                     is the magnificent Mt. Moon. Route 3 connects Pewter City with the West side of Mt. Moon. Route 3 has become a \n\
                     a popular destination for young trainers to train their pokemon and the path is usually dense with trainers \n\
                     iching to battle.\n")
                }
                4=>{
                    travelling("Pewter City");
                    game_state.move_loc(PewterCity(PewterCityLocations::Outside));

                }
                _=>println!("Invalid choice"),
            }
            Route3(NorthWestNode)=>match choice{
                1=>{
                    type_text("\n. . .Walking...\n");
                    thread::sleep(Duration::from_millis(400));
                    game_state.move_loc(Route3(NorthCentralNode));
                    game_state.trainer_battle(2);
                    thread::sleep(Duration::from_millis(400));
                    type_text("\n...\n")
                }
                2=>{
                    travelling_encounter(Route3(PewterConnection), Route3(SouthWestNode), &mut game_state);
                }
                3=>{
                    type_text("\n. . .Walking...\n");
                    thread::sleep(Duration::from_millis(400));
                    game_state.move_loc(Route3(PewterConnection));
                    game_state.trainer_battle(1);
                    thread::sleep(Duration::from_millis(400));
                    type_text("\n...\n");
                }
                _=>println!("Invalid choice"),
            }
            Route3(NorthCentralNode)=>match choice{
                1=>{
                    type_text("\n. . .Walking...\n");
                    thread::sleep(Duration::from_millis(400));
                    game_state.move_loc(Route3(NorthEastNode));
                    game_state.trainer_battle(3);
                    thread::sleep(Duration::from_millis(400));
                    type_text("\n...\n")
                }
                2=>{
                    travelling_encounter(Route3(PewterConnection), Route3(SouthCentralNode), &mut game_state);
                }
                3=>{
                    type_text("\n. . .Walking...\n");
                    thread::sleep(Duration::from_millis(400));
                    game_state.move_loc(Route3(NorthWestNode));
                    game_state.trainer_battle(2);
                    thread::sleep(Duration::from_millis(400));
                    type_text("\n...\n")
                }
                _=>println!("Invalid choice"),
            }
            Route3(NorthEastNode)=>match choice{
                1=>{
                    type_text("\n. . .Walking...\n");
                    thread::sleep(Duration::from_millis(400));
                    game_state.move_loc(Route3(MtMoonConnection));
                    game_state.trainer_battle(4);
                    thread::sleep(Duration::from_millis(400));
                    type_text("\n...\n")
                }
                2=>{
                    travelling_encounter(Route3(PewterConnection), Route3(SouthEastNode), &mut game_state);
                }
                3=>{type_text("\n. . .Walking...\n");
                    thread::sleep(Duration::from_millis(400));
                    game_state.move_loc(Route3(NorthCentralNode));
                    game_state.trainer_battle(3);
                    thread::sleep(Duration::from_millis(400));
                    type_text("\n...\n")
                }
                _=>println!("Invalid choice"),
            }
            Route3(SouthWestNode)=>match choice {
                1=>{
                    travelling_encounter(Route3(PewterConnection), Route3(SouthCentralNode), &mut game_state);
                }
                2=>{
                    travelling_encounter(Route3(PewterConnection), Route3(NorthWestNode), &mut game_state);
                }
                3=>{
                    travelling_encounter(Route3(PewterConnection), Route3(PewterConnection), &mut game_state);
                }
                _=>println!("Invalid choice"),
            }
            Route3(SouthCentralNode)=>match choice {
                1=>{
                    travelling_encounter(Route3(PewterConnection), Route3(SouthEastNode), &mut game_state);
                }
                2=>{
                    travelling_encounter(Route3(PewterConnection), Route3(NorthCentralNode), &mut game_state);
                }
                3=>{
                    travelling_encounter(Route3(PewterConnection), Route3(SouthWestNode), &mut game_state);
                }
                _=>println!("Invalid choice"),
            }
            Route3(SouthEastNode)=>match choice {
                1=>{
                    travelling_encounter(Route3(PewterConnection), Route3(MtMoonConnection), &mut game_state);
                }
                2=>{
                    travelling_encounter(Route3(PewterConnection), Route3(NorthEastNode), &mut game_state);
                }
                3=>{
                    travelling_encounter(Route3(PewterConnection), Route3(SouthCentralNode), &mut game_state);
                }
                _=>println!("Invalid choice"),
            }
            Route3(MtMoonConnection)=>match choice {
                1 => {
                    travelling_encounter(Route3(PewterConnection), Route3(MtMoonOpening), &mut game_state);
                }
                2 => {
                    type_text("\n. . .Walking...\n");
                    thread::sleep(Duration::from_millis(400));
                    game_state.move_loc(Route3(NorthCentralNode));
                    game_state.trainer_battle(4);
                    thread::sleep(Duration::from_millis(400));
                    type_text("\n...\n")
                }
                3 => {
                    travelling_encounter(Route3(PewterConnection), Route3(SouthEastNode), &mut game_state);
                }
                _ => println!("Invalid choice"),
            }
            Route3(MtMoonOpening)=>match choice{
                    1=>{
                        if game_state.event.lavender_tower_ghost == false{
                            type_text("\nBLUE: Hey! Wait! Don't go in there! People are saying there's a dangerous monster on the loose.\n\
                            \n. . . . . . . .\n\
                            No I'm not just saying that to stop you!\n\
                            \n. . . . . . . .\n\
                            Okay, fine, if you don't believe me, I'm just going to have to stop you myself!\n");

                            match game_state.starter {
                                Bulb => {
                                    let result  = game_state.trainer_battle(2002);
                                    if result {
                                        type_text("\nBLUE: Okay fine, do what you want, but don't blame me if it GETS you!.\n");
                                        game_state.event.lavender_tower_ghost = true;
                                        thread::sleep(Duration::from_millis(1400));
                                    }
                                }
                                Charm => {
                                    let result = game_state.trainer_battle(3002);
                                    if result {
                                        type_text("\nBLUE: Okay fine, do what you want, but don't blame me if it GETS you!.\n");
                                        game_state.event.lavender_tower_ghost = true;
                                        thread::sleep(Duration::from_millis(1400));
                                    }
                                }
                                Squirt => {
                                    let result = game_state.trainer_battle(1002);
                                    if result {
                                        type_text("\nBLUE: Okay fine, do what you want, but don't blame me if it GETS you!.\n");
                                        game_state.event.lavender_tower_ghost = true;
                                        thread::sleep(Duration::from_millis(1400));
                                    }
                                }
                            }
                        }else{
                            game_state.move_loc(Route3(MtMoonInside))
                        }
                    }
                    2=>{game_state.move_loc(Route3(MtMoonPC))}
                    3=>{
                        travelling_encounter(Route3(PewterConnection), Route3(MtMoonConnection), &mut game_state);
                    }
                    _=>println!("Invalid choice"),
                }
            Route3(MtMoonPC)=>match choice{
                    1=>{
                        game_state.player.party.pokecentre_heal();
                        game_state.last_used_pcentre = Route3(MtMoonPC);
                    },
                    2=>game_state.move_loc(Route3(MtMoonOpening)),
                    3=> billpc(&mut game_state),
                4=>{
                    type_text("\nJR.TRAINER: I want to get to Cerulean City so bad to see MISTY but I heard there is a \n\
                    terrible monster in the Mt.Moon and now I'm too scared to go!\n");
                }
                    _=>println!("Invalid choice"),
                }
            Route3(MtMoonInside)=>match choice{
                1=>{
                    game_over();
                    save_temp(&game_state);
                    println!("\n");
                    break
                }
                2=>{
                    game_state.move_loc(Route3(MtMoonOpening))
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
    Route3(Route3Loc),
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
    //Route3,
}
#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Route3Loc{
    PewterConnection,
    NorthWestNode,
    NorthEastNode,
    NorthCentralNode,
    SouthCentralNode,
    SouthWestNode,
    SouthEastNode,
    MtMoonConnection,
    MtMoonOpening,
    MtMoonPC,
    MtMoonInside,
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
                game_state.starter = Bulb;
            }
            "2" => {
                &game_state.player.party.add_party_member(charmander.clone());
                rival_id = 3001;
                choice = false;
                game_state.starter = Charm;
            }
            "3" => {
                &game_state.player.party.add_party_member(squirtle.clone());
                rival_id = 1001;
                choice = false;
                game_state.starter = Squirt;
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
pub fn game_over(){
    type_text("\nYou try to push the large rock.\n");
    thread::sleep(Duration::from_millis(500));
    type_text("\n.  .  .  \n");
    thread::sleep(Duration::from_millis(500));
    type_text("\nThe rock starts to budge!\n");
    thread::sleep(Duration::from_millis(500));
    type_text("\nThe rock continues to move...\n");
    thread::sleep(Duration::from_millis(1000));



    println!(r"                            _____   ____
                       _,.,|     `'`-.._`--._
                    _,' ,j |            `'-. `-,
                 _,'_,-' ' |._              `.  \`.
               ,' ,',.....L   `-._            \  . `.
             .' ,'''`.__  |       `-.._        | |   \
   ,.._     ,'-/     '  `.|..'''|`._   `-.___.-','-._ `.
 ,' . _>-.._/ /     /    /   `-.' \ `-._  |   ,'     `-..
/,..|`._'  / /     /   ,'   _ _\   `.   `-:..'          `\
''  | .--./ /     /   / ,'''|/ .''''\`.._ |  \            |
  /'`.   / |`...+.   /.' _.`+._ `._/ \'| `|\  `.____      |
 /,..:.-+ _|.-''''`./__.'      `.|    j   `.\  /---._'---.|`.
 '     _:'    ____  | |          `+---'     `\/       '-._| |
     ,'    ,+'  |   ' '.           \`.       |            `.|
    .     d |  /     \  \          |  \      |             ||
    |   _/..+.'       \  \      __,^.  '._   |            j |
   ,'_,'        ___    \  `----' ,.--`+..,.-'+`-.._       | |
  ','     ____,'/     / +...--'_,.--''||       '._ `-..__/ /
   `...--''|  .'   _,'| / ..-''       ||          `.    / |
          ,'./ ,.-'   |j |          __||          .'`,'__.'
          \__.'\     j | |        ,'    `-.     ,' ,'.' .'
              \|     | 'j       ,'         `. ,' .',' .'
             . `.____|/ |__    :            |`,-'.'_.'
             '.  `._ _.'-._`-._|            +----''
               `.   `''-.._`-._|            |
                |          `<' `.           |
                /            `.  `.         '
           ,.':'_,-           |,..'          `._
          '.__|' ,--.    __,.''> .             /`.
              '''`---`'''  \_.' _|-':__,....--''''
                             `-',..-'");

    thread::sleep(Duration::from_millis(4000));

    type_text(format!("\n{}\n", "You've been tragically eaten by an angry GOLEM!".color(CINNABAR)).as_str());
    thread::sleep(Duration::from_millis(2000));
    let game_over = style(r" _______  _______  _______  _______    _______           _______  _______
(  ____ \(  ___  )(       )(  ____ \  (  ___  )|\     /|(  ____ \(  ____ )
| (    \/| (   ) || () () || (    \/  | (   ) || )   ( || (    \/| (    )|
| |      | (___) || || || || (__      | |   | || |   | || (__    | (____)|
| | ____ |  ___  || |(_)| ||  __)     | |   | |( (   ) )|  __)   |     __)
| | \_  )| (   ) || |   | || (        | |   | | \ \_/ / | (      | (\ (
| (___) || )   ( || )   ( || (____/\  | (___) |  \   /  | (____/\| ) \ \__
(_______)|/     \||/     \|(_______/  (_______)   \_/   (_______/|/   \__/
                                                                          ").with(Red);
    println!("{}", game_over);
    println!("\n\n{}", "Thanks for playing!!".color(FUCHSIA))
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
