use crate::{Party, Pokemon};
use crate::mon_base_stats::PokemonSpecies::Pidgey;

struct EnemyTrainer{
    id: u16,
    name: &'static str,
    party: Party,
    reward: u16,
}
enum TrainerType{
    Lass,
    BugCatcher,
    Youngster,
}
fn enemy_trainers(){
    let enemy_trainers:Vec<EnemyTrainer> = vec![
        EnemyTrainer{
            id: 1,
            name: "Lass",
            party: Party {
                mon1: Some(Pokemon::new(Pidgey, 9)),
                mon2: Some(Pokemon::new(Pidgey, 9)),
                mon3: None,
                mon4: None,
                mon5: None,
                mon6: None,
            },
            reward: 135,
        },
        EnemyTrainer{
            id: 2,
            name: "Bug Catcher",
            party: Party {
                mon1: None,
                mon2: None,
                mon3: None,
                mon4: None,
                mon5: None,
                mon6: None,
            },
            reward: 0,
        }
    ];
}
/*
const LASS1: EnemyTrainer = EnemyTrainer{
    id: 1,
    name: "Lass",
    party: Party {
        mon1: Some(Pokemon::new(Pidgey, 9)),
        mon2: Some(Pokemon::new(Pidgey, 9)),
        mon3: None,
        mon4: None,
        mon5: None,
        mon6: None,
    },
    reward: 135,
};
const BUGCATCHER1: EnemyTrainer = EnemyTrainer{
    id: 2,
    name: "Bug Catcher",
    party: Party {
        mon1: None,
        mon2: None,
        mon3: None,
        mon4: None,
        mon5: None,
        mon6: None,
    },
    reward: 0,
};

 */