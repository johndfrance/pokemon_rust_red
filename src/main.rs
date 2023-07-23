use colored::Colorize;
mod game;
mod lib;
mod mon_base_stats;
mod mon_move_sets;
mod move_data;
mod type_matchups;
mod enemy_trainers;
mod temp_code;

use std::cmp::Ordering;
use crate::game::*;
use crate::mon_base_stats::*;
use crate::move_data::*;
use crate::type_matchups::*;
use crate::mon_base_stats::PokemonSpecies::{Charamander, Pidgey, Squirtle};
use crate::PokemonSpecies::{Bulbasaur, Pikachu};
use crate::Status::Burned;

use rand::Rng;
use std::collections::HashMap;
use std::fmt::format;
use std::io;
use std::io::Write;

use std::thread::sleep;
use std::time::Duration;
use crate::mon_move_sets::LEARNABLEMOVES;

use crate::move_data::Moves::Tackle;


fn main() {
    let mut game_state = GameState::new();
    rust_red_game(game_state);

    /*
    println!("POKEMON - RUST RED");
    let msg1 = "\nWhat is your name?";
    type_text(msg1);

    let player_name = read_user_input();
    let mut player = Player {
        name: player_name.clone(),
        poke_team: vec![],
        cash: 0,
    };
    let msg2 = format!("Welcome to the world of Pokemon {}!\n", player_name);
    let msg2 = msg2.as_str();
    type_text(msg2);
    println!(
        "Choose your starting Pokemon: \n\
    1. Bulbasaur\n\
    2. Charmander\n\
    3. Squirtle\n\
    4. Pikachu"
    );
    let mut choice = true;
    let bulbasaur = Pokemon::new(Bulbasaur, 6);
    let charmander = Pokemon::new(Charamander, 5);
    let squirtle = Pokemon::new(Squirtle, 5);
    let pikachu = Pokemon::new(Pikachu, 5);

    while choice {
        let starter_choice = read_user_input();
        let starter_choice = starter_choice.as_str();
        match starter_choice {
            "1" => {
                player.poke_team.push(bulbasaur.clone());
                choice = false;
            }
            "2" => {
                player.poke_team.push(charmander.clone());
                choice = false;
            }
            "3" => {
                player.poke_team.push(squirtle.clone());
                choice = false;
            }
            "4" => {
                player.poke_team.push(pikachu.clone());
                choice = false;
            }
            _ => println!("Sorry, that wasn't a valid choice."),
        }
    }
    game(player);

     */
}
pub struct GameState{
    player: Player,
    pokedex: PokeDex,

    //enemy_trainers: HashMap<u16, Bool>,
}
impl GameState{
    fn new()->GameState{
        GameState{
            player: Player{
                name: "".to_string(),
                poke_team: vec![],
                party: Party {
                    mon1: None,
                    mon2: None,
                    mon3: None,
                    mon4: None,
                    mon5: None,
                    mon6: None,
                },
                cash: 100,
            },
            pokedex: PokeDex {},
            //enemy_trainers: Default::default(),
        }
    }
}

pub struct Player{
    name: String,
    poke_team: Vec<Pokemon>,
    party: Party,
    cash: u16,
}
impl Player{
    pub fn enter_name(&mut self, name: String){
        self.name = name;
    }

}
struct PokeDex{}
struct Party{
    mon1: Option<Pokemon>,
    mon2: Option<Pokemon>,
    mon3: Option<Pokemon>,
    mon4: Option<Pokemon>,
    mon5: Option<Pokemon>,
    mon6: Option<Pokemon>,
}

