use crate::move_data::Moves;
use crate::move_data::Moves::{DefenseCurl, Growl, Gust, LeechSeed, SandAttack, Scratch, Screech, Tackle, TailWhip, ThunderShock};
use crate::PokemonSpecies::*;
use crate::{ExpCat, PokeTypes};
use crate::evolution::{EvolutionData};
use crate::ExpCat::{MediumFast, MediumSlow};
use crate::PokeTypes::*;

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PokemonBaseData {
    dex_num: u8,
    pub name: &'static str,
    pub base_hp: u16,
    pub base_attack: u16,
    pub base_defense: u16,
    pub base_speed: u16,
    pub base_special: u16,
    pub primary_type: PokeTypes,
    pub secondary_type: PokeTypes,
    pub base_exp: u16,
    pub lvl1_moves: (Moves, Moves),
    //pub evolution: Option<Evolution>,
    pub exp_cat: ExpCat,
    pub capture_rate: u8,
}

#[derive(Clone, PartialEq, Debug, Copy, Serialize, Deserialize)]
pub enum PokemonSpecies {
    Bulbasaur,
    Ivysaur,
    Venusaur,
    Charamander,
    Charmeleon,
    Charizard,
    Squirtle,
    Wartortle,
    Blastoise,
    Caterpie,
    Metapod,
    Weedle,
    Kakuna,
    Pidgey,
    Pidgeotto,
    Pidgeot,
    Rattata,
    Spearow,
    Ekans,
    Pikachu,
    NidoranF,
    Jigglypuff,
    Geodude,
    Graveler,
    Golem,
    Onix,
}

