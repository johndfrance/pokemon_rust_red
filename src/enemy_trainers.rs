use crate::mon_base_stats::PokemonSpecies::Pidgey;
use crate::{Party, Pokemon};

struct EnemyTrainer {
    id: u16,
    name: &'static str,
    party: Party,
    reward: u16,
}
enum TrainerType {
    Lass,
    BugCatcher,
    Youngster,
}
pub fn enemy_trainers(trainer_id: u32){
    let enemy_trainers: Vec<EnemyTrainer> = vec![
        EnemyTrainer {
            id: 1,
            name: "Lass",
            party: Party {
                mon: [
                    Some(Pokemon::new(Pidgey, 9)),
                    Some(Pokemon::new(Pidgey, 9)),
                    None,
                    None,
                    None,
                    None,
                ],
            },
            reward: 135,
        },
        EnemyTrainer {
            id: 2,
            name: "Bug Catcher",
            party: Party {
                mon: [None, None, None, None, None, None],
            },
            reward: 0,
        },
    ];
}
