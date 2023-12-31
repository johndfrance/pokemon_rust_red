#![allow(dead_code, unused)]
mod battle_logic;
mod enemy_trainers;
mod evolution;
mod game;
mod lib;
mod mon_base_stats;
mod mon_move_sets;
mod move_data;
mod npc_dialogue;
mod type_matchups;
mod items;
mod wild_battle_logic;
mod pokemon_structure;
mod region_groups;
mod pokedex;
mod gym_challenges;
mod color_hub;
mod special_locations;
mod new_battle_logic;

use crate::game::*;
use crate::mon_base_stats::PokemonSpecies::{Charamander, Metapod, Pidgey, Rattata, Squirtle};
use crate::mon_base_stats::*;
use crate::move_data::*;
use crate::type_matchups::*;
use crate::PokemonSpecies::{Bulbasaur, Pikachu};
use crate::Status::{Asleep, Burned, Fainted, Frozen, Healthy, Paralyzed, Poisoned};
use crate::mon_move_sets::LEARNABLEMOVES;
use crate::move_data::Moves::Tackle;
use crate::enemy_trainers::Trainer;
use crate::evolution::{CATERPIE, EvolutionData, EvolutionTriggers};
use crate::lib::{CINNABAR, get_user_input, YELLOW, OAK, VERMILION, SAFFRON, travelling};
use crate::MoveCat::Physical;
use crate::StatType::{Accuracy, Attack, Defense, Evasion, Special, Speed};
use crate::items::StdItem;
use crate::color_hub::color_me;
use crate::battle_logic::battle2;

use colored::Colorize;
use rand::Rng;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::format;
use std::{env, io, thread};
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::Duration;
use std::fs::{File, OpenOptions};
use crossterm::style::Color::{Blue, Cyan, Red, Yellow};
use crossterm::style::{Color, style, Stylize};


// MAIN
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let red = style(r" _____      _                                _____           _     _____          _
 |  __ \    | |                              |  __ \         | |   |  __ \        | |
 | |__) |__ | | _____ _ __ ___   ___  _ __   | |__) |   _ ___| |_  | |__) |___  __| |
 |  ___/ _ \| |/ / _ \ '_ ` _ \ / _ \| '_ \  |  _  / | | / __| __| |  _  // _ \/ _` |
 | |  | (_) |   <  __/ | | | | | (_) | | | | | | \ \ |_| \__ \ |_  | | \ \  __/ (_| |
 |_|   \___/|_|\_\___|_| |_| |_|\___/|_| |_| |_|  \_\__,_|___/\__| |_|  \_\___|\__,_|
                                                                                     ").with(Red);
    println!("{}", red);
    loop {
        println!("\n1. Continue\n2. New Game");
        println!("\n{}", "WARNING: Options marked with a red TODO will likely crash the game.\nAvoid loss of data by saving frequently. - John".color(VERMILION));
        let game_select = get_user_input(2);
        match game_select {
            1 => {
                let mut loaded_game = load_game();
                match loaded_game {
                    Ok(_) => {
                        rust_red_game(loaded_game.unwrap());
                    }
                    Err(_) => {
                        eprint!("\nNo saved game detected")
                    }
                }
            }
            2 => {
                let mut game_state = GameState::new();
                let oak = "OAK";
                let poke = "POKEMON";
                let intro_msg = format!("{}: Hello there! Welcome to the world of {}! My name is {}! \
                \nPeople call me the {} PROFFESSOR! This world is inhabited by creatures called {}! \nFor \
                some people, {} are pets. Others use them for fights. \nMyself...I study {} as a profession.",
                                        oak.color(OAK), poke.color(YELLOW), oak.color(OAK),
                                        poke.color(YELLOW),poke.color(YELLOW),poke.color(YELLOW),
                                        poke.color(YELLOW),);
                type_text(intro_msg.as_str());
                let msg1 = "\n\nFirst, what is your name?\n";
                type_text(msg1);
                let player_name = read_user_input();
                game_state.player.enter_name(player_name.clone());
                let msg2 = format!("Welcome to the world of {} {}!\n", poke.color(YELLOW), player_name.cyan());
                let msg2 = msg2.as_str();
                type_text(msg2);
                type_text("\n. . . . . . .\n");
                rust_red_game(game_state);
            }
            _ => unreachable!(),
        }
    }
}

