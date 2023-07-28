use colored::Colorize;
mod battle_logic;
mod enemy_trainers;
mod evolution;
mod game;
mod lib;
mod mon_base_stats;
mod mon_move_sets;
mod move_data;
mod npc_dialogue;
//mod temp_code;
mod type_matchups;
mod items;
mod wild_battle_logic;
mod pokemon_structure;

use crate::game::*;

use crate::mon_base_stats::PokemonSpecies::{Charamander, Pidgey, Rattata, Squirtle};
use crate::mon_base_stats::*;
use crate::move_data::*;
use crate::type_matchups::*;
use crate::PokemonSpecies::{Bulbasaur, Pikachu};
use crate::Status::{Asleep, Burned, Fainted, Frozen, Healthy, Paralyzed, Poisoned};
use crate::mon_move_sets::LEARNABLEMOVES;
use crate::move_data::Moves::Tackle;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::format;
use std::{env, io};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use crate::enemy_trainers::Trainer;
use crate::MoveCat::Physical;
use crate::StatType::{Attack, Defense, Special, Speed};

// MAIN
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // TODO: Impl 'Continue/New Game' menu
    println!("Do you want to:\n1.New Game.\n2.Continue\n3.Debug");



    let mut game_state = GameState::new();
    rust_red_game(game_state);
}
// GameState will track all the key data such as whether events have triggered, player party etc.
pub struct GameState {
    player: Player,
    pokedex: PokeDex,
    location: Regions,
    //enemy_trainers: HashMap<u16, Bool>,
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
                cash: 100,
            },
            pokedex: PokeDex {},
            location: Regions::PalletTown(PalletTownLocations::RedsHouse),
            //enemy_trainers: Default::default(),
        }
    }
    pub fn move_loc(&mut self, loc: Regions){
        self.location = loc;
    }
    fn load()->GameState{
        todo!()
    }
    fn save(&self){
        todo!()
    }
}
// Player will be nested inside GameState and contain data specific to the player (party, items etc)
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
struct PokeDex {}

