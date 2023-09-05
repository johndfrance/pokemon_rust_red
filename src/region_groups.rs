/*
Title: region_groups.rs

Description: Each region of Kanto has different wild pokemon. This file provides a grouping of what
wild pokemon are in each region.
 */
use rand::Rng;
use crate::game::{PalletTownLocations, PewterCityLocations, Regions};
use crate::mon_base_stats::PokemonSpecies;
use crate::mon_base_stats::PokemonSpecies::{Caterpie, Jigglypuff, Kakuna, Metapod, Pidgey, Pikachu, Rattata, Spearow, Weedle};
use crate::game::Regions::*;
use crate::game::PalletTownLocations::*;
use crate::game::ViridianCityLocations::*;
use crate::Pokemon;
use serde::{Serialize, Deserialize};
use crate::game::PewterCityLocations::*;
use crate::game::Route3Loc::PewterConnection;


pub struct WildPokemon{
    pub species:PokemonSpecies,
   pub level: u16,
}
pub fn get_wild_encounter(location: Regions)->Option<Pokemon>{
    let random_number = rand::thread_rng().gen_range(0..=1);
    if random_number == 0 {
        let encounter_data = WILD_ENCOUNTER_MAP
            .iter()
            .find(|(loc, _)| loc == &location)
            .map(|(_, encounter_possibilities)| *encounter_possibilities);
        let len_encounter = encounter_data.unwrap().len();
        let mut rng = rand::thread_rng();
        let pick = rng.gen_range(0..(len_encounter));
        let unwraped_data = encounter_data.unwrap();
        let chosen_mon = &unwraped_data[pick];
        let chosen_species = chosen_mon.species;
        let chosen_level = chosen_mon.level;
        let wild_mon = Pokemon::new(chosen_species, chosen_level);
        return Some(wild_mon);
    }
    None
}

const WILD_ENCOUNTER_MAP: &[(Regions, &[WildPokemon])]=&[
    (PalletTown(Route1),&[
         WildPokemon { species: Pidgey, level: 2},
         WildPokemon { species: Pidgey, level: 3},
         WildPokemon { species: Pidgey, level: 4},
         WildPokemon { species: Pidgey, level: 5},
        WildPokemon{ species: Rattata, level: 2 },
        WildPokemon{ species: Rattata, level: 3 },
        WildPokemon{ species: Rattata, level: 4 },
    ]),
    (ViridianCity(Route2), &[
        WildPokemon { species: Pidgey, level: 3},
        WildPokemon { species: Pidgey, level: 4},
        WildPokemon { species: Pidgey, level: 5},
        WildPokemon{ species: Rattata, level: 3},
        WildPokemon{ species: Rattata, level: 4},
        WildPokemon{ species: Weedle, level: 3},
        WildPokemon{ species: Weedle, level: 4},
        WildPokemon{ species: Weedle, level: 5},
        WildPokemon{ species: Caterpie, level: 3},
        WildPokemon{ species: Caterpie, level: 4},
        WildPokemon{ species: Caterpie, level: 5},
    ]),
    (ViridianCity(ViridianForest), &[
        WildPokemon{ species: Weedle, level: 3},
        WildPokemon{ species: Weedle, level: 4},
        WildPokemon{ species: Weedle, level: 5},
        WildPokemon{ species: Caterpie, level: 3},
        WildPokemon{ species: Caterpie, level: 4},
        WildPokemon{ species: Caterpie, level: 5},
        WildPokemon{species:Kakuna, level:5},
        WildPokemon{species:Kakuna, level:6},
        WildPokemon{species:Metapod, level:5},
        WildPokemon{species:Metapod, level:6},
        WildPokemon{ species: Weedle, level: 3},
        WildPokemon{ species: Weedle, level: 4},
        WildPokemon{ species: Weedle, level: 5},
        WildPokemon{ species: Caterpie, level: 3},
        WildPokemon{ species: Caterpie, level: 4},
        WildPokemon{ species: Caterpie, level: 5},
        WildPokemon{species:Kakuna, level:5},
        WildPokemon{species:Kakuna, level:6},
        WildPokemon{species:Metapod, level:5},
        WildPokemon{species:Metapod, level:6},
        WildPokemon{species:Pikachu, level: 6},
        WildPokemon{species:Pikachu, level: 5},
    ]),
    (Route3(PewterConnection), &[
        WildPokemon{species: Pidgey, level: 10},
        WildPokemon{species: Pidgey, level: 7},
        WildPokemon{species: Pidgey, level: 8},
        WildPokemon{species: Pidgey, level: 9},
        WildPokemon{species: Spearow, level: 11},
        WildPokemon{species: Spearow, level: 9},
        WildPokemon{species: Spearow, level: 7},
        WildPokemon{species: Spearow, level: 8},
        WildPokemon{species: Pidgey, level: 10},
        WildPokemon{species: Pidgey, level: 7},
        WildPokemon{species: Pidgey, level: 8},
        WildPokemon{species: Pidgey, level: 9},
        WildPokemon{species: Spearow, level: 11},
        WildPokemon{species: Spearow, level: 6},
        WildPokemon{species: Spearow, level: 7},
        WildPokemon{species: Spearow, level: 8},
        WildPokemon{species: Jigglypuff, level: 6},
        WildPokemon{species: Jigglypuff, level: 5},
        WildPokemon{species: Jigglypuff, level: 7},
    ]),

];