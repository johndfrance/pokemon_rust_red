#[derive(PartialEq, Clone, Debug, Copy)]
pub enum PokeTypes {
    Normal,
    Grass,
    Fire,
    Water,
    Electric,
    Rock,
    Bug,
    Poison,
    Ground,
    Flying,
    Ghost,
    Fighting,
    Psychic,
    Ice,
    Dragon,
    None,
}

pub enum Effectivness {
    NoEffect,
    NotVeryEffective,
    NormalEffective,
    SuperEffective,
}

impl Effectivness {
    pub fn effectivness_modifier(&self) -> f32 {
        match self {
            Effectivness::NoEffect => 0.0,
            Effectivness::NotVeryEffective => 0.5,
            Effectivness::NormalEffective => 1.0,
            Effectivness::SuperEffective => 2.0,
        }
    }
    pub fn flavour_text(&self) {
        match self {
            Effectivness::NoEffect => println!("Attack had no effect!"),
            Effectivness::NotVeryEffective => println!("Attack was not very effective!"),
            Effectivness::NormalEffective => {}
            Effectivness::SuperEffective => println!("Attack was super effective!"),
        }
    }
}

impl PokeTypes {
    pub fn type_match_board(&self, defender: &PokeTypes) -> Effectivness {
        match self {
            PokeTypes::Normal => match defender {
                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Fire => match defender {
                PokeTypes::Water => Effectivness::NotVeryEffective,
                PokeTypes::Rock => Effectivness::NotVeryEffective,
                PokeTypes::Fire => Effectivness::NotVeryEffective,
                PokeTypes::Dragon => Effectivness::NotVeryEffective,

                PokeTypes::Grass => Effectivness::SuperEffective,
                PokeTypes::Bug => Effectivness::SuperEffective,
                PokeTypes::Ice => Effectivness::SuperEffective,
                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Fighting => match defender {
                PokeTypes::Normal => Effectivness::SuperEffective,
                PokeTypes::Rock => Effectivness::SuperEffective,
                PokeTypes::Ice => Effectivness::SuperEffective,

                PokeTypes::Flying => Effectivness::NotVeryEffective,
                PokeTypes::Poison => Effectivness::NotVeryEffective,
                PokeTypes::Bug => Effectivness::NotVeryEffective,

                PokeTypes::Ghost => Effectivness::NoEffect,
                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Flying => match defender {
                PokeTypes::Fighting => Effectivness::SuperEffective,
                PokeTypes::Bug => Effectivness::SuperEffective,
                PokeTypes::Grass => Effectivness::SuperEffective,
                PokeTypes::Rock => Effectivness::NotVeryEffective,
                PokeTypes::Electric => Effectivness::NotVeryEffective,
                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Poison => match defender {
                PokeTypes::Bug => Effectivness::SuperEffective,
                PokeTypes::Grass => Effectivness::SuperEffective,
                PokeTypes::Poison => Effectivness::NotVeryEffective,
                PokeTypes::Rock => Effectivness::NotVeryEffective,
                PokeTypes::Ground => Effectivness::NotVeryEffective,
                PokeTypes::Ghost => Effectivness::NotVeryEffective,
                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Ground => match defender {
                PokeTypes::Poison => Effectivness::SuperEffective,
                PokeTypes::Rock => Effectivness::SuperEffective,
                PokeTypes::Fire => Effectivness::SuperEffective,
                PokeTypes::Electric => Effectivness::SuperEffective,
                PokeTypes::Bug => Effectivness::NotVeryEffective,
                PokeTypes::Grass => Effectivness::NotVeryEffective,
                PokeTypes::Flying => Effectivness::NoEffect,
                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Rock => match defender {
                PokeTypes::Flying => Effectivness::SuperEffective,
                PokeTypes::Bug => Effectivness::SuperEffective,
                PokeTypes::Fire => Effectivness::SuperEffective,
                PokeTypes::Ice => Effectivness::SuperEffective,

                PokeTypes::Fighting => Effectivness::NotVeryEffective,
                PokeTypes::Ground => Effectivness::NotVeryEffective,

                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Bug => match defender {
                PokeTypes::Poison => Effectivness::SuperEffective,
                PokeTypes::Grass => Effectivness::SuperEffective,
                PokeTypes::Psychic => Effectivness::SuperEffective,

                PokeTypes::Fighting => Effectivness::NotVeryEffective,
                PokeTypes::Flying => Effectivness::NotVeryEffective,
                PokeTypes::Ghost => Effectivness::NotVeryEffective,
                PokeTypes::Fire => Effectivness::NotVeryEffective,

                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Ghost => match defender {
                PokeTypes::Ghost => Effectivness::SuperEffective,

                PokeTypes::Normal => Effectivness::NoEffect,
                PokeTypes::Psychic => Effectivness::NoEffect,

                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Water => match defender {
                PokeTypes::Ground => Effectivness::SuperEffective,
                PokeTypes::Rock => Effectivness::SuperEffective,
                PokeTypes::Fire => Effectivness::SuperEffective,

                PokeTypes::Water => Effectivness::NotVeryEffective,
                PokeTypes::Grass => Effectivness::NotVeryEffective,
                PokeTypes::Dragon => Effectivness::NotVeryEffective,
                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Grass => match defender {
                PokeTypes::Ground => Effectivness::SuperEffective,
                PokeTypes::Rock => Effectivness::SuperEffective,
                PokeTypes::Water => Effectivness::SuperEffective,

                PokeTypes::Flying => Effectivness::NotVeryEffective,
                PokeTypes::Poison => Effectivness::NotVeryEffective,
                PokeTypes::Bug => Effectivness::NotVeryEffective,
                PokeTypes::Fire => Effectivness::NotVeryEffective,
                PokeTypes::Grass => Effectivness::NotVeryEffective,
                PokeTypes::Dragon => Effectivness::NotVeryEffective,
                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Electric => match defender {
                PokeTypes::Flying => Effectivness::SuperEffective,
                PokeTypes::Water => Effectivness::SuperEffective,

                PokeTypes::Grass => Effectivness::NotVeryEffective,
                PokeTypes::Rock => Effectivness::NotVeryEffective,
                PokeTypes::Dragon => Effectivness::NotVeryEffective,

                PokeTypes::Ground => Effectivness::NoEffect,
                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Psychic => match defender {
                PokeTypes::Fighting => Effectivness::SuperEffective,
                PokeTypes::Poison => Effectivness::SuperEffective,

                PokeTypes::Psychic => Effectivness::NotVeryEffective,
                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Ice => match defender {
                PokeTypes::Flying => Effectivness::SuperEffective,
                PokeTypes::Ground => Effectivness::SuperEffective,
                PokeTypes::Grass => Effectivness::SuperEffective,
                PokeTypes::Dragon => Effectivness::SuperEffective,

                PokeTypes::Water => Effectivness::NotVeryEffective,
                PokeTypes::Ice => Effectivness::NotVeryEffective,
                _ => Effectivness::NormalEffective,
            },
            PokeTypes::Dragon => match defender {
                PokeTypes::Dragon => Effectivness::SuperEffective,
                _ => Effectivness::NormalEffective,
            },
            PokeTypes::None => Effectivness::NormalEffective,
        }
    }
}