impl PokemonSpecies {
    pub fn return_base_stats(&self) -> PokemonBaseData {
        match self {
            Bulbasaur => BULBASAUR,
            Ivysaur=>IVYSAUR,
            Venusaur=>VENUSAUR,
            Charamander => CHARAMANDER,
            Charmeleon=>CHAMELEON,
            Charizard=>CHARIZARD,
            Squirtle => SQUIRTLE,
            Wartortle=>WARTORTLE,
            Blastoise=>BLASTOISE,
            Caterpie => CATERPIE,
            Metapod=>METAPOD,
            Weedle => WEEDLE,
            Kakuna=>KAKUNA,
            Pidgey => PIDGEY,
            Pidgeotto=>PIDGEOTTO,
            Pidgeot=>PIDGEOT,
            Rattata => RATTATA,
            Spearow=>SPEAROW,
            Ekans=>EKANS,
            Pikachu => PIKACHU,
            NidoranF=>NIDORANF,
            Jigglypuff=>JIGGLYPUFF,
            Geodude=>GEODUDE,
            Graveler=>GRAVELER,
            Golem=>GOLEM,
            Onix=>ONIX,
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
    primary_type: Grass,
    secondary_type: Poison,
    base_exp: 64,
    lvl1_moves: (Tackle, Growl), //Should be Tackle/Growl
    exp_cat: MediumSlow,
    capture_rate: 45,
};
const IVYSAUR: PokemonBaseData = PokemonBaseData{
    dex_num: 2,
    name: "Ivysaur",
    base_hp: 60,
    base_attack: 62,
    base_defense: 63,
    base_speed: 60,
    base_special: 80,
    primary_type: Grass,
    secondary_type: Poison,
    base_exp: 141,
    lvl1_moves: (Tackle, Growl),
    exp_cat: MediumSlow,
    capture_rate: 45,
};
const VENUSAUR: PokemonBaseData = PokemonBaseData{
    dex_num: 3,
    name: "Venusaur",
    base_hp: 80,
    base_attack: 82,
    base_defense: 83,
    base_speed: 80,
    base_special: 100,
    primary_type: Grass,
    secondary_type: Poison,
    base_exp: 208,
    lvl1_moves: (Tackle, Growl),
    exp_cat: MediumSlow,
    capture_rate: 45,
};
const CHARAMANDER: PokemonBaseData = PokemonBaseData {
    dex_num: 4,
    name: "Charamander",
    base_hp: 39,
    base_attack: 52,
    base_defense: 43,
    base_speed: 65,
    base_special: 50,
    primary_type: Fire,
    secondary_type: None,
    base_exp: 65,
    lvl1_moves: (Scratch, Growl),
    exp_cat: MediumSlow,
    capture_rate: 45,
};
const CHAMELEON: PokemonBaseData = PokemonBaseData{
    dex_num: 5,
    name: "Charmeleon",
    base_hp: 58,
    base_attack: 64,
    base_defense: 58,
    base_speed: 80,
    base_special: 65,
    primary_type: Fire,
    secondary_type: None,
    base_exp: 142,
    lvl1_moves: (Scratch,Growl),
    exp_cat: MediumSlow,
    capture_rate: 45,
};
const CHARIZARD: PokemonBaseData =PokemonBaseData{
    dex_num: 6,
    name: "Charizard",
    base_hp: 78,
    base_attack: 84,
    base_defense: 78,
    base_speed: 100,
    base_special: 85,
    primary_type: Fire,
    secondary_type: Flying,
    base_exp: 209,
    lvl1_moves: (Scratch, Growl),
    exp_cat: MediumSlow,
    capture_rate: 45,
};
const SQUIRTLE: PokemonBaseData = PokemonBaseData {
    dex_num: 7,
    name: "Squirtle",
    base_hp: 44,
    base_attack: 48,
    base_defense: 65,
    base_speed: 43,
    base_special: 50,
    primary_type: Water,
    secondary_type: None,
    base_exp: 66,
    lvl1_moves: (Tackle, TailWhip),
    exp_cat: MediumSlow,
    capture_rate: 45,
};
const WARTORTLE: PokemonBaseData = PokemonBaseData{
    dex_num: 8,
    name: "Wartortle",
    base_hp: 59,
    base_attack: 63,
    base_defense: 80,
    base_speed: 58,
    base_special: 65,
    primary_type: Water,
    secondary_type: None,
    base_exp: 143,
    lvl1_moves: (Tackle,TailWhip),
    exp_cat: MediumSlow,
    capture_rate: 45,
};
const BLASTOISE: PokemonBaseData =PokemonBaseData{
    dex_num: 9,
    name: "Blastoise",
    base_hp: 79,
    base_attack: 83,
    base_defense: 100,
    base_speed: 78,
    base_special: 85,
    primary_type: Water,
    secondary_type: None,
    base_exp: 210,
    lvl1_moves: (Tackle, TailWhip),
    exp_cat: MediumSlow,
    capture_rate: 45,
};
const CATERPIE: PokemonBaseData = PokemonBaseData {
    dex_num: 10,
    name: "Caterpie",
    base_hp: 45,
    base_attack: 30,
    base_defense: 35,
    base_speed: 45,
    base_special: 20,
    primary_type: Bug,
    secondary_type: None,
    base_exp: 53,
    lvl1_moves: (Tackle, Moves::StringShot),
    exp_cat: MediumFast,
    capture_rate: 255,
};
const METAPOD: PokemonBaseData = PokemonBaseData{
    dex_num: 11,
    name: "Metapod",
    base_hp: 50,
    base_attack: 20,
    base_defense: 55,
    base_speed: 30,
    base_special: 25,
    primary_type: Bug,
    secondary_type: None,
    base_exp: 72,
    lvl1_moves: (Tackle, Moves::Harden),
    exp_cat: MediumFast,
    capture_rate: 120,
};
const WEEDLE: PokemonBaseData = PokemonBaseData {
    dex_num: 13,
    name: "Weedle",
    base_hp: 40,
    base_attack: 35,
    base_defense: 30,
    base_speed: 50,
    base_special: 20,
    primary_type: Bug,
    secondary_type: Poison,
    base_exp: 52,
    lvl1_moves: (Moves::PoisonSting, Moves::StringShot),
    exp_cat: MediumFast,
    capture_rate: 255,
};
const KAKUNA: PokemonBaseData = PokemonBaseData{
    dex_num: 14,
    name: "Kakuna",
    base_hp: 45,
    base_attack: 25,
    base_defense: 50,
    base_speed: 35,
    base_special: 25,
    primary_type: Bug,
    secondary_type: Poison,
    base_exp: 71,
    lvl1_moves: (Moves::Tackle, Moves::Harden),
    exp_cat: MediumFast,
    capture_rate: 120,
};
const PIDGEY: PokemonBaseData = PokemonBaseData {
    dex_num: 16,
    name: "Pidgey",
    base_hp: 40,
    base_attack: 45,
    base_defense: 40,
    base_speed: 56,
    base_special: 35,
    primary_type: Normal,
    secondary_type: Flying,
    base_exp: 55,
    lvl1_moves: (Gust, Moves::Gust),
    exp_cat: MediumSlow,
    capture_rate: 255,
};
const PIDGEOTTO: PokemonBaseData =PokemonBaseData{
    dex_num: 17,
    name: "Pidgeotto",
    base_hp: 63,
    base_attack: 60,
    base_defense: 55,
    base_speed: 71,
    base_special: 50,
    primary_type: PokeTypes::Normal,
    secondary_type: Flying,
    base_exp: 113,
    lvl1_moves: (Gust, SandAttack),
    exp_cat: MediumSlow,
    capture_rate: 120,
};
const PIDGEOT: PokemonBaseData = PokemonBaseData{
    dex_num: 18,
    name: "Pidgeot",
    base_hp: 83,
    base_attack: 80,
    base_defense: 75,
    base_speed: 91,
    base_special: 70,
    primary_type: PokeTypes::Normal,
    secondary_type: Flying,
    base_exp: 172,
    lvl1_moves: (Gust, SandAttack),
    exp_cat: MediumSlow,
    capture_rate: 45,
};
const RATTATA: PokemonBaseData = PokemonBaseData {
    dex_num: 19,
    name: "Rattata",
    base_hp: 30,
    base_attack: 56,
    base_defense: 35,
    base_speed: 72,
    base_special: 25,
    primary_type: PokeTypes::Normal,
    secondary_type: PokeTypes::None,
    base_exp: 57,
    lvl1_moves: (Tackle, TailWhip),
    exp_cat: ExpCat::MediumFast,
    capture_rate: 255,
};
const SPEAROW: PokemonBaseData = PokemonBaseData{
    dex_num: 21,
    name: "Spearow",
    base_hp: 40,
    base_attack: 60,
    base_defense: 30,
    base_speed: 70,
    base_special: 31,
    primary_type: PokeTypes::Normal,
    secondary_type: PokeTypes::Flying,
    base_exp: 58,
    lvl1_moves: (Moves::Peck, Growl),
    exp_cat: MediumFast,
    capture_rate: 255,
};
const EKANS: PokemonBaseData = PokemonBaseData{
    dex_num: 23,
    name: "Ekans",
    base_hp: 35,
    base_attack: 60,
    base_defense: 44,
    base_speed: 55,
    base_special: 40,
    primary_type: Poison,
    secondary_type: None,
    base_exp: 0,
    lvl1_moves: (Moves::Wrap, Moves::Leer),
    exp_cat: MediumFast,
    capture_rate: 255,
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
    secondary_type: PokeTypes::None,
    base_exp: 82,
    lvl1_moves: (ThunderShock, Growl),
    exp_cat: MediumFast,
    capture_rate: 190,
};
const NIDORANF: PokemonBaseData = PokemonBaseData{
    dex_num: 29,
    name: "Nidoran♀",
    base_hp: 55,
    base_attack: 47,
    base_defense: 52,
    base_speed: 41,
    base_special: 40,
    primary_type: Poison,
    secondary_type: None,
    base_exp: 59,
    lvl1_moves: (Tackle, Growl),
    exp_cat: MediumSlow,
    capture_rate: 0,
};
const JIGGLYPUFF: PokemonBaseData = PokemonBaseData{
    dex_num: 39,
    name: "Jigglypuff",
    base_hp: 115,
    base_attack: 45,
    base_defense: 20,
    base_speed: 20,
    base_special: 25,
    primary_type: Normal,
    secondary_type: None,
    base_exp: 76,
    lvl1_moves: (Moves::Sing, Moves::Sing),
    exp_cat: ExpCat::Fast,
    capture_rate: 173,
};
const GEODUDE: PokemonBaseData=PokemonBaseData{
    dex_num: 74,
    name: "Geodude",
    base_hp: 40,
    base_attack: 80,
    base_defense: 100,
    base_speed: 20,
    base_special: 30,
    primary_type: Rock,
    secondary_type: Ground,
    base_exp: 86,
    lvl1_moves: (Tackle,Tackle),
    exp_cat: MediumSlow,
    capture_rate: 255,
};
const GRAVELER: PokemonBaseData = PokemonBaseData{
    dex_num: 75,
    name: "Graveler",
    base_hp: 55,
    base_attack: 95,
    base_defense: 115,
    base_speed: 35,
    base_special: 45,
    primary_type: Rock,
    secondary_type: Ground,
    base_exp: 134,
    lvl1_moves: (Tackle, DefenseCurl),
    exp_cat: MediumSlow,
    capture_rate: 120,
};
const GOLEM: PokemonBaseData=PokemonBaseData{
    dex_num: 76,
    name: "Golem",
    base_hp: 80,
    base_attack: 110,
    base_defense: 130,
    base_speed: 45,
    base_special: 55,
    primary_type: Rock,
    secondary_type: Ground,
    base_exp: 177,
    lvl1_moves: (Tackle, DefenseCurl),
    exp_cat: MediumSlow,
    capture_rate: 45,
};
const ONIX: PokemonBaseData =PokemonBaseData{
    dex_num: 95,
    name: "Onix",
    base_hp: 35,
    base_attack: 45,
    base_defense: 160,
    base_speed: 70,
    base_special: 30,
    primary_type: Rock,
    secondary_type: Ground,
    base_exp: 108,
    lvl1_moves: (Tackle, Screech),
    exp_cat: MediumFast,
    capture_rate: 45,
};