pub fn save_temp(game_state: &GameState){
    let json_data = serde_json::to_string(game_state);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("pokemon.json");
    file.expect("Error").write_all(json_data.expect("Error").as_bytes());
    println!("{}", "\nSAVE SUCCESSFUL".color(SAFFRON));
    //Ok(())
}
fn load_game()->Result<GameState, Box<dyn std::error::Error>>{
    let mut file = File::open("pokemon.json")?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;
    let loaded_pokemon: GameState = serde_json::from_str(&json_str)?;
    return Ok(loaded_pokemon);
}
// GameState will track all the key data such as whether events have triggered, player party etc.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    player: Player,
    pokedex: PokeDex,
    location: Regions,
    event: EventRec,
    pc: BillPC,
    last_used_pcentre: Regions,
    bag: Vec<StdItem>,
    badge_box: BadgeBox,
    defeated_trainers: Vec<u16>,
    starter: Starter
}
impl GameState {
    // At the start of a new game a blank GameState is created.
    fn new() -> GameState {
        GameState {
            player: Player {
                name: "".to_string(),
                party: Party {
                    mon: [None, None, None, None, None, None],
                },
                cash: 3000,
            },
            event: EventRec{
                starter_received: false,
                oaks_parcel_delivered: false,
                lavender_tower_ghost: false,
                snorlax_route12: false,
                snorlax_route16: false,
                lee_or_chan: false,
                omanyte_or_kabuto: false,
                articuno_encountered: false,
                zapdoz_encountered: false,
                moltres_encountered: false,
                mewtwo_encountered: false,
            },

            pokedex: PokeDex {},
            location: Regions::PalletTown(PalletTownLocations::RedsHouse),
            pc: BillPC{
                boxes: vec![],
            },
            last_used_pcentre: Regions::PalletTown(PalletTownLocations::RedsHouse),

            bag: vec![],
            badge_box: BadgeBox {
                boulder: false,
                cascade: false,
                thunder: false,
                rainbow: false,
                soul: false,
                marsh: false,
                volcano: false,
                earth: false,
            },
            defeated_trainers: Vec::new(),
            starter: Starter::Bulb,
        }
    }
    fn set_trainer_defeated(&mut self, trainer_id: u16){
        self.defeated_trainers.push(trainer_id);
    }
    fn is_trainer_defeated(&self, trainer_id:u16)->bool{
        self.defeated_trainers.contains(&trainer_id)
    }
    pub fn move_loc(&mut self, loc: Regions){
        self.location = loc;
    }
    fn move_mon_party_to_pc(&mut self){
        type_text("\nWhat Pokemon Would you like to put in you Box?\n");
        self.player.party.display_party();
        let selection = get_user_input(self.player.party.num_in_party() as u8);
        let selected_mon = self.player.party.mon[(selection-1)as usize].clone();
        self.pc.add_mon(selected_mon.unwrap());
        self.player.party.remove_party_member((selection.clone()) as usize);
    }
    fn move_mon_pc_to_party(&mut self){
        type_text("\nWhat Pokemon would you like to move to your Party\n");
        &self.pc.view_box();
        let selection = get_user_input(self.pc.boxes.len().clone() as u8);
        let selected_mon = self.pc.boxes[(selection-1) as usize].clone();
        if &self.player.party.num_in_party()< &(6 as usize) {
            self.player.party.add_party_member(selected_mon);
            self.pc.remove_mon((selection.clone()) as usize);
        }else{
            println!("Error, You already have a Full Party");
        }

    }
    fn load()->GameState{
        todo!()
    }
    fn save(&self){
        todo!()
    }
    fn trainer_battle(&mut self, trainer_id: u16)->bool{

        if !self.is_trainer_defeated(trainer_id){
            let mut trainer  = Trainer::get(trainer_id.clone());
            let result = battle2(self, &mut trainer);
            return if result {
                type_text(format!("\nYou win ${}!\n", trainer.reward.clone()).as_str());
                self.player.cash += trainer.reward;
                true
                //self.set_trainer_defeated(trainer_id.clone());
            } else {
                type_text("\nYou black out and wake up at the PokeCentre!\n");
                self.location = self.last_used_pcentre;
                false
            }
        }
        return true;
    }
}
#[derive(Debug, Serialize, Deserialize)]
enum Starter{
    Bulb,
    Charm,
    Squirt,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventRec{
    starter_received: bool,
    oaks_parcel_delivered: bool,
    lavender_tower_ghost: bool,
    snorlax_route12: bool,
    snorlax_route16: bool,
    lee_or_chan: bool,
    omanyte_or_kabuto: bool,
    articuno_encountered: bool,
    zapdoz_encountered: bool,
    moltres_encountered: bool,
    mewtwo_encountered: bool,
}
#[derive(Debug, Serialize, Deserialize)]
struct BillPC{
    boxes: Vec<Pokemon>,
}
impl BillPC{
    fn view_box(&self){
        let mut counter = 1;
        for mon in &self.boxes{
            println!("{}.{} - {}", counter, mon.name, mon.level);
            counter +=1;
        }
    }
    fn add_mon(&mut self, mon: Pokemon){
        self.boxes.push(mon);
    }
    fn remove_mon(&mut self, index: usize){
        self.boxes.remove(index-1);

        return
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct BadgeBox{
    boulder: bool,
    cascade: bool,
    thunder: bool,
    rainbow: bool,
    soul: bool,
    marsh: bool,
    volcano: bool,
    earth: bool,
}
// Player will be nested inside GameState and contain data specific to the player (party, items etc)
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    name: String,
    party: Party,
    cash: u16,
}
impl Player {
    pub fn enter_name(&mut self, name: String) {
        self.name = name;
    }
}
// TODO: This should get its own separate file.
// I will need some sort of array that flags whether each pokemon type has been seen or caught before.
// I will also need a list of all the pokemon + description, probably separate from base_stats.
#[derive(Debug, Serialize, Deserialize)]
struct PokeDex {}

impl PokeDex{

}

// For now having an array of 6 Pokemon Options seems like the right balance. For now I have the player
// using this and the enemy Trainers using a Vec<Pokemon> and I will see if one is clearly better.
#[derive(Debug, Serialize, Deserialize)]
struct Party {
    mon: [Option<Pokemon>; 6],
}
impl Party {
    // This needs to be re-written as a loop, as is its a hang-over from a previous design.
    pub fn add_party_member(&mut self, new_mon: Pokemon) {
        if self.mon[0] == None {
            self.mon[0] = Some(new_mon);
        } else if self.mon[1] == None {
            self.mon[1] = Some(new_mon);
        } else if self.mon[2] == None {
            self.mon[2] = Some(new_mon);
        } else if self.mon[3] == None {
            self.mon[3] = Some(new_mon);
        } else if self.mon[4] == None {
            self.mon[4] = Some(new_mon);
        } else if self.mon[5] == None {
            self.mon[5] = Some(new_mon);
        } else {
            println!("\nNo room for new pokemon!");
            //TODO - Send new_mon to "BILL's PC"
            // Maybe create a method on the main GameState struct that wraps this?
        }
    }
    pub fn party_menu(&mut self){
        loop {
            println!("\n Your Party:");
            self.display_party();
            println!("\nWhat would you like to do?");
            println!("1. Swap Pokemon");
            println!("2. Pokemon Stats");
            println!("9. Exit Menu");
            let selection = get_user_input(9);
            match selection {
                1 => self.swap_party_members(),
                2 => self.display_stats(),
                _ => {break}
            }
        }
    }
    // For Swapping the order of party members from the Overworld menu.
    fn swap_party_members(&mut self){
        println!("What Pokemon do you want to move?");
        self.display_party();
        let valid_pokemon = self.num_in_party();
        let selected_poke = get_user_input(valid_pokemon as u8);
        let selected_poke = selected_poke - 1;
        println!("What spot do you want to put {}", self.mon[selected_poke as usize].as_ref().unwrap().name);
        let selected_spot = get_user_input(valid_pokemon.clone() as u8);
        let selected_spot = selected_spot - 1;
        self.mon.swap(selected_poke.clone() as usize,selected_spot as usize);
    }
    fn display_stats(&self){
        println!("What Pokemon do you want to see stats for?");
        self.display_party();
        let valid_pokemon = self.num_in_party();
        let selected_poke =get_user_input(valid_pokemon as u8);
        let selected_poke = selected_poke - 1;
        println!("\nName: {} - Level: {}",self.mon[selected_poke as usize].as_ref().unwrap().name,self.mon[selected_poke as usize].as_ref().unwrap().level);
        println!("********************");
        println!("HP: {}", self.mon[selected_poke as usize].as_ref().unwrap().max_hp.value,);
        println!("Attack: {}", self.mon[selected_poke as usize].as_ref().unwrap().attk.value,);
        println!("Defense: {}", self.mon[selected_poke as usize].as_ref().unwrap().def.value,);
        println!("Special: {}", self.mon[selected_poke as usize].as_ref().unwrap().spec.value,);
        println!("Speed: {}", self.mon[selected_poke as usize].as_ref().unwrap().spd.value,);
    }
    fn remove_party_member(&mut self, index: usize){
        &self.mon[index-1].take();

        let mut i = 0;
        let len = self.mon.len();
        while i < len{
            if self.mon[i].is_none() {
                let next_non_none = self.mon.iter_mut().skip(i + 1).position(|x| x.is_some());
                if let Some(index) = next_non_none {
                    self.mon.swap(i, i + 1 + index);
                } else {
                    break;
                }
            }else{
                i +=1;
            }
        }
        return
    }