impl Party {
    pub fn add_party_member(&mut self, new_mon: Pokemon){
        if self.mon1 == None{
            self.mon1 = Some(new_mon);
        }else if self.mon2 == None {
            self.mon2 = Some(new_mon);
        }else if self.mon3 == None {
            self.mon3 == Some(new_mon);
        }else if self.mon4 == None {
            self.mon4 == Some(new_mon);
        }else if self.mon5 == None {
            self.mon5 == Some(new_mon);
        }else if self.mon6 == None{
            self.mon6 == Some(new_mon);
        }else{
            println!("\nNo room for new pokemon!");
            //TODO - Send new_mon to "BILL's PC"
        }
    }
}
/*
struct Events{
    enemy_trainers: HashMap<u16, Bool>,
    events: HashMap<u16, Bool>
}
*/
/*
fn game(mut player: Player) {
    let mut player1 = player.poke_team[0].clone();

    let mut opponent_count = 1;
    loop {
        let enemy = generate_enemy();
        println!(
            "\nEnemy {}: {} - Level: {}",
            opponent_count, enemy.name, enemy.level
        );
        println!("*************************************");
        let still_alive = battle(&mut player1, enemy);
        if !still_alive {
            break;
        }
        opponent_count += 1;
    }
    println!("GAME OVER")
}

fn generate_enemy() -> Pokemon {
    let variants = [
        Pikachu,
        Bulbasaur,
        Charamander,
        Squirtle,
        Pidgey,
        // Add more Pokémon variants
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..variants.len());
    let mut lvl_rng = rand::thread_rng();
    let enemy_lvl = lvl_rng.gen_range(1..5);
    let enemy_species = variants[index].clone();
    let enemy = Pokemon::new(enemy_species, enemy_lvl);
    return enemy;
}

 */
fn enemy_move_select(enemy: &Pokemon)->u8{
    let known_moves = enemy.moves.len();
    let mut rng = rand::thread_rng();
    let move_select = rng.gen_range(1..known_moves);
    return move_select as u8;
}

