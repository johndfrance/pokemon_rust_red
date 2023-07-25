use std::string::ToString;
use crate::mon_base_stats::PokemonSpecies::{Caterpie, Ekans, Jigglypuff, Kakuna, Metapod, NidoranF, Pidgey, Rattata, Spearow, Weedle};
use crate::{Party, Pokemon, Trainer};
use crate::mon_base_stats::PokemonSpecies;


// The reason for doing this is that the size of the data in a const needs to be known at compile time,
// so we cannot call Pokemon::new() within the const. The way around this is to just list the pokemon
// and level paring and then have a function that renders the trainers elsewhere. This has the added
// benefit of only calculating the stats of pokemon when needed. Had I been allowed to impliment this
// the way I first wanted it would have calculated the stats of all pokemon for all trainers whenever
// the game is booted up.
pub struct TrainerTemplate{
    id: u16,
    name: &'static str,
    party: [Option<(PokemonSpecies, u16)>; 6],
    reward: u16,
}
struct EnemyTrainer2{
    name: String,
    party: Vec<Pokemon>,
    reward: u16,
}
impl EnemyTrainer2{
    fn get(id: u16)-> EnemyTrainer2{
        let trainers: Vec<TrainerTemplate> = vec![LASS1, BUGCATCHER1, YOUNGSTER1];
        let found_trainer = trainers.iter().find(|trainer| trainer.id == id).unwrap();
        let mut trainer_party: Vec<Pokemon> = vec![];
        for mon in found_trainer.party{
           let (new_mon, level) = mon.unwrap_or((Pidgey, 1)); //TODO This is sloppy
            // There must be a method that lets us do one thing is unwrap is valid, and not if unwrap is None.
            if !(new_mon == Pidgey && level == 1) {
                trainer_party.push(Pokemon::new(new_mon, level))
            }
        }

        EnemyTrainer2{
            name: found_trainer.name.clone().parse().unwrap(),
            party: trainer_party,
            reward: found_trainer.reward.clone(),
        }
    }
}


//VIRIDIAN FOREST TRAINERS

//PEWTER CITY GYM TRAINERS

// ROUTE 3 TRAINERS
pub const LASS1: TrainerTemplate = TrainerTemplate{
    id: 1,
    name: "Lass Janice",
    party: [Some((Pidgey, 9)), Some((Pidgey, 9)), None, None, None, None],
    reward: 135,
};
pub const BUGCATCHER1: TrainerTemplate = TrainerTemplate{
  id: 2,
    name: "Bug Catcher Colton",
    party: [Some((Caterpie, 10)), Some((Weedle, 10)), Some((Caterpie, 10)), None, None, None],
    reward: 100,
};
pub const YOUNGSTER1: TrainerTemplate = TrainerTemplate{
    id: 3,
    name: "Youngster Ben",
    party: [Some((Rattata, 11)), Some((Ekans, 11)), None, None, None, None,],
    reward: 165,
};
pub const BUGCATCHER2: TrainerTemplate = TrainerTemplate{
    id: 4,
    name: "Bug Catcher Greg",
    party: [Some((Weedle, 9)), Some((Kakuna, 9)), Some((Caterpie, 9)), Some((Metapod, 9)), None,None],
    reward: 90,
};
pub const YOUNGSTER2: TrainerTemplate = TrainerTemplate{
    id: 5,
    name: "Youngster Calvin",
    party: [Some((Spearow, 14)), None, None, None, None, None],
    reward: 210,
};
pub const LASS2: TrainerTemplate = TrainerTemplate{
    id: 6,
    name: "Lass Sally",
    party: [Some((Rattata, 10)), Some((NidoranF, 10)), None, None, None, None],
    reward: 150,
};
pub const BUGCATCHER3: TrainerTemplate = TrainerTemplate{
    id: 7,
    name: "Bug Catcher James",
    party: [Some((Caterpie, 10)), Some((Metapod, 10)), None, None, None, None],
    reward: 110,
};
pub const LASS3: TrainerTemplate = TrainerTemplate{
    id: 8,
    name: "Lass Robin",
    party: [Some((Jigglypuff, 14)), None, None, None, None, None],
    reward: 150,
};







struct EnemyTrainer {
    id: u16,
    name: &'static str,
    party: Party,
    reward: u16,
}
