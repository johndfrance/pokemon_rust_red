use crate::mon_base_stats::PokemonSpecies;
use crate::PokemonSpecies::*;
use crate::EvolutionTriggers::*;

impl PokemonSpecies {
    pub fn return_evolution(&self)->EvolutionData{
        match self {
            Bulbasaur=>BULBASAUR,
            Ivysaur=>IVYSAUR,
            Venusaur=>VENUSAUR,
            Charamander=>CHARMANDER,
            Charmeleon=>CHARMELEON,
            Charizard=>CHARIZARD,
            Squirtle=>SQUIRTLE,
            Wartortle=>WARTORTLE,
            Blastoise=>BLASTOISE,
            Caterpie=>CATERPIE,
            Metapod=>METAPOD,
            Butterfly=>BUTTERFREE,
            Weedle=>WEEDLE,
            Kakuna=>KAKUNA,
            Beedrill=>BEEDRILL,
            Pidgey=>PIDGEY,
            Pidgeotto=>PIDGEOTTO,
            Pidgeot=>PIDGEOT,
            Rattata=>RATTATA,
            Raticate=>RATICATE,
            Spearow=>SPEAROW,
            Fearow=>FEAROW,
            Pikachu=>PIKACHU,
            Jigglypuff=>JIGGLYPUFF,
            Poliwag=>POLIWAG,
            Poliwhirl=>POLIWHIRL,
            Onix=>ONIX,
            Lapras=>LAPRAS,
            _=>todo!()
        }
    }
}
#[derive(PartialEq)]
pub enum EvolutionTriggers{
    ByLevel(u16),
    ByItem,
}
pub struct EvolutionData{
    pub pokemon: PokemonSpecies,
    pub next_stage: Option<PokemonSpecies>,
    pub trigger: EvolutionTriggers,
}

const BULBASAUR: EvolutionData = EvolutionData{
    pokemon: Bulbasaur,
    next_stage: Some(Ivysaur),
    trigger: ByLevel(16),
};
const IVYSAUR: EvolutionData = EvolutionData{
    pokemon: Ivysaur,
    next_stage: Some(Venusaur),
    trigger: ByLevel(32),
};
const VENUSAUR: EvolutionData=EvolutionData{
    pokemon: Venusaur,
    next_stage: None,
    trigger: ByLevel(100),
};
const CHARMANDER: EvolutionData=EvolutionData{
    pokemon: Charamander,
    next_stage: Some(Charmeleon),
    trigger: ByLevel(16),
};
const CHARMELEON: EvolutionData=EvolutionData{
    pokemon: Charmeleon,
    next_stage: Some(Charizard),
    trigger: ByLevel(36),
};
const CHARIZARD: EvolutionData=EvolutionData{
    pokemon: Charizard,
    next_stage: None,
    trigger: ByLevel(100),
};
const SQUIRTLE: EvolutionData=EvolutionData{
    pokemon: Squirtle,
    next_stage: Some(Wartortle),
    trigger: ByLevel(16),
};
const WARTORTLE: EvolutionData =EvolutionData{
    pokemon: Wartortle,
    next_stage: Some(Blastoise),
    trigger: ByLevel(36),
};
const BLASTOISE: EvolutionData=EvolutionData{
    pokemon: Blastoise,
    next_stage: None,
    trigger: ByLevel(100),
};
pub const CATERPIE: EvolutionData = EvolutionData{
    pokemon: Caterpie,
    next_stage: Some(Metapod),
    trigger: ByLevel(7),
};
const METAPOD: EvolutionData = EvolutionData{
    pokemon: Metapod,
    next_stage: Some(Butterfly),
    trigger: ByLevel(10),
};
const BUTTERFREE: EvolutionData = EvolutionData{
    pokemon: Butterfly,
    next_stage: None,
    trigger: ByLevel(100),
};
const WEEDLE: EvolutionData=EvolutionData{
    pokemon: Weedle,
    next_stage: Some(Kakuna),
    trigger: ByLevel(7),
};
const KAKUNA: EvolutionData = EvolutionData{
    pokemon: Kakuna,
    next_stage: Some(Beedrill),
    trigger: ByLevel(10),
};
const BEEDRILL: EvolutionData= EvolutionData{
    pokemon: Beedrill,
    next_stage: None,
    trigger: ByLevel(100),
};
const PIDGEY: EvolutionData =EvolutionData{
    pokemon: Pidgey,
    next_stage: Some(Pidgeotto),
    trigger: ByLevel(18),
};
const PIDGEOTTO: EvolutionData =EvolutionData{
    pokemon: Pidgeotto,
    next_stage: Some(Pidgeot),
    trigger: ByLevel(36),
};
const PIDGEOT:EvolutionData=EvolutionData{
    pokemon: Pidgeot,
    next_stage: None,
    trigger: ByLevel(100),
};
const RATTATA: EvolutionData=EvolutionData{
    pokemon: Rattata,
    next_stage: Some(Raticate),
    trigger: ByLevel(20),
};
const RATICATE: EvolutionData=EvolutionData{
    pokemon: Raticate,
    next_stage: None,
    trigger:ByLevel(100),
};
const SPEAROW: EvolutionData = EvolutionData{
  pokemon:Spearow,
    next_stage: Some(Fearow),
    trigger:ByLevel(20),
};
const FEAROW: EvolutionData = EvolutionData{
    pokemon: Fearow,
    next_stage: None,
    trigger: ByLevel(100),
};
const PIKACHU: EvolutionData=EvolutionData{
    pokemon: Pikachu,
    next_stage: Some(Richu),
    trigger: EvolutionTriggers::ByItem,
};
const JIGGLYPUFF: EvolutionData=EvolutionData{
    pokemon: Jigglypuff,
    next_stage:Some(Wigglytuff),
    trigger:ByItem
};
const POLIWAG: EvolutionData=EvolutionData{
    pokemon: Poliwag,
    next_stage: Some(Poliwhirl),
    trigger: ByLevel(25),
};
const POLIWHIRL: EvolutionData = EvolutionData{
    pokemon: Poliwhirl,
    next_stage: None,
    trigger: ByItem,
};
const ONIX: EvolutionData=EvolutionData{
    pokemon: Onix,
    next_stage: None,
    trigger: ByLevel(100),
};
const LAPRAS: EvolutionData = EvolutionData{
    pokemon: Lapras,
    next_stage: None,
    trigger: ByItem,
};