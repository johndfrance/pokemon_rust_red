use std::string::ToString;
use crate::mon_base_stats::PokemonSpecies::{Beedrill, Bulbasaur, Butterfly, Caterpie, Charamander, Charmeleon, Diglett, Ekans, Geodude, Ivysaur, Jigglypuff, Kakuna, Metapod, NidoranF, Nidorina, Onix, Pidgey, Rattata, Sandshrew, Spearow, Squirtle, Wartortle, Weedle, Zubat};
use crate::{Party, PartyOperations, Pokemon};
use crate::mon_base_stats::PokemonSpecies;
use crate::Status::Healthy;


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
pub struct Trainer{
    pub id: u16,
    pub name: &'static str,
    pub poke_team: Vec<Pokemon>,
    pub reward: u16,

}
impl Trainer{
    pub fn get(id: u16)-> Trainer{
        let trainers: Vec<TrainerTemplate> = vec![RIVAL1_1,RIVAL1_2, RIVAL2_1,RIVAL2_2, RIVAL3_1,RIVAL3_2, BUGCATCHER1, YOUNGSTER1, BUGCATCHER4, BUGCATCHER5, BUGCATCHER6, BUGCATCHER7,JRTRAINER1, BROCK, LASS1, LASS2, LASS3, BUGCATCHER2, BUGCATCHER3, YOUNGSTER2];

        let found_trainer = trainers
            .iter()
            .find(|trainer| trainer.id == id)
            .unwrap();

        let mut trainer_party: Vec<Pokemon> = vec![];

        for mon in found_trainer.party{
            if mon != None{
                let (new_mon, level) = mon.unwrap();
                trainer_party.push(Pokemon::new(new_mon, level));
            }
        }
        Trainer{
            id,
            name: found_trainer.name.clone(),
            poke_team: trainer_party,
            reward: found_trainer.reward.clone(),
        }
    }

    fn new(team: Vec<Pokemon>)->Trainer{
        todo!()
    }
    fn new_rand()->Trainer{
        todo!()
    }
}
impl PartyOperations for Trainer {
    fn check_all_fainted(&self) -> bool {
        for mon in &self.poke_team {
            if mon.status == Healthy {
                return true;
            }
        }
        false
    }
    fn return_first_healthy(&self) -> Result<usize, &str> {
        let mut counter: usize = 0;
        for mon in &self.poke_team {
            if mon.status == Healthy {
                return Ok(counter);
            }
            counter += 1;
        }
        Err("No Healthy Pokemon Found")
    }
}
//RIVAL VERSIONS
const RIVAL1_1: TrainerTemplate=TrainerTemplate{
    id: 1001,
    name: "Blue",
    party: [Some((Bulbasaur, 5)), None, None, None, None, None],
    reward: 150,
};
const RIVAL1_2: TrainerTemplate=TrainerTemplate{
    id: 1002,
    name: "Blue",
    party: [Some((Nidorina, 13)),  Some((Spearow, 12)), Some((Ivysaur, 16)), None, None, None],
    reward: 500,
};
const RIVAL2_1: TrainerTemplate=TrainerTemplate{
    id: 2001,
    name: "Blue",
    party: [Some((Charamander, 5)), None, None, None, None, None],
    reward: 150,
};
const RIVAL2_2: TrainerTemplate=TrainerTemplate{
    id: 2002,
    name: "Blue",
    party: [Some((Nidorina, 13)),  Some((Spearow, 12)), Some((Charmeleon, 16)), None, None, None],
    reward: 500,
};
const RIVAL3_1: TrainerTemplate=TrainerTemplate{
    id: 3001,
    name: "Blue",
    party: [Some((Squirtle, 5)), None, None, None, None, None],
    reward: 150,
};
const RIVAL3_2: TrainerTemplate=TrainerTemplate{
    id: 3002,
    name: "Blue",
    party: [Some((Nidorina, 13)),  Some((Spearow, 12)), Some((Wartortle, 16)), None, None, None],
    reward: 500,
};


//VIRIDIAN FOREST TRAINERS
pub const BUGCATCHER4: TrainerTemplate = TrainerTemplate{
    id: 8,
    name: "Bug Catcher Rick",
    party: [Some((Caterpie, 6)), Some((Weedle, 6)), None, None, None, None],
    reward: 60,
};
pub const BUGCATCHER5: TrainerTemplate = TrainerTemplate{
    id: 9,
    name: "Bug Catcher Doug",
    party: [Some((Weedle, 7)), Some((Kakuna, 8)), Some((Weedle, 7)), None, None, None],
    reward: 70,
};
pub const BUGCATCHER6: TrainerTemplate = TrainerTemplate{
    id: 10,
    name: "Bug Catcher Sammy",
    party: [Some((Weedle, 9)), None, None, None, None, None],
    reward: 90,
};
pub const BUGCATCHER7: TrainerTemplate = TrainerTemplate{
    id: 12,
    name: "Bug Catcher Dan",
    party: [Some((Caterpie, 7)), Some((Metapod, 8)), Some((Caterpie, 7)), None, None, None],
    reward: 70,
};



//PEWTER CITY GYM TRAINERS
pub const JRTRAINER1: TrainerTemplate =TrainerTemplate{
    id: 11,
    name: "Jr. Trainer Jerry",
party: [Some((Diglett, 11)), Some((Sandshrew, 11)),None,None,None,None],
    reward: 220,
};
pub const BROCK: TrainerTemplate = TrainerTemplate{
    id: 501,
    name: "Gym Leader Brock",
    party: [Some((Geodude, 12)),Some((Onix, 14)), None, None, None, None ],
    reward: 1386,
};
//CERULEAN CITY GYM TRAINERS


// ROUTE 3 TRAINERS
pub const LASS1: TrainerTemplate = TrainerTemplate{
    id: 1,
    name: "Lass Janice",
    party: [Some((Pidgey, 9)), Some((Pidgey, 11)), None, None, None, None],
    reward: 135,
};
pub const BUGCATCHER1: TrainerTemplate = TrainerTemplate{
  id: 2,
    name: "Bug Catcher Colton",
    party: [Some((Caterpie, 10)), Some((Metapod, 10)), Some((Butterfly, 10)), None, None, None],
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
    party: [Some((Weedle, 9)), Some((Kakuna, 10)), Some((Caterpie, 9)), Some((Metapod, 10)), None,None],
    reward: 90,
};
pub const YOUNGSTER2: TrainerTemplate = TrainerTemplate{
    id: 5,
    name: "Youngster Calvin",
    party: [Some((Spearow, 15)), None, None, None, None, None],
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
    party: [Some((Butterfly, 12)), Some((Beedrill, 12)), None, None, None, None],
    reward: 110,
};
pub const LASS3: TrainerTemplate = TrainerTemplate{
    id: 8,
    name: "Lass Robin",
    party: [Some((Jigglypuff, 15)), None, None, None, None, None],
    reward: 150,
};