// For now having an array of 6 Pokemon Options seems like the right balance. For now I have the player
// using this and the enemy Trainers using a Vec<Pokemon> and I will see if one is clearly better.
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
            self.mon[2] == Some(new_mon);
        } else if self.mon[3] == None {
            self.mon[3] == Some(new_mon);
        } else if self.mon[4] == None {
            self.mon[4] == Some(new_mon);
        } else if self.mon[5] == None {
            self.mon[5] == Some(new_mon);
        } else {
            println!("\nNo room for new pokemon!");
            //TODO - Send new_mon to "BILL's PC"
        }
    }
    pub fn pokecentre_heal(&mut self){
        if self.mon[0] != None{
            /*
            println!("DEBUG - {} CURRENTLY HAS {}/{} HP",
                     self.mon[0].as_ref().unwrap().name,
                     self.mon[0].as_ref().unwrap().current_hp,
                     self.mon[0].as_ref().unwrap().max_hp.value);
             */
            if self.mon[0].as_ref().unwrap().status != Healthy{
                self.mon[0].as_mut().unwrap().status = Healthy
            }
            self.mon[0].as_mut().unwrap().current_hp = self.mon[0].clone().unwrap().max_hp.value;
        }
        /*
        println!("DEBUG - {} NOW HAS {}/{} HP",
                 self.mon[0].as_ref().unwrap().name,
                 self.mon[0].as_ref().unwrap().current_hp,
                 self.mon[0].as_ref().unwrap().max_hp.value);
         */
        type_text("\nYour Pokemon are all Healed!\n");
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
                /*
                let healthy_mon = mon.clone().unwrap();
                return Ok(healthy_mon);
                 */
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
#[derive(Clone, PartialEq, Debug)]
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
            StatType::Attack=>&self.attack,
            StatType::Defense=>&self.defense,
            StatType::Speed=>&self.speed,
            StatType::Special=>&self.special,
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
            StatType::Attack=>self.attack,
            StatType::Defense=>self.defense,
            StatType::Speed=>self.speed,
            StatType::Special=>self.special,
            StatType::Accuracy=>self.accuracy,
            StatType::Evasion=>self.evasion,
        };
        if stat_in_question > -6{
            stat_in_question -= 1;
        }else {
            println!("{:?} can't lowered any more!",stat_type);
        }
    }
    fn raise_stat(&mut self, stat_type: StatType){
        let mut stat_in_question = match stat_type{
            StatType::Attack=>self.attack,
            StatType::Defense=>self.defense,
            StatType::Speed=>self.speed,
            StatType::Special=>self.special,
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
#[derive(Debug, PartialEq, Clone)]
struct SpecialConditionFlags{
    leech_seeded: bool,
}

// Main Pokemon Struct
#[derive(Clone, PartialEq, Debug)]
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

            // TODO: HP uses different formula.

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
        attack_stat.value = leveler(&level, &attack_stat);
        defense_stat.value = leveler(&level, &defense_stat);
        speed_stat.value = leveler(&level, &speed_stat);
        special_stat.value = leveler(&level, &special_stat);
        hit_point_stat.value = hp_leveller(&level, &hit_point_stat);

        let exp = get_leveling_data(&level);

        Pokemon {
            name: base_stats.name.to_string(),
            species,
            status: Status::Healthy,
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
            moves: vec![first_move.clone(), second_move.clone()],
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

        match move_cat{
            MoveCat::Physical|MoveCat::Special=>{

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
                if move_cat == Physical{
                    defense = self.def.value.clone() as f32;
                    def_mod = self.stat_mod_stages.get_stat_mod(Defense);
                    //if def_mod != 1.0 { println!("DEF MOD = {}", def_mod) }
                    defense = defense*def_mod;

                    attack = attking_poke.attk.value.clone() as f32;
                    atk_mod = attking_poke.stat_mod_stages.get_stat_mod(Attack);
                    //if atk_mod != 1.0 {println!("ATK MOD = {}", atk_mod)}
                    attack = attack*atk_mod;

                }else{
                    defense = self.spec.value.clone() as f32;
                    def_mod = self.stat_mod_stages.get_stat_mod(Special);
                    //if def_mod != 1.0 { println!("DEF MOD = {}", def_mod) }
                    defense = defense*def_mod;
                    attack = attking_poke.spec.value.clone() as f32;
                    atk_mod = attking_poke.stat_mod_stages.get_stat_mod(Special);
                    //if atk_mod != 1.0 {println!("ATK MOD = {}", atk_mod)}
                    attack = attack*atk_mod;
                }

                let ad_ratio = attack / defense;

                let dam = ((((2.0 * attacker_level + 2.0) * base_power * ad_ratio) / 50.0) + 2.0)
                    * stab
                    * matchup_multiplier;
                let damage = dam as u16;

                type_text(format!("{} was hit for {} points of damage!\n", &self.name.cyan(), damage).as_str());
                if self.current_hp > damage {
                    self.current_hp -= damage;
                } else {
                    self.current_hp = 0;
                    self.status = Fainted;
                }

            },
            MoveCat::Status=>{
                attcking_move.move_stats().effect_type.apply_effect(self)

            },
        }
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
        println!("Gained {} experience points!", exp_gain);
        self.exp += exp_gain as u32;
        self.check_level_up();
    }
    fn check_level_up(&mut self) {
        let next_level = &self.level + 1;
        let next_lvl_exp = get_leveling_data(&next_level);
        let current_exp = &self.exp;
        if current_exp >= &next_lvl_exp {
            println!("{} leveled up to {}!", self.name, next_level);
            self.level_up();
        }
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
            "{}'s new stats are:\n\
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
                                "1" => self.replace_move(0, move_name),
                                "2" => self.replace_move(1, move_name),
                                "3" => self.replace_move(2, move_name),
                                "4" => self.replace_move(3, move_name),
                                "5" => {
                                    println!("Did not learn {:?}!", move_name)
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

    pub fn leech_seed_effect(&mut self, benefactor: &mut Pokemon){
        let seeding_damage = (self.max_hp.value.clone())/8;
        println!("{} leeched for {} HP", self.name, seeding_damage);
        if seeding_damage <= self.current_hp {
            self.current_hp -= seeding_damage;

        }else {
            println!("{} Fainted!", self.name);
            self.current_hp = 0;
            self.status = Fainted;
        }
        println!("{} healed!", benefactor.name);
        if (&benefactor.current_hp + seeding_damage.clone()) >= benefactor.max_hp.value{
            benefactor.current_hp = benefactor.max_hp.value.clone()
        }else {
            benefactor.current_hp += &seeding_damage;
        }
    }
}

#[derive(Clone, PartialEq, Debug, Copy)]
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

#[derive(Clone, PartialEq, )]
pub enum MoveCat {
    Physical,
    Special,
    Status,
}
#[derive(Clone)]
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
            MoveEffectCat::Burned => {target.status = Burned;}
            MoveEffectCat::Poisoned=>{target.status = Poisoned;}
            MoveEffectCat::Sleeped=>{target.status = Asleep;}
            MoveEffectCat::Frozen=>{target.status = Frozen;}
            MoveEffectCat::Paralyzed=>{target.status = Paralyzed;}

            MoveEffectCat::DefenseDown1=>{target.stat_mod_stages.lower_stat(Defense); println!("{} defense was {}!", target.name.cyan(), "weakened".red())},
            MoveEffectCat::AttackDown1=>{target.stat_mod_stages.lower_stat(Attack); println!("{} attack was {}!", target.name.cyan(), "weakened".red())},
            MoveEffectCat::SpeedDown1=> target.stat_mod_stages.lower_stat(Speed),
            MoveEffectCat::SpecDown1=>target.stat_mod_stages.lower_stat(Special),

            MoveEffectCat::AttackUp1=>target.stat_mod_stages.raise_stat(Attack),
            MoveEffectCat::DefenseUp1=>target.stat_mod_stages.raise_stat(Defense),
            MoveEffectCat::SpeedDown1=>target.stat_mod_stages.raise_stat(Speed),
            MoveEffectCat::SpecUp1=>target.stat_mod_stages.raise_stat(Special),


            MoveEffectCat::LeechSeed=>target.special_conditions.leech_seeded = true,

            _ => {}
        }
    }
}
// Every Move Effect will require an associated function. For the sake of organization these will be
// kept in a separate impl block.

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum ExpCat {
    Fast,
    MediumFast,
    MediumSlow,
    Slow,
}
#[derive(Clone, PartialEq, Debug, Copy)]
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
    let delay = 10;
    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(delay));
    }
}

fn enemy_ai() {}