    fn num_in_party(&self)->usize{
        let mut counter = 0;
        for mon in &self.mon{
            if *mon !=None{
                counter+=1;
            }
        }
        return counter;
    }
    fn display_party(&self){
        let mut counter = 1;
        for poke in &self.mon{
            if *poke != None{
                println!("{}. {} - Lvl: {} ({}/{})", counter, poke.as_ref().unwrap().name, poke.as_ref().unwrap().level, poke.as_ref().unwrap().current_hp, poke.as_ref().unwrap().max_hp.value );
                counter+=1;
            }
            else {
                println!("NONE");
                counter +=1;
            }
        }

    }

    pub fn pokecentre_heal(&mut self){
        let mut party_member_rank = 0;
        for poke in &self.mon.clone() {
            if self.mon[party_member_rank.clone()] != None {

                if self.mon[party_member_rank.clone()].as_ref().unwrap().status != Healthy {
                    self.mon[party_member_rank.clone()].as_mut().unwrap().status = Healthy
                }
                self.mon[party_member_rank.clone()].as_mut().unwrap().current_hp = self.mon[party_member_rank.clone()].clone().unwrap().max_hp.value;
            }
            party_member_rank +=1;
        }
        type_text("\nYour Pokemon are all Healed!\n");
    }
    fn status_reset(&mut self){
        let mut party_member_rank = 0;
        for poke in &self.mon.clone() {
            if self.mon[party_member_rank.clone()] != None {
                if self.mon[party_member_rank.clone()].as_ref().unwrap().status != Fainted {
                    self.mon[party_member_rank.clone()].as_mut().unwrap().status = Healthy
                }
            }
            party_member_rank +=1;
        }
    }
}
// Should be moved to battle_logic.rs once that is finalized.
fn enemy_move_select(enemy: &Pokemon) -> u8 {
    let known_moves = enemy.moves.len();
    let mut rng = rand::thread_rng();
    let move_select = rng.gen_range(1..=known_moves);
    return move_select as u8;
}

// To be moved to battle_logic.rs
// Ideally the battle function will deal with both Trainer and Wild battles, I suspect using these Traits.
trait BattleParticipant {
    fn can_catch(&self) -> bool;
    fn can_run(&self) -> bool;
}
impl BattleParticipant for Trainer {
    fn can_catch(&self) -> bool {
        false
    }
    fn can_run(&self) -> bool {
        false
    }
}
impl BattleParticipant for Pokemon {
    fn can_catch(&self) -> bool {
        true
    }
    fn can_run(&self) -> bool {
        true
    }
}


// Currently the main use of this trait is to handle bringing in healthy pokemon into battles
// after another pokemon faints.
trait PartyOperations {
    fn check_all_fainted(&self) -> bool {
        false
    }
    fn return_first_healthy(&self) -> Result<usize, &str> {
        Err("Somethings Gone Wrong")
    }
}

impl PartyOperations for Party {
    fn check_all_fainted(&self) -> bool {
        for mon in &self.mon {
            if mon.as_ref() != None {
                if mon.as_ref().unwrap().status == Healthy {
                    return true;
                }
            }
        }
        false
    }
    fn return_first_healthy(&self) -> Result<usize, &str> {
        let mut counter: usize = 0;
        for mon in &self.mon {
            if mon.as_ref().unwrap().status == Healthy {
                return Ok(counter);
            }
            counter += 1;
        }
        Err("No Healthy Pokemon Found")
    }
}
#[derive(Debug)]
enum StatType{
    Attack,
    Defense,
    Speed,
    Special,
    Accuracy,
    Evasion,
}
// Each stat can be raised or lowered by up to 6 'stages' in a battle. This struct tracks that.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
struct BattleStats {
    attack: i8,
    defense: i8,
    speed: i8,
    special: i8,
    accuracy: i8,
    evasion: i8,
}
impl BattleStats{
    fn get_stat_mod(&self, stat_type: StatType)->f32{
        let mut stat_in_question = match stat_type{
            Attack=>&self.attack,
            Defense=>&self.defense,
            Speed=>&self.speed,
            Special=>&self.special,
            StatType::Accuracy=>&self.accuracy,
            StatType::Evasion=>&self.evasion,
        };
        let modifier = match stat_in_question {
            -6 => 0.25, // Stat reduced to 25%
            -5 => 0.29, // Stat reduced to 29%
            -4 => 0.33, // Stat reduced to 33%
            -3 => 0.4,  // Stat reduced to 40%
            -2 => 0.5,  // Stat reduced to 50%
            -1 => 0.67, // Stat reduced to 67%
            0 => 1.0,   // No modification to the base stat
            1 => 1.5,   // Stat increased by 50%
            2 => 2.0,   // Stat doubled
            3 => 2.5,   // Stat increased by 150%
            4 => 3.0,   // Stat increased by 200%
            5 => 3.5,   // Stat increased by 250%
            6 => 4.0,   // Stat increased by 300%
            _ => 1.0,   // Handle any other stage values as no modification
        };
        return modifier;
    }
    // Future: Some moves raise/lower stats by more than 1, for now this will be ignored.
    fn lower_stat(&mut self, stat_type: StatType){
        let mut stat_in_question = match stat_type{
            Attack=>self.attack,
            Defense=>self.defense,
            Speed=>self.speed,
            Special=>self.special,
            StatType::Accuracy=>self.accuracy,
            StatType::Evasion=>self.evasion,
        };
        if stat_in_question > -6{
            match stat_type{
                Attack=>{self.attack -=1},
                Defense=>{self.defense-=1},
                Speed=>{self.speed-=1},
                Special=>{self.special-=1},
                StatType::Accuracy=>{self.accuracy-=1},
                StatType::Evasion=>{self.evasion-=1},
            }
            //self.stat_type -= 1;
            //println!("STAT LOWERED, STAT NOW {}", stat_in_question)

        }else {
            println!("{:?} can't lowered any more!",stat_type);
        }
    }
    fn raise_stat(&mut self, stat_type: StatType){
        let mut stat_in_question = match stat_type{
            Attack=>self.attack,
            Defense=>self.defense,
            Speed=>self.speed,
            Special=>self.special,
            StatType::Accuracy=>self.accuracy,
            StatType::Evasion=>self.evasion,
        };
        if stat_in_question < 6{
            stat_in_question += 1;
        }else {
            println!("{:?} can't raised any more!",stat_type);
        }
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SpecialConditionFlags{
    leech_seeded: bool,
}

// Main Pokemon Struct
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Pokemon {
    name: String,
    species: PokemonSpecies,
    status: Status,
    current_hp: u16,
    level: u16,
    exp: u32,
    primary_type: PokeTypes,
    secondary_type: PokeTypes,
    max_hp: Stat,
    attk: Stat,
    def: Stat,
    spd: Stat,
    spec: Stat,
    first_move: Moves, // Dep
    second_move: Moves, // Dep
    moves: Vec<Moves>,
    base_exp: u16,
    stat_mod_stages: BattleStats,
    special_conditions: SpecialConditionFlags,
}
impl Pokemon {
    fn new(species: PokemonSpecies, level: u16) -> Self {
        fn leveler(level: &u16, stat: &Stat) -> u16 {
            let iv = &stat.iv;
            let value = &stat.value;
            let ev = &stat.ev;

            let levelled_stat =
                ((((value + iv) * 2 + (integer_square_root(ev) / 4)) * level) / 100) + 5;
            return levelled_stat;
        }
        fn hp_leveller(level: &u16, stat: &Stat)->u16{
            let iv = &stat.iv;
            let value = &stat.value;
            let ev = &stat.ev;


            let levelled_hp = ((((value + iv) * 2 + (integer_square_root(ev) / 4)) * level) / 100)
                + level
                + 10;

            return levelled_hp;
        }

        fn random_iv() -> u16 {
            let mut rng = rand::thread_rng();
            let iv_stat: u16 = rng.gen_range(0..=15);
            return iv_stat;
        }
        let base_stats = species.return_base_stats();

        let mut hit_point_stat = Stat {
            value: base_stats.base_hp.clone(),
            ev: 1,
            iv: random_iv(),
        };
        let mut attack_stat = Stat {
            value: base_stats.base_attack.clone(),
            ev: 1,
            iv: random_iv(),
        };
        let mut defense_stat = Stat {
            value: base_stats.base_defense.clone(),
            ev: 1,
            iv: random_iv(),
        };
        let mut speed_stat = Stat {
            value: base_stats.base_speed.clone(),
            ev: 1,
            iv: random_iv(),
        };
        let mut special_stat = Stat {
            value: base_stats.base_special.clone(),
            ev: 1,
            iv: random_iv(),
        };
        let (first_move, second_move) = base_stats.lvl1_moves.clone();

        let total_move_pool = LEARNABLEMOVES.iter().find(|data| data.species == species);
        let mut valid_move_pool: Vec<Moves> = vec![first_move, second_move];
        for (num, move_name) in total_move_pool.unwrap().level_up_moves{
            if num <= &level{
                valid_move_pool.push(*move_name);
            }
        }
        if valid_move_pool.len() > 4{
            valid_move_pool.shuffle(&mut rand::thread_rng());
            valid_move_pool.truncate(4);
        }

        attack_stat.value = leveler(&level, &attack_stat);
        defense_stat.value = leveler(&level, &defense_stat);
        speed_stat.value = leveler(&level, &speed_stat);
        special_stat.value = leveler(&level, &special_stat);
        hit_point_stat.value = hp_leveller(&level, &hit_point_stat);

        let exp = get_leveling_data(&level);

        Pokemon {
            name: base_stats.name.to_string(),
            species,
            status: Healthy,
            current_hp: hit_point_stat.value.clone(),
            level,
            exp,
            primary_type: base_stats.primary_type,
            secondary_type: base_stats.secondary_type,
            max_hp: hit_point_stat,
            attk: attack_stat,
            def: defense_stat,
            spd: speed_stat,
            spec: special_stat,
            moves: valid_move_pool,
            first_move,
            second_move,
            base_exp: base_stats.base_exp,
            stat_mod_stages: BattleStats{
                attack: 0,
                defense: 0,
                speed: 0,
                special: 0,
                accuracy: 0,
                evasion: 0,
            },
            special_conditions: SpecialConditionFlags { leech_seeded: false },
        }
    }

    fn damage(&mut self, attking_poke: &Pokemon, attcking_move: &Moves) {
        let move_data = attcking_move.move_stats();

        let move_cat = move_data.move_cat;

        let move_accuracy = move_data.accuracy;
        let defender_evasion = self.stat_mod_stages.evasion.clone();
        let attacker_accuracy = attking_poke.stat_mod_stages.accuracy.clone();
        /*
        println!("Move Accuracy: {}", move_accuracy);
        println!("Defender Evasion: {}", defender_evasion);
        println!("Attack Accuracy: {}", attacker_accuracy);
         */
        let mut hit = true;
        if !(move_accuracy == 100 && defender_evasion >=0 && attacker_accuracy >=0){
            //println!("IF VALIDATED");
            let accuracy_target = move_accuracy as f32 * self.stat_mod_stages.get_stat_mod(Evasion).clone() * attking_poke.stat_mod_stages.get_stat_mod(Accuracy).clone();
            //println!("{}", accuracy_target);
            let random_number = rand::thread_rng().gen_range(0..=100);
            if accuracy_target < random_number as f32 {
                hit = false;
            }
        }
        if hit {
            match move_cat {
                MoveCat::Physical | MoveCat::Special => {
                    let base_power = move_data.base_power as f32;
                    let move_type = move_data.move_type;
                    let attacker_level = attking_poke.level.clone() as f32;

                    let attacking_poke_type = &attcking_move.move_stats().move_type;
                    let defending_poke_type1 = &self.primary_type;
                    let defending_poke_type2 = &self.secondary_type;
                    let matchup_multiplier = attcking_move
                        .move_stats()
                        .move_type
                        .type_match_board(defending_poke_type1)
                        .effectivness_modifier();

                    let mut seconary_matchup_multi = 1;
                    /*
                if defending_poke_type2 != None{} todo!
                 */

                    attacking_poke_type
                        .type_match_board(defending_poke_type1)
                        .flavour_text();


                    let mut stab: f32 = 1.0;
                    if move_type == *attacking_poke_type {
                        stab = 1.5;
                    }
                    let mut defense;
                    let mut def_mod;
                    let mut attack;
                    let mut atk_mod;
                    if move_cat == Physical {
                        defense = self.def.value.clone() as f32;
                        def_mod = self.stat_mod_stages.get_stat_mod(Defense);
                        //if def_mod != 1.0 { println!("DEF MOD = {}", def_mod) }
                        defense = defense * def_mod;

                        attack = attking_poke.attk.value.clone() as f32;
                        atk_mod = attking_poke.stat_mod_stages.get_stat_mod(Attack);
                        //if atk_mod != 1.0 {println!("ATK MOD = {}", atk_mod)}
                        attack = attack * atk_mod;
                    } else {
                        defense = self.spec.value.clone() as f32;
                        def_mod = self.stat_mod_stages.get_stat_mod(Special);
                        //if def_mod != 1.0 { println!("DEF MOD = {}", def_mod) }
                        defense = defense * def_mod;
                        attack = attking_poke.spec.value.clone() as f32;
                        atk_mod = attking_poke.stat_mod_stages.get_stat_mod(Special);
                        //if atk_mod != 1.0 {println!("ATK MOD = {}", atk_mod)}
                        attack = attack * atk_mod;
                    }

                    let ad_ratio = attack / defense;

                    let dam = (((((2.0 * attacker_level) / 5.0 + 2.0) * base_power * ad_ratio) / 50.0) + 2.0)
                        * stab
                        * matchup_multiplier;
                    let damage = dam as u16;

                    type_text(format!("{} was hit for {} points of damage!\n", &self.name.clone().cyan(), damage).as_str());
                    if self.current_hp > damage {
                        self.current_hp -= damage;
                    } else {
                        self.current_hp = 0;
                        self.status = Fainted;
                    }
                    thread::sleep(Duration::from_millis(700));
                    attcking_move.move_stats().effect_type.apply_effect(self);
                },
                MoveCat::Status => {
                    attcking_move.move_stats().effect_type.apply_effect(self);
                    thread::sleep(Duration::from_millis(700));
                },
            }
        }else{
            type_text(format!("{}'s move missed!\n\n", attking_poke.name.clone().cyan()).as_str())
        }
    }
    fn battle_stats_reset(&mut self){
        self.stat_mod_stages.attack = 0;
        self.stat_mod_stages.defense = 0;
        self.stat_mod_stages.special = 0;
        self.stat_mod_stages.speed = 0;
        self.stat_mod_stages.evasion = 0;
        self.stat_mod_stages.accuracy = 0;
    }
    // I don't think this is used currently?
    fn move_names(&self) -> Vec<&str> {
        let mut move_names = Vec::new();
        for moves in &self.moves {
            move_names.push(moves.move_stats().name)
        }
        return move_names;
    }
    // MOVE Related Functions
    fn add_move(&mut self, moves: &Moves) -> Result<(), &'static str> {
        if self.moves.len() >= 4 {
            return Err("Cannot add another move");
        }
        self.moves.push(*moves);
        Ok(())
    }
    fn remove_move(&mut self, index: usize) {
        self.moves.remove(index);
    }
    fn replace_move(&mut self, index: usize, moves: &Moves) {
        self.moves[index] = *moves;
    }

    fn move_name(&self) -> &str {
        let move_stats = &self.first_move.move_stats();
        return move_stats.name;
    }
    fn second_move_name(&self) -> &str {
        let move_stats = &self.second_move.move_stats();
        return move_stats.name;
    }

    // EXP and LEVELLING Related Functions
    fn gain_exp(&mut self, foe: &Pokemon) {
        let enemy_level = foe.level.clone();
        let enemy_base_exp = foe.base_exp.clone();
        let exp_gain = (enemy_base_exp * enemy_level) / 7;
        type_text(format!("\nGained {} experience points!\n", exp_gain).as_str());
        thread::sleep(Duration::from_millis(600));
        self.exp += exp_gain as u32;
        self.check_level_up();
    }
    fn check_level_up(&mut self) {
        let next_level = &self.level + 1;
        let next_lvl_exp = get_leveling_data(&next_level);
        let current_exp = &self.exp;
        if current_exp >= &next_lvl_exp {
            type_text(format!("\n{} leveled up to {}!\n", self.name, next_level).as_str());
            self.evolve();
            self.level_up();
        }
        thread::sleep(Duration::from_millis(600));
    }
    fn level_up(&mut self) {
        // There is likely a cleaner way to do this with closures. Will return to this in future.
        fn leveler(level: &u16, stat: &Stat, species: &PokemonSpecies, stattype: StatType) -> u16 {
            let iv = &stat.iv;
            //let value = &stat.value;
            let ev = &stat.ev;
            let base_stats = species.return_base_stats();
            let mut relevant_stat = 0;
            let is_hp: bool;
            relevant_stat = match stattype {
                StatType::Attack => base_stats.base_attack,
                StatType::Defense => base_stats.base_defense,
                StatType::Speciall => base_stats.base_special,
                StatType::Speed => base_stats.base_speed,
                StatType::HPP => base_stats.base_hp,
            };
            if stattype == StatType::HPP {
                is_hp = true;
            } else {
                is_hp = false;
            }
            return if is_hp == false {
                let levelled_stat =
                    ((((relevant_stat + iv) * 2 + (integer_square_root(ev) / 4)) * level) / 100)
                        + 5;
                levelled_stat
            } else {
                let levelled_stat =
                    ((((relevant_stat + iv) * 2 + (integer_square_root(ev) / 4)) * level) / 100)
                        + level
                        + 10;

                levelled_stat
            };
        }
        #[derive(PartialEq)]
        enum StatType {
            Attack,
            Defense,
            Speed,
            Speciall,
            HPP,
        }
        self.level += 1;

        self.attk.value = leveler(&self.level, &self.attk, &self.species, StatType::Attack);
        self.def.value = leveler(&self.level, &self.def, &self.species, StatType::Defense);
        self.spd.value = leveler(&self.level, &self.spd, &self.species, StatType::Speed);
        self.spec.value = leveler(&self.level, &self.spec, &self.species, StatType::Speciall);
        self.max_hp.value = leveler(&self.level, &self.max_hp, &self.species, StatType::HPP);
        println!(
            "\n{}'s new stats are:\n\
        HP: {}\n\
        Attack:{}\n\
        Defense: {}\n\
        Speed: {}\n\
        Special: {}\n",
            self.name,
            self.max_hp.value,
            self.attk.value,
            self.def.value,
            self.spd.value,
            self.spec.value
        );
        thread::sleep(Duration::from_millis(1000));
        self.check_lvl_up_new_move();
    }
    fn check_lvl_up_new_move(&mut self) {
        let level = &self.level.clone();

        let species = &self.species;
        if let Some(lvlup_moves) = LEARNABLEMOVES.iter().find(|data| data.species == *species) {
            for (num, move_name) in lvlup_moves.level_up_moves {
                if num == level {
                    println!("{} can learn {:?}!", &self.name, move_name);
                    if &self.moves.len() >= &(4 as usize) {
                        println!(
                            "You already have 4 moves, choose which one you would like to forget."
                        );
                        let mut counter = 1;
                        for moves in &self.moves {
                            println!("{}. {}", counter, moves.move_stats().name);
                            counter += 1;
                        }
                        println!(
                            "Input 5 if you don't want to learn the new move {:?}",
                            move_name
                        );
                        let mut choice = true;
                        while choice {
                            let move_selection = read_user_input();
                            let move_selection = move_selection.as_str();
                            match move_selection {
                                "1" => {self.replace_move(0, move_name); choice = false}
                                "2" => {self.replace_move(1, move_name); choice = false}
                                "3" => {self.replace_move(2, move_name); choice = false}
                                "4" => {self.replace_move(3, move_name); choice = false}
                                "5" => {
                                    println!("Did not learn {:?}!", move_name);
                                    choice = false;
                                }
                                _ => {
                                    println!("Sorry, invalid choice!")
                                }
                            }
                        }
                    } else {
                        &self.add_move(move_name);
                    }
                }
            }
        }
    }
    fn evolve(&mut self){
        let level = self.level.clone();
        let level = level + 1;
        let current_species  = self.species.clone();

        let evo_data = self.species.return_evolution();

        if evo_data.next_stage != None{
            if evo_data.trigger == EvolutionTriggers::ByLevel(level){
                self.species = evo_data.next_stage.unwrap();
                let new_name = self.species.return_base_stats().name;
                self.name = new_name.to_string();
                println!("\nYour {:?} evolved into an {}!\n", current_species, self.name);
                thread::sleep(Duration::from_millis(1500));
            }
        }
    }

    fn burn_poison_effect(&mut self){
        let damage = (self.max_hp.value.clone())/8;
        let current_health = &self.current_hp;
        if self.status == Burned{
            type_text(format!("\n{} is hurt by its Burn!", self.name).as_str())
        }
        if self.status == Poisoned{
            type_text(format!("\n{} is hurt by Poison!", self.name).as_str());
        }
        type_text(format!("\n{} suffered {} points of damage!\n", self.name, damage).as_str());
        if current_health >= &damage {
            self.current_hp -= damage;
        } else {
            self.current_hp = 0;
        }
        thread::sleep(Duration::from_millis(600));
    }


    pub fn leech_seed_effect(&mut self, benefactor: &mut Pokemon){
        let seeding_damage = (self.max_hp.value.clone())/8;
        println!("{} leeched for {} HP", self.name.clone().cyan(), seeding_damage.to_string().red());
        thread::sleep(Duration::from_millis(500));
        if seeding_damage <= self.current_hp {
            self.current_hp -= seeding_damage;

        }else {
            println!("{} Fainted!", self.name);
            thread::sleep(Duration::from_millis(500));
            self.current_hp = 0;
            self.status = Fainted;
        }
        println!("{} healed!", benefactor.name);
        thread::sleep(Duration::from_millis(500));
        if (&benefactor.current_hp + seeding_damage.clone()) >= benefactor.max_hp.value{
            benefactor.current_hp = benefactor.max_hp.value.clone()
        }else {
            benefactor.current_hp += &seeding_damage;
        }
    }
}

#[derive(Clone, PartialEq, Debug, Copy, Serialize, Deserialize)]
struct Stat {
    value: u16,
    ev: u16,
    iv: u16,
}
// This should probably be an associated function of Pokemon struct.
fn get_leveling_data(level: &u16) -> u32 {
    let mut levelling_data: HashMap<ExpCat, Vec<u32>> = Default::default();
    levelling_data.insert(
        ExpCat::Fast,
        vec![
            0, 0, 6, 21, 51, 100, 172, 274, 409, 583, 800, 1064, 1382, 1757, 2195, 2700, 3276,
            3930, 4665, 5487, 6400, 7408, 8518, 9733, 11059, 12500, 14060, 15746, 17561, 19511,
            21600, 23832, 26214, 28749, 31443, 34300, 37324, 40522, 43897, 47455, 51200, 55136,
            59270, 63605, 68147, 72900, 77868, 83058, 88473, 94119, 100000,
        ],
    );
    levelling_data.insert(
        ExpCat::MediumFast,
        vec![
            0, 0, 6, 21, 51, 100, 172, 274, 409, 583, 800, 1064, 1382, 1757, 2195, 2700, 3276,
            3930, 4665, 5487, 6400, 7408, 8518, 9733, 11059, 12500, 14060, 15746, 17561, 19511,
            21600, 23832, 26214, 28749, 31443, 34300, 37324, 40522, 43897, 47455, 51200, 55136,
            59270, 64000, 68921, 74088, 79507, 85184, 91125, 97336, 103823, 110592, 117649, 125000,
        ],
    );
    levelling_data.insert(
        ExpCat::MediumSlow,
        vec![
            0, 0, 9, 57, 96, 135, 179, 236, 314, 419, 560, 742, 973, 1261, 1612, 2036, 2535, 3120,
            3798, 4575, 5460, 6458, 7577, 8825, 10208, 11735, 13411, 15244, 17242, 19411, 21760,
            24603, 27021, 29949, 33084, 36435, 40007, 43808, 47846, 52127, 56660, 61450, 66505,
            71833, 77440, 83335, 89523, 96012, 102810, 109923, 117360,
        ],
    );
    levelling_data.insert(
        ExpCat::Slow,
        vec![
            0, 10, 33, 80, 156, 270, 428, 640, 911, 1250, 1663, 2160, 2746, 3430, 4218, 5120, 6141,
            7290, 8573, 10000, 11576, 13310, 15208, 17280, 19531, 21970, 24603, 27440, 30486,
            33750, 37238, 40960, 44921, 49130, 53593, 58320, 63316, 68590, 74158, 800000, 86151,
            92610, 99383, 106480, 113906, 121670, 129778, 138240, 147061, 156250,
        ],
    );

    let med_slow = levelling_data.get(&ExpCat::MediumSlow);
    let exp_for_level = &med_slow.unwrap()[*level as usize];

    return exp_for_level.clone();
}

#[derive(Clone, PartialEq,Serialize, Deserialize )]
pub enum MoveCat {
    Physical,
    Special,
    Status,
}
#[derive(Clone, Serialize, Deserialize)]
pub enum MoveEffectCat {
    None,