fn battle(game_state: &mut GameState, enemy: &mut Trainer) -> bool {
    let player_name = game_state.player.name.clone();
    let enemy_name = enemy.name.clone();
    let winner: bool;
    //let msg = format!("{} has challenged you to a battle!", enemy_name);
    type_text(format!("{} has challenged you to a battle!\n", enemy_name).as_str());

    loop{

        type_text(format!("{} sends out {}\n",enemy_name, enemy.poke_team[0].name ).as_str());
        type_text(format!("{} sends out {:?}\n", player_name, game_state.player.party.mon1.clone().unwrap().name).as_str());
        loop{
            println!("Player {}(LVL{}) has {}/{:?}HP.",
                     game_state.player.party.mon1.clone().unwrap().name,
                    game_state.player.party.mon1.clone().unwrap().level,
                    game_state.player.party.mon1.clone().unwrap().current_hp,
                     game_state.player.party.mon1.clone().unwrap().max_hp.value);
            println!("Enemy {}(LVL{}) has {}/{:?}HP.",
                enemy.poke_team[0].name,
                enemy.poke_team[0].level,
                enemy.poke_team[0].current_hp,
                enemy.poke_team[0].max_hp.value);

            let mut move_count = 1;
            for moves in game_state.player.party.mon1.clone().unwrap().moves{
                print!(" {}. {}", move_count, moves.move_stats().name);
                move_count +=1;
            }
            io::stdout().flush().unwrap();
            let mut valid_move_picked = false;
            let mut player1_input;

            //while !valid_move_picked{
                player1_input = read_user_input();
                valid_move_picked = move_selection_validator(&game_state, &player1_input);
                //let player1_input = player1_input.as_str(); }

            let player1_input = player1_input.as_str();
            fn move_selection_validator(game_state: &GameState, selected_move: &String) ->bool{
                let number_of_moves_known = game_state.player.party.mon1.clone().unwrap().moves.len();
                if selected_move.parse::<usize>().is_ok(){
                    if  selected_move.parse::<usize>().unwrap() > number_of_moves_known{
                        type_text("No known move in that slot!");
                        return false
                    }
                    else { return true }

                }else {
                    type_text("Please enter a valid move with a 1, 2, 3, or 4.");
                    return false
                }
            }


            let player_selected_move = match player1_input{
                "1"=>{game_state.player.party.mon1.clone().unwrap().moves[0]}
                "2"=>{game_state.player.party.mon1.clone().unwrap().moves[1]}
                "3"=>{game_state.player.party.mon1.clone().unwrap().moves[2]}
                "4"=>{game_state.player.party.mon1.clone().unwrap().moves[3]}
                _ => {game_state.player.party.mon1.clone().unwrap().moves[0]} // This could be eliminated by using an enum of [1, 2, 3, 4]
            };
            let enemy_move_selection = enemy_move_select(&enemy.poke_team[0]).to_string();
            let enemy_move_selection = enemy_move_selection.as_str();
            let enemy_move_selection= match enemy_move_selection {
                "1"=>{enemy.poke_team[0].moves[0]}
                "2"=>{enemy.poke_team[0].moves[1]}
                "3"=>{enemy.poke_team[0].moves[2]}
                "4"=>{enemy.poke_team[0].moves[3]}
                _=>{enemy.poke_team[0].moves[0]}
            };

            let speed_order = game_state.player.party.mon1.clone().unwrap().spd.value.cmp(&enemy.poke_team[0].spd.value);
            match speed_order{
                Ordering::Greater=>{
                    enemy.poke_team[0].damage(&game_state.player.party.mon1.clone().unwrap(), &player_selected_move);
                    if enemy.poke_team[0].current_hp == 0{
                        type_text("Enemy Fainted!");
                        winner = true;
                        break
                    }else {
                        game_state.player.party.mon1.as_mut().unwrap().damage(&enemy.poke_team[0], &enemy_move_selection);
                        if game_state.player.party.mon1.clone().unwrap().current_hp == 0{
                            type_text("You Fainted!");
                            winner = false;
                            break
                        }
                    }
                }
                Ordering::Less=>{
                    game_state.player.party.mon1.as_mut().unwrap().damage(&enemy.poke_team[0], &enemy_move_selection);
                    if game_state.player.party.mon1.clone().unwrap().current_hp == 0{
                        type_text("You Fainted!");
                        winner = false;
                        break
                    }else {
                        enemy.poke_team[0].damage(&game_state.player.party.mon1.clone().unwrap(), &player_selected_move);
                        if enemy.poke_team[0].current_hp == 0{
                            type_text("Enemy Fainted!");
                            winner = true;
                            break
                        }
                    }
                }
                Ordering::Equal=>{enemy.poke_team[0].damage(&game_state.player.party.mon1.clone().unwrap(), &player_selected_move);
                    if enemy.poke_team[0].current_hp == 0{
                        type_text("Enemy Fainted!");
                        winner = true;
                        break
                    }else {
                        game_state.player.party.mon1.as_mut().unwrap().damage(&enemy.poke_team[0], &enemy_move_selection);
                        if game_state.player.party.mon1.clone().unwrap().current_hp == 0{
                            type_text("You Fainted!");
                            winner = false;
                            break
                        }
                    }
                }
            }
        }
        break
    }
    if winner == true {
        game_state.player.party.mon1.as_mut().unwrap().gain_exp(&enemy.poke_team[0]);
        game_state.player.party.mon1.as_mut().unwrap().check_level_up();
    }
    return winner;
}



struct Trainer {
    name: &'static str,
    poke_team: Vec<Pokemon>,
    cash: u16,
}

impl Trainer {
    pub fn new()->Trainer{
        Trainer{
            name: "Blue",
            poke_team: vec![Pokemon::new(Squirtle, 5)],
            cash: 100,
        }
    }

}

