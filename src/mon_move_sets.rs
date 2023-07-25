use crate::mon_base_stats::PokemonSpecies;
use std::collections::HashMap;

use crate::move_data::Moves;
use crate::move_data::Moves::{
    Agility, Bite, Bubble, Ember, FireSpin, Flamethrower, FocusEnergy, Growth, HydroPump,
    HyperFang, LeechSeed, Leer, MirrorMove, PoisonPowder, QuickAttack, Rage, RazorLeaf, SandAttack,
    SkullBash, Slash, SleepPowder, SolarBeam, SuperFang, Swift, Thunder, ThunderWave, VineWhip,
    WaterGun, Whirlwind, WingAttack, Withdraw,
};

pub struct LearnableMoves {
    pub species: PokemonSpecies,
    pub level_up_moves: &'static [(u16, Moves)],
}

pub const LEARNABLEMOVES: &[LearnableMoves] = &[
    LearnableMoves {
        species: PokemonSpecies::Bulbasaur,
        level_up_moves: &[
            (7, LeechSeed),
            (13, VineWhip),
            (20, PoisonPowder),
            (27, RazorLeaf),
            (34, Growth),
            (41, SleepPowder),
            (48, SolarBeam),
        ],
    },
    LearnableMoves {
        species: PokemonSpecies::Charamander,
        level_up_moves: &[
            (9, Ember),
            (15, Leer),
            (22, Rage),
            (30, Slash),
            (38, Flamethrower),
            (46, FireSpin),
        ],
    },
    LearnableMoves {
        species: PokemonSpecies::Squirtle,
        level_up_moves: &[
            (8, Bubble),
            (15, WaterGun),
            (22, Bite),
            (28, Withdraw),
            (35, SkullBash),
            (42, HydroPump),
        ],
    },
    LearnableMoves {
        species: PokemonSpecies::Pidgey,
        level_up_moves: &[
            (5, SandAttack),
            (12, QuickAttack),
            (19, Whirlwind),
            (28, WingAttack),
            (36, Agility),
            (44, MirrorMove),
        ],
    },
    LearnableMoves {
        species: PokemonSpecies::Rattata,
        level_up_moves: &[
            (7, QuickAttack),
            (14, HyperFang),
            (23, FocusEnergy),
            (34, SuperFang),
        ],
    },
    LearnableMoves {
        species: PokemonSpecies::Pikachu,
        level_up_moves: &[
            (9, ThunderWave),
            (16, QuickAttack),
            (26, Swift),
            (33, Agility),
            (43, Thunder),
        ],
    },
    /*
    LearnableMoves{
        species: PokemonSpecies::Charamander,
        level_up_moves: &[],
    }
     */
    /*
    tm_moves: HashMap(
        (3, SwordsDance),
        (6, Toxic),
        (8, BodySlam),
        (9, TakeDown),
        (10, DoubleEdge),
        (20, Rage),
        (21, MegaDrain),
        (22, SolarBeam),
        (31, Mimic),
        (32, DoubleTeam),
        (33, Reflect),
        (34, Bide),
        (44, Rest),
        (50, Substitude),
        (101, Cut),
    ),
    */
];