    AttackDown1,
    DefenseDown1,
    SpeedDown1,
    SpecDown1,
    AccuracyDown1,
    EvasionDown1,

    AttackUp1,
    DefenseUp1,
    SpeedUp1,
    SpecUp1,
    AccuracyUp1,
    EvasionUp1,

    Burned,
    Sleeped,
    Frozen,
    Poisoned,
    Paralyzed,

    // Unique Moves
    LeechSeed,


    BurnSideEffect1,
}
impl MoveEffectCat {
    fn apply_effect(&self, target: &mut Pokemon) {
        match self {
            MoveEffectCat::Burned => {
                target.status = Burned;
                println!("{} suffers a burn!", target.name.clone().cyan());
            }
            MoveEffectCat::Poisoned=>{
                target.status = Poisoned;
                println!("{} was poisoned!", target.name.clone().cyan());
            }
            MoveEffectCat::Sleeped=>{
                target.status = Asleep;
                println!("{} was put to sleep!", target.name.clone().cyan());
            }
            MoveEffectCat::Frozen=>{target.status = Frozen;}
            MoveEffectCat::Paralyzed=>{target.status = Paralyzed;}

            MoveEffectCat::DefenseDown1=>{target.stat_mod_stages.lower_stat(Defense);
                println!("{} defense was {}!", target.name.clone().cyan(), Stylize::red("weakened"))},
            MoveEffectCat::AttackDown1=>{target.stat_mod_stages.lower_stat(Attack);
                println!("{} attack was {}!", target.name.clone().cyan(), Stylize::red("weakened"))},
            MoveEffectCat::SpeedDown1=> {target.stat_mod_stages.lower_stat(Speed);
                println!("{} speed was {}!", target.name.clone().cyan(), Stylize::red("weakened"))},
            MoveEffectCat::SpecDown1=>{target.stat_mod_stages.lower_stat(Special);
                println!("{} special was {}!", target.name.clone().cyan(), Stylize::red("weakened"))},
            MoveEffectCat::AccuracyDown1=>{target.stat_mod_stages.lower_stat(Accuracy);
                println!("{} accuracy was {}!", target.name.clone().cyan(), Stylize::red("weakened"))},
            MoveEffectCat::EvasionDown1=>{target.stat_mod_stages.lower_stat(Evasion);
                println!("{} evasion was {}!", target.name.clone().cyan(), Stylize::red("weakened"))},

            MoveEffectCat::AttackUp1=>target.stat_mod_stages.raise_stat(Attack),
            MoveEffectCat::DefenseUp1=>target.stat_mod_stages.raise_stat(Defense),
            MoveEffectCat::SpeedDown1=>target.stat_mod_stages.raise_stat(Speed),
            MoveEffectCat::SpecUp1=>target.stat_mod_stages.raise_stat(Special),

            MoveEffectCat::LeechSeed=>target.special_conditions.leech_seeded = true,

            MoveEffectCat::BurnSideEffect1=>{
                let random_number = rand::thread_rng().gen_range(0..=9);
                if random_number == 0{
                    target.status = Burned;
                    type_text(format!("\n{} suffered a burn!\n", target.name).as_str());
                }
            }

            _ => {}
        }
    }
}
// Every Move Effect will require an associated function. For the sake of organization these will be
// kept in a separate impl block.

#[derive(Eq, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum ExpCat {
    Fast,
    MediumFast,
    MediumSlow,
    Slow,
}
#[derive(Clone, PartialEq, Debug, Copy, Serialize, Deserialize)]
enum Status {
    Healthy,
    Fainted,
    Burned,
    Asleep,
    Frozen,
    Paralyzed,
    Poisoned,
}
/*
impl Status {
    fn status_effect(&self, target: &mut Pokemon) {
        match self {
            Burned => {
                let max_health = &target.max_hp;
                let burn_damage = max_health / 16;
                let current_health = &target.current_hp;

                if current_health >= &burn_damage {
                    target.current_hp -= burn_damage;
                } else {
                    target.current_hp = 0;
                }
            }
            _ => {}
        }
    }
}
 */

// Should be moved to a lib.rs file.
fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
fn integer_square_root(x: &u16) -> u16 {
    let y = *x as f32;
    let root_y = y.sqrt();
    let root_x = root_y as u16;
    root_x
}
fn type_text(text: &str) {
    let delay = 25;
    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(delay));
    }
}

fn enemy_ai() {}