fn apply_stat_modifier(base_stat: u16, stages: i32) -> u16 {
    let modifier = match stages {
        -6 => 0.25, // Stat reduced to 25% of the base value
        -5 => 0.29, // Stat reduced to 29% of the base value
        -4 => 0.33, // Stat reduced to 33% of the base value
        -3 => 0.4,  // Stat reduced to 40% of the base value
        -2 => 0.5,  // Stat reduced to 50% of the base value
        -1 => 0.67, // Stat reduced to 67% of the base value
        0 => 1.0,   // No modification to the base stat
        1 => 1.5,   // Stat increased by 50% of the base value
        2 => 2.0,   // Stat doubled
        3 => 2.5,   // Stat increased by 150% of the base value
        4 => 3.0,   // Stat increased by 200% of the base value
        5 => 3.5,   // Stat increased by 250% of the base value
        6 => 4.0,   // Stat increased by 300% of the base value
        _ => 1.0,   // Handle any other stage values as no modification
    };

    (base_stat as f32 * modifier) as u16
}

struct BattleStats {
    attack: u8,
    defense: u8,
    speed: u8,
    special: u8,
    accuracy: u8,
    evasion: u8,
}

#[derive(Clone, PartialEq, Debug)]
struct Pokemon {
    name: String,
    species: PokemonSpecies,
    status: Status,
    current_hp: u16,
    level: u16,
    exp: u32,
    primary_type: PokeTypes,
    max_hp: Stat,
    attk: Stat,
    def: Stat,
    spd: Stat,
    spec: Stat,
    first_move: Moves,
    second_move: Moves,

    moves: Vec<Moves>,

    base_exp: u16,
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
        hit_point_stat.value = leveler(&level, &hit_point_stat);

        let exp = get_leveling_data(&level);

        Pokemon {
            name: base_stats.name.to_string(),
            species,
            status: Status::Healthy,
            current_hp: hit_point_stat.value.clone(),
            level,
            exp,
            primary_type: base_stats.primary_type,
            max_hp: hit_point_stat,
            attk: attack_stat,
            def: defense_stat,
            spd: speed_stat,
            spec: special_stat,
            moves: vec![first_move.clone(), second_move.clone()],
            first_move,
            second_move,

            base_exp: base_stats.base_exp,
        }
    }

    fn damage(&mut self, attking_poke: &Pokemon, attcking_move: &Moves) {
        let move_data = attcking_move.move_stats();
        let base_power = move_data.base_power as f32;
        let move_type = move_data.move_type;
        let attacker_level = attking_poke.level.clone() as f32;

        let attacking_poke_type = &attcking_move.move_stats().move_type;
        let defending_poke_type = &self.primary_type;
        let matchup_multiplier = attcking_move.move_stats().move_type
            .type_match_board(defending_poke_type)
            .effectivness_modifier();
        attacking_poke_type.type_match_board(defending_poke_type).flavour_text();

        let mut stab: f32 = 1.0;
        if move_type == *attacking_poke_type {
            stab = 1.5;
        }
        let defense = self.def.value.clone() as f32;
        let attack = attking_poke.attk.value.clone() as f32;

        let ad_ratio = attack / defense;

        let dam = ((((2.0 * attacker_level + 2.0) * base_power * ad_ratio) / 50.0) + 2.0)
            * stab
            * matchup_multiplier;
        let damage = dam as u16;

        println!("{} points of damage!", damage);
        if self.current_hp > damage {
            self.current_hp -= damage;
        } else {
            self.current_hp = 0;
        }
    }
    fn move_names(&self)->Vec<&str>{
        let mut move_names = Vec::new();
        for moves in &self.moves{
            move_names.push(moves.move_stats().name)
        }
        return move_names;
    }

    fn add_move(&mut self, moves: &Moves) -> Result<(), &'static str>{
        if self.moves.len() >= 4{
            return Err("Cannot add another move");
        }
        self.moves.push(*moves);
        Ok(())
    }
    fn remove_move(&mut self, index: usize){
        self.moves.remove(index);
    }
    fn replace_move(&mut self, index: usize, moves: &Moves){
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
    fn gain_exp(&mut self, foe: &Pokemon) {
        let enemy_level = foe.level.clone();
        let enemy_base_exp = foe.base_exp.clone();
        let exp_gain = (enemy_base_exp * enemy_level) / 7;
        println!("Gained {} experience points!", exp_gain);
        self.exp += exp_gain as u32;
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
            relevant_stat = match stattype {
                StatType::Attack => base_stats.base_attack,
                StatType::Defense => base_stats.base_defense,
                StatType::Speciall => base_stats.base_special,
                StatType::Speed => base_stats.base_speed,
                StatType::HPP => base_stats.base_hp,
            };

            //TODO Levelling HP uses a different formula.

            let levelled_stat =
                ((((relevant_stat + iv) * 2 + (integer_square_root(ev) / 4)) * level) / 100) + 5;
            return levelled_stat;
        }
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
    fn check_lvl_up_new_move(&mut self){
        let level = &self.level.clone();
        let species = &self.species;
        if let Some(lvlup_moves) = LEARNABLEMOVES.iter().find(|data| data.species == *species){
            for (num, move_name) in lvlup_moves.level_up_moves{
                if num == level{
                    println!("{} can learn {:?}!", &self.name, move_name);
                    if &self.moves.len() >= &(4 as usize) {
                        println!("You already have 4 moves, choose which one you would like to forget.");
                        let mut counter = 1;
                        for moves in &self.moves{
                            println!("{}. {}", counter, moves.move_stats().name);
                            counter +=1;
                        }
                        println!("Input 5 if you don't want to learn the new move {:?}", move_name);
                        let mut choice = true;
                        while choice{
                            let move_selection = read_user_input();
                            let move_selection = move_selection.as_str();
                            match move_selection{
                                "1"=>{self.replace_move(0, move_name)},
                                "2"=>{self.replace_move(1, move_name)},
                                "3"=>{self.replace_move(2, move_name)},
                                "4"=>{self.replace_move(3, move_name)},
                                "5"=>{println!("Did not learn {:?}!", move_name)},
                                _=>{println!("Sorry, invalid choice!")},
                            }

                        }
                    }else{
                        &self.add_move(move_name);
                    }
                }
            }
        }
    }
    fn modify_stats() {}
}
#[derive(Clone, PartialEq, Debug)]
struct Stat {
    value: u16,
    ev: u16,
    iv: u16,
}

