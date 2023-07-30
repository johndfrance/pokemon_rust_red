use crate::mon_base_stats::PokemonSpecies;
use crate::PokemonSpecies::*;

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
    pokemon: PokemonSpecies::Bulbasaur,
    next_stage: Some(Venusaur),
    trigger: EvolutionTriggers::ByLevel(16),
};
const IVYSAUR: EvolutionData = EvolutionData{
    pokemon: Ivysaur,
    next_stage: Some(Venusaur),
    trigger: EvolutionTriggers::ByLevel(32),
};
const VENUSAUR: EvolutionData=EvolutionData{
    pokemon: Venusaur,
    next_stage: None,
    trigger: EvolutionTriggers::ByLevel(100),
};
const CHARMANDER: EvolutionData=EvolutionData{
    pokemon: Charamander,
    next_stage: Some(Charmeleon),
    trigger: EvolutionTriggers::ByLevel(16),
};
const CHARMELEON: EvolutionData=EvolutionData{
    pokemon: Charmeleon,
    next_stage: Some(Charizard),
    trigger: EvolutionTriggers::ByLevel(36),
};
const CHARIZARD: EvolutionData=EvolutionData{
    pokemon: Charizard,
    next_stage: None,
    trigger: EvolutionTriggers::ByLevel(100),
};
const SQUIRTLE: EvolutionData=EvolutionData{
    pokemon: Squirtle,
    next_stage: Some(Wartortle),
    trigger: EvolutionTriggers::ByLevel(16),
};
const WARTORTLE: EvolutionData =EvolutionData{
    pokemon: Wartortle,
    next_stage: Some(Blastoise),
    trigger: EvolutionTriggers::ByLevel(36),
};
const BLASTOISE: EvolutionData=EvolutionData{
    pokemon: Blastoise,
    next_stage: None,
    trigger: EvolutionTriggers::ByLevel(100),
};
pub const CATERPIE: EvolutionData = EvolutionData{
    pokemon: Caterpie,
    next_stage: Some(Metapod),
    trigger: EvolutionTriggers::ByLevel(7),
};