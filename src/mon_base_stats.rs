use crate::move_data::Moves;

use crate::move_data::Moves::{Growl, Scratch, Tackle, TailWhip, ThunderShock};
use crate::PokeTypes;

pub struct PokemonBaseData {
    dex_num: u8,
    pub name: &'static str,
    pub base_hp: u16,
    pub base_attack: u16,
    pub base_defense: u16,
    pub base_speed: u16,
    pub base_special: u16,
    pub primary_type: PokeTypes,
    //pub secondary_type: PokeTypes,
    pub base_exp: u16,
    pub lvl1_moves: (Moves, Moves),
    //ExpCat?

}

#[derive(Clone, PartialEq, Debug)]
pub enum PokemonSpecies {
    Bulbasaur,
    Charamander,
    Squirtle,
    Caterpie,
    Weedle,
    Pidgey,
    Rattata,
    Pikachu,
}

impl PokemonSpecies {
    pub fn return_base_stats(&self) -> PokemonBaseData {
        match self {
            PokemonSpecies::Bulbasaur => BULBASAUR,
            PokemonSpecies::Charamander => CHARAMANDER,
            PokemonSpecies::Squirtle => SQUIRTLE,
            PokemonSpecies::Caterpie=>CATERPIE,
            PokemonSpecies::Weedle=>WEEDLE,
            PokemonSpecies::Pidgey => PIDGEY,
            PokemonSpecies::Rattata=>RATTATA,
            PokemonSpecies::Pikachu => PIKACHU,
        }
    }
}

const BULBASAUR: PokemonBaseData = PokemonBaseData {
    dex_num: 1,
    name: "Bulbasaur",
    base_hp: 45,
    base_attack: 49,
    base_defense: 49,
    base_speed: 46,
    base_special: 65,
    primary_type: PokeTypes::Grass,
    base_exp: 64,
    lvl1_moves: (Tackle, Growl),
};
const CHARAMANDER: PokemonBaseData = PokemonBaseData {
    dex_num: 4,
    name: "Charamander",
    base_hp: 39,
    base_attack: 52,
    base_defense: 43,
    base_speed: 65,
    base_special: 50,
    primary_type: PokeTypes::Fire,
    base_exp: 65,
    lvl1_moves: (Scratch, Growl),
};
const SQUIRTLE: PokemonBaseData = PokemonBaseData {
    dex_num: 7,
    name: "Squirtle",
    base_hp: 44,
    base_attack: 48,
    base_defense: 65,
    base_speed: 43,
    base_special: 50,
    primary_type: PokeTypes::Water,
    base_exp: 66,
    lvl1_moves: (Tackle, TailWhip),
};
const CATERPIE: PokemonBaseData = PokemonBaseData{
    dex_num: 10,
    name: "Caterpie",
    base_hp: 45,
    base_attack: 30,
    base_defense: 35,
    base_speed: 45,
    base_special: 20,
    primary_type: PokeTypes::Bug,
    base_exp: 0, //TODO
    lvl1_moves: (Tackle, Moves::StringShot),
};
const WEEDLE: PokemonBaseData = PokemonBaseData{
    dex_num: 13,
    name: "Weedle",
    base_hp: 40,
    base_attack: 35,
    base_defense: 30,
    base_speed: 50,
    base_special: 20,
    primary_type: PokeTypes::Bug,
    base_exp: 0, //TODO
    lvl1_moves: (Moves::PoisonSting, Moves::StringShot),
};
const PIDGEY: PokemonBaseData = PokemonBaseData {
    dex_num: 16,
    name: "Pidgey",
    base_hp: 40,
    base_attack: 45,
    base_defense: 40,
    base_speed: 56,
    base_special: 35,
    primary_type: PokeTypes::Normal,
    base_exp: 55,
    lvl1_moves: (Moves::Gust, Moves::Gust),
};
const RATTATA: PokemonBaseData = PokemonBaseData{
    dex_num: 19,
    name: "Rattata",
    base_hp: 30,
    base_attack: 56,
    base_defense: 35,
    base_speed: 72,
    base_special: 25,
    primary_type: PokeTypes::Normal,
    base_exp: 0, //TODO
    lvl1_moves: (Tackle, TailWhip),
};
const PIKACHU: PokemonBaseData = PokemonBaseData {
    dex_num: 25,
    name: "Pikachu",
    base_hp: 35,
    base_attack: 55,
    base_defense: 30,
    base_speed: 90,
    base_special: 50,
    primary_type: PokeTypes::Electric,
    base_exp: 82,
    lvl1_moves: (ThunderShock, Growl),
};