fn get_leveling_data(level: &u16) -> u32 {
    let mut levelling_data: HashMap<ExpCat, Vec<u32>> = Default::default();
    levelling_data.insert(
        ExpCat::MediumSlow,
        vec![
            0, 0, 9, 57, 96, 135, 179, 236, 314, 419, 560, 742, 973, 1261, 1612, 2036, 2535, 3120,
            3798, 4575, 5460, 6458, 7577, 8825, 10208, 11735, 13411, 15244, 17242, 19411, 21760,
            24603, 27021, 29949, 33084, 36435, 40007, 43808, 47846, 52127, 56660, 61450, 66505,
            71833, 77440, 83335, 89523, 96012, 102810, 109923, 117360,
        ],
    );

    let med_slow = levelling_data.get(&ExpCat::MediumSlow);
    let exp_for_level = &med_slow.unwrap()[*level as usize];

    return exp_for_level.clone();
}

#[derive(Clone)]
pub enum MoveCat {
    Physical,
    Special,
    Status,
}
#[derive(Clone)]
pub enum MoveEffectCat {
    None,
    BurnSideEffect1,
    SpeedDownSideEffect,
    DefenseDown1,
}

impl MoveEffectCat {
    fn apply_effect(&self, target: &mut Pokemon) {
        match self {
            MoveEffectCat::BurnSideEffect1 => {
                target.status = Burned;
            }
            MoveEffectCat::SpeedDownSideEffect => {}
            _ => {}
        }
    }
}
// Every Move Effect will require an associated function. For the sake of organization these will be
// kept in a separate impl block.

#[derive(Eq, Hash, PartialEq)]
enum ExpCat {
    Fast,
    MediumFast,
    MediumSlow,
    Slow,
}
#[derive(Clone, PartialEq, Debug)]
enum Status {
    Healthy,
    Fainted,
    Burned,
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

fn enemy_ai(){}
