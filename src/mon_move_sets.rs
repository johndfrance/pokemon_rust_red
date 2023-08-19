use crate::mon_base_stats::PokemonSpecies;
use std::collections::HashMap;


use crate::move_data::Moves;
use crate::move_data::Moves::{Acid, Agility, Bind, Bite, BodySlam, Bubble, Confusion, DefenseCurl, Dig, Disable, DoubleEdge, DoubleKick, DoubleSlap, DrillPeck, Earthquake, Ember, Explosion, FireSpin, Flamethrower, FocusEnergy, FuryAttack, FurySwipes, Glare, Growl, Growth, Harden, HydroPump, HyperFang, LeechSeed, Leer, MirrorMove, PinMissile, PoisonPowder, PoisonSting, Pound, Psybeam, QuickAttack, Rage, RazorLeaf, RestMove, RockThrow, SandAttack, Scratch, Screech, SelfDestruct, SkullBash, Slam, Slash, SleepPowder, SolarBeam, StunSpore, SuperFang, Supersonic, Swift, TailWhip, Thunder, ThunderWave, Twineedle, VineWhip, WaterGun, Whirlwind, WingAttack, Withdraw};
use crate::PokemonSpecies::*;

pub struct LearnableMoves {
    pub species: PokemonSpecies,
    pub level_up_moves: &'static [(u16, Moves)],
}

