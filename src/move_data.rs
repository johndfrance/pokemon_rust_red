use crate::{MoveCat, MoveEffectCat, PokeTypes};
use std::collections::binary_heap::PeekMut;

#[derive(Clone)]
pub struct MoveData {
    pub name: &'static str,
    pub base_power: u16,
    pub accuracy: u16,
    pub move_type: PokeTypes,
    pub move_cat: MoveCat,
    pub effect_type: MoveEffectCat,
    pub pp: u16,
}
// List of Moves currently implemented
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Moves {
    Agility,
    Bite,
    Bubble,
    DefenseCurl,
    Ember,
    FireSpin,
    Flamethrower,
    FocusEnergy,
    Growth,
    Growl,
    Gust,
    Harden,
    HydroPump,
    HyperFang,
    LeechSeed,
    Leer,
    MirrorMove,
    Peck,
    PoisonPowder,
    PoisonSting,
    QuickAttack,
    Rage,
    RazorLeaf,
    SandAttack,
    Scratch,
    Screech,
    Sing,
    SkullBash,
    Slash,
    SleepPowder,
    SolarBeam,
    StringShot,
    SuperFang,
    Swift,
    Tackle,
    Thunder,
    ThunderShock,
    ThunderWave,
    TailWhip,
    VineWhip,
    WaterGun,
    Whirlwind,
    WingAttack,
    Withdraw,
    Wrap,
}
impl Moves {
    pub fn move_stats(&self) -> MoveData {
        match self {
            Moves::Agility => AGILITY,
            Moves::Bite => BITE,
            Moves::Bubble => BUBBLE,
            Moves::Ember => EMBER,
            Moves::DefenseCurl=>DEFENSECURL,
            Moves::FireSpin => FIRESPIN,
            Moves::Flamethrower => FLAMETHROWER,
            Moves::FocusEnergy => FOCUSENERGY,
            Moves::Growl => GROWL,
            Moves::Growth => GROWTH,
            Moves::Gust => GUST,
            Moves::Harden=>HARDEN,
            Moves::HydroPump => HYDROPUMP,
            Moves::HyperFang => HYPERFANG,
            Moves::LeechSeed => LEECHSEED,
            Moves::Leer => LEER,
            Moves::MirrorMove => MIRRORMOVE,
            Moves::Peck=>PECK,
            Moves::PoisonPowder => POISONPOWDER,
            Moves::PoisonSting => POISONSTING,
            Moves::QuickAttack => QUICKATTACK,
            Moves::Rage => RAGE,
            Moves::RazorLeaf => RAZORLEAF,
            Moves::SandAttack => SANDATTACK,
            Moves::Scratch => SCRATCH,
            Moves::Screech=>SCREECH,
            Moves::Sing => SING,
            Moves::SkullBash => SKULLBASH,
            Moves::Slash => SLASH,
            Moves::SleepPowder => SLEEPPOWDER,
            Moves::SolarBeam => SOLARBEAM,
            Moves::StringShot => STRINGSHOT,
            Moves::SuperFang => SUPERFANG,
            Moves::Swift => SWIFT,
            Moves::Tackle => TACKLE,
            Moves::TailWhip => TAILWHIP,
            Moves::Thunder => THUNDER,
            Moves::ThunderShock => THUNDERSHOCK,
            Moves::ThunderWave => THUNDERWAVE,
            Moves::VineWhip => VINEWHIP,
            Moves::WaterGun => WATERGUN,
            Moves::Whirlwind => WHIRLWIND,
            Moves::Withdraw => WITHDRAW,
            Moves::WingAttack => WINGATTACK,
            Moves::Wrap=>WRAP
        }
    }
}