pub const LEARNABLEMOVES: &[LearnableMoves] = &[
    LearnableMoves {
        species: Bulbasaur,
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
    LearnableMoves{
        species: Ivysaur,
        level_up_moves: &[
            (7, LeechSeed),
            (13, VineWhip),
            (22, PoisonPowder),
            (30, RazorLeaf),
            (38, Growth),
            (46, SleepPowder),
            (54, SolarBeam),
        ],
    },
    LearnableMoves{
        species: Venusaur,
        level_up_moves: &[
            (7, LeechSeed),
            (13, VineWhip),
            (22, PoisonPowder),
            (30, RazorLeaf),
            (43, Growth),
            (55, SleepPowder),
            (65, SolarBeam),
        ],
    },
    LearnableMoves {
        species: Charamander,
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
        species: Charmeleon,
        level_up_moves: &[
            (9, Ember),
            (15, Leer),
            (24, Rage),
            (33, Slash),
            (42, Flamethrower),
            (56, FireSpin),
        ],
    },
    LearnableMoves {
        species: Charizard,
        level_up_moves: &[
            (9, Ember),
            (15, Leer),
            (24, Rage),
            (36, Slash),
            (46, Flamethrower),
            (55, FireSpin),
        ],
    },
    LearnableMoves {
        species: Squirtle,
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
        species: Wartortle,
        level_up_moves: &[
            (8, Bubble),
            (15, WaterGun),
            (24, Bite),
            (31, Withdraw),
            (39, SkullBash),
            (47, HydroPump),
        ],
    },
    LearnableMoves {
        species: Blastoise,
        level_up_moves: &[
            (8, Bubble),
            (15, WaterGun),
            (24, Bite),
            (31, Withdraw),
            (42, SkullBash),
            (52, HydroPump),
        ],
    },
    LearnableMoves{
        species: Caterpie,
        level_up_moves: &[],
    },
    LearnableMoves{
        species: Metapod,
        level_up_moves: &[
            (7, Harden),
        ]
    },
    LearnableMoves{
        species: Butterfly,
        level_up_moves: &[
            (12, Confusion),
            (15, PoisonPowder),
            (16, StunSpore),
            (17, SleepPowder),
            (21, Supersonic),
            (26, Whirlwind),
            (32, Psybeam),
        ],
    },
    LearnableMoves{
        species: Weedle,
        level_up_moves: &[],
    },
    LearnableMoves{
        species: Kakuna,
        level_up_moves: &[
            (7, Harden),
        ],
    },
    LearnableMoves{
        species: Beedrill,
        level_up_moves: &[
            (12, FuryAttack),
            (16, FocusEnergy),
            (20, Twineedle),
            (25, Rage),
            (30, PinMissile),
            (35, Agility),
        ],
    },
    LearnableMoves {
        species: Pidgey,
        level_up_moves: &[
            (5, SandAttack),
            (12, QuickAttack),
            (19, Whirlwind),
            (28, WingAttack),
            (36, Agility),
            (44, MirrorMove),
        ],
    },
    LearnableMoves{
        species: Pidgeotto,
        level_up_moves: &[
            (5, SandAttack),
            (12, QuickAttack),
            (21, Whirlwind),
            (31, WingAttack),
            (40, Agility),
            (49, MirrorMove),
        ],
    },
    LearnableMoves{
        species: Pidgeot,
        level_up_moves: &[
            (5, SandAttack),
            (12, QuickAttack),
            (21, Whirlwind),
            (31, WingAttack),
            (44, Agility),
            (54, MirrorMove),
        ],
    },
    LearnableMoves {
        species: Rattata,
        level_up_moves: &[
            (7, QuickAttack),
            (14, HyperFang),
            (23, FocusEnergy),
            (34, SuperFang),
        ],
    },
    LearnableMoves {
        species: Raticate,
        level_up_moves: &[
            (7, QuickAttack),
            (14, HyperFang),
            (27, FocusEnergy),
            (41, SuperFang),
        ],
    },
    LearnableMoves{
        species: Spearow,
        level_up_moves: &[
            (9, Leer),
            (15, FuryAttack),
            (22, MirrorMove),
            (29, DrillPeck),
            (36, Agility),
        ],
    },

    LearnableMoves{
        species: Ekans,
        level_up_moves: &[
            (10, PoisonSting),
            (17, Bite),
            (24, Glare),
            (31, Screech),
            (38, Acid),
        ],
    },

    LearnableMoves {
        species: Pikachu,
        level_up_moves: &[
            (9, ThunderWave),
            (16, QuickAttack),
            (26, Swift),
            (33, Agility),
            (43, Thunder),
        ],
    },

    LearnableMoves{
        species: Sandshrew,
        level_up_moves: &[
            (10, SandAttack),
            (17, Slash),
            (24, PoisonSting),
            (31, Swift),
            (38, FurySwipes),
        ],
    },

    LearnableMoves{
        species: NidoranF,
        level_up_moves: &[
            (8, Scratch),
            (14, PoisonSting),
            (21, TailWhip),
            (29, Bite),
            (36, FurySwipes),
            (43, DoubleKick),
        ],
    },

    LearnableMoves{
        species: Jigglypuff,
        level_up_moves: &[
            (9, Pound),
            (14, Disable),
            (19, DefenseCurl),
            (24, DoubleSlap),
            (29, RestMove),
            (34, BodySlam),
            (39, DoubleEdge),
        ],
    },
    
    LearnableMoves{
        species: Diglett,
        level_up_moves: &[
            (15, Growl),
            (19, Dig),
            (24, SandAttack),
            (31, Slash),
            (40, Earthquake),
        ],
    },
    
    LearnableMoves{
        species: Geodude,
        level_up_moves: &[
            (11, DefenseCurl),
            (16, RockThrow),
            (21, SelfDestruct),
            (26, Harden),
            (31, Earthquake),
            (36, Explosion),
        ],
    },
    LearnableMoves{
        species: Graveler,
        level_up_moves: &[
            (11, DefenseCurl),
            (16, RockThrow),
            (21, SelfDestruct),
            (29, Harden),
            (36, Earthquake),
            (43, Explosion),
        ],
    },
    LearnableMoves{
        species: Golem,
        level_up_moves: &[
            (11, DefenseCurl),
            (16, RockThrow),
            (21, SelfDestruct),
            (29, Harden),
            (36, Earthquake),
            (43, Explosion),
        ],
    },
    LearnableMoves{
        species: Onix,
        level_up_moves: &[
            (15, Bind),
            (19, RockThrow),
            (25, Rage),
            (33, Slam),
            (43, Harden),
        ],
    },


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