// Data for each move, as a constant instance of MoveData struct.
pub const AGILITY: MoveData = MoveData {
    name: "Agility",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Psychic,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 30,
};
pub const BITE: MoveData = MoveData {
    name: "Bite",
    base_power: 60,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 25,
};
pub const BUBBLE: MoveData = MoveData {
    name: "Bubble",
    base_power: 20,
    accuracy: 100,
    move_type: PokeTypes::Water,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::SpeedDown1,
    pp: 30,
};
const DEFENSECURL: MoveData=MoveData{
    name: "Defense Curl",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::DefenseUp1,
    pp: 40,
};
pub const EMBER: MoveData = MoveData {
    name: "Ember",
    base_power: 40,
    accuracy: 100,
    move_type: PokeTypes::Fire,
    move_cat: MoveCat::Special,
    effect_type: MoveEffectCat::BurnSideEffect1,
    pp: 25,
};
pub const FIRESPIN: MoveData = MoveData {
    name: "Fire Spin",
    base_power: 15,
    accuracy: 70,
    move_type: PokeTypes::Fire,
    move_cat: MoveCat::Special,
    effect_type: MoveEffectCat::None, //TODO
    pp: 15,
};
pub const FLAMETHROWER: MoveData = MoveData {
    name: "Flamethrower",
    base_power: 95,
    accuracy: 100,
    move_type: PokeTypes::Fire,
    move_cat: MoveCat::Special,
    effect_type: MoveEffectCat::None, //TODO
    pp: 100,
};
pub const FOCUSENERGY: MoveData = MoveData {
    name: "Focus Energy",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 30,
};
pub const GROWL: MoveData = MoveData {
    name: "Growl",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::AttackDown1,
    pp: 40,
};
pub const GROWTH: MoveData = MoveData {
    name: "Growth",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, // TODO
    pp: 40,
};
pub const GUST: MoveData = MoveData {
    name: "Gust",
    base_power: 45,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None,
    pp: 35,
};
const HARDEN: MoveData = MoveData{
    name: "Harden",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 30,
};
pub const HYDROPUMP: MoveData = MoveData {
    name: "Hydro Pump",
    base_power: 120,
    accuracy: 80,
    move_type: PokeTypes::Water,
    move_cat: MoveCat::Special,
    effect_type: MoveEffectCat::None,
    pp: 5,
};
pub const HYPERFANG: MoveData = MoveData {
    name: "Hyper Fang",
    base_power: 80,
    accuracy: 90,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 15,
};
pub const LEECHSEED: MoveData = MoveData {
    name: "Leech Seed",
    base_power: 0,
    accuracy: 90,
    move_type: PokeTypes::Grass,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::LeechSeed,
    pp: 10,
};
pub const LEER: MoveData = MoveData {
    name: "Leer",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 30,
};
pub const MIRRORMOVE: MoveData = MoveData {
    name: "Mirror Move",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Flying,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 20,
};
const PECK: MoveData=MoveData{
    name: "Peck",
    base_power: 35,
    accuracy: 100,
    move_type: PokeTypes::Flying,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None,
    pp: 35,
};
pub const POISONPOWDER: MoveData = MoveData {
    name: "Poison Powder",
    base_power: 0,
    accuracy: 75,
    move_type: PokeTypes::Poison,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 35,
};
pub const POISONSTING: MoveData = MoveData {
    name: "Poison Sting",
    base_power: 15,
    accuracy: 100,
    move_type: PokeTypes::Poison,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 35,
};
pub const QUICKATTACK: MoveData = MoveData {
    name: "Quick Attack",
    base_power: 40,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 30,
};
pub const RAGE: MoveData = MoveData {
    name: "Rage",
    base_power: 20,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 100,
};
pub const RAZORLEAF: MoveData = MoveData {
    name: "Razor Leaf",
    base_power: 55,
    accuracy: 95,
    move_type: PokeTypes::Grass,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None,
    pp: 25,
};
pub const SANDATTACK: MoveData = MoveData {
    name: "Sand-Attack",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 15,
};
pub const SCRATCH: MoveData = MoveData {
    name: "Scratch",
    base_power: 40,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None,
    pp: 35,
};
const SCREECH: MoveData=MoveData{
    name: "Screech",
    base_power: 0,
    accuracy: 85,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::DefenseDown1, //Todo Should be Defnese down 2
    pp: 40,
};
const SING: MoveData=MoveData{
    name: "Sing",
    base_power: 0,
    accuracy: 55,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 15,
};
pub const SKULLBASH: MoveData = MoveData {
    name: "Skull Bash",
    base_power: 100,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 15,
};
pub const SLASH: MoveData = MoveData {
    name: "Slash",
    base_power: 70,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 20,
};
pub const SLEEPPOWDER: MoveData = MoveData {
    name: "Sleep Powder",
    base_power: 0,
    accuracy: 75,
    move_type: PokeTypes::Grass,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, // TODO
    pp: 15,
};
pub const SOLARBEAM: MoveData = MoveData {
    name: "Solar Beam",
    base_power: 120,
    accuracy: 100,
    move_type: PokeTypes::Grass,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, // TODO - Takes two turns: First turn charge, second turn fire.
    pp: 10,
};
pub const STRINGSHOT: MoveData = MoveData {
    name: "String Shot",
    base_power: 0,
    accuracy: 95,
    move_type: PokeTypes::Bug,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 40,
};
pub const SUPERFANG: MoveData = MoveData {
    name: "Super Fang",
    base_power: 50, // This move always cuts the enemy's HP by half, this is a temp value.
    accuracy: 90,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 10,
};
pub const SWIFT: MoveData = MoveData {
    name: "Swift",
    base_power: 60,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None,
    pp: 20,
};
pub const TACKLE: MoveData = MoveData {
    name: "Tackle",
    base_power: 35,
    accuracy: 95,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None,
    pp: 35,
};
pub const TAILWHIP: MoveData = MoveData {
    name: "Tail Whip",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::DefenseDown1,
    pp: 30,
};
pub const THUNDER: MoveData = MoveData {
    name: "Thunder",
    base_power: 120,
    accuracy: 70,
    move_type: PokeTypes::Electric,
    move_cat: MoveCat::Special,
    effect_type: MoveEffectCat::None, //TODO
    pp: 10,
};
pub const THUNDERSHOCK: MoveData = MoveData {
    name: "Thunder Shock",
    base_power: 40,
    accuracy: 100,
    move_type: PokeTypes::Electric,
    move_cat: MoveCat::Special,
    effect_type: MoveEffectCat::None, //TODO
    pp: 30,
};
pub const THUNDERWAVE: MoveData = MoveData {
    name: "Thunder Wave",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Electric,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 20,
};
pub const VINEWHIP: MoveData = MoveData {
    name: "Vine Whip",
    base_power: 35,
    accuracy: 100,
    move_type: PokeTypes::Grass,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None,
    pp: 10,
};
pub const WATERGUN: MoveData = MoveData {
    name: "Water Gun",
    base_power: 40,
    accuracy: 100,
    move_type: PokeTypes::Water,
    move_cat: MoveCat::Special,
    effect_type: MoveEffectCat::None,
    pp: 25,
};
pub const WHIRLWIND: MoveData = MoveData {
    name: "Whirlwind",
    base_power: 0,
    accuracy: 85,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 20,
};
pub const WINGATTACK: MoveData = MoveData {
    name: "Wing Attack",
    base_power: 35,
    accuracy: 100,
    move_type: PokeTypes::Flying,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None,
    pp: 35,
};
pub const WITHDRAW: MoveData = MoveData {
    name: "Withdraw",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Water,
    move_cat: MoveCat::Special,
    effect_type: MoveEffectCat::None, //TODO
    pp: 40,
};
const WRAP:MoveData=MoveData{
    name: "Wrap",
    base_power: 15,
    accuracy: 85,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 20,
};