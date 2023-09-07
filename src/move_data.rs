use crate::{MoveCat, MoveEffectCat, PokeTypes};
use std::collections::binary_heap::PeekMut;
use serde::{Serialize, Deserialize};
use crate::MoveCat::{Special, Status};
use crate::type_matchups::PokeTypes::{Bug, Fighting, Flying, Grass, Ground, Ice, Poison, Psychic, Rock};

#[derive(Clone, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
pub enum Moves {
    Acid,
    Agility,
    Bind,
    Bite,
    BodySlam,
    Bubble,
    Confusion,
    DefenseCurl,
    Dig,
    Disable,
    DoubleEdge,
    DoubleKick,
    DoubleSlap,
    DrillPeck,
    Earthquake,
    Ember,
    Explosion,
    FireSpin,
    Flamethrower,
    FocusEnergy,
    FuryAttack,
    FurySwipes,
    Glare,
    Growth,
    Growl,
    Gust,
    Harden,
    HydroPump,
    HyperFang,
    Hypnosis,
    LeechLife,
    LeechSeed,
    Leer,
    MirrorMove,
    Mist,
    Peck,
    PinMissile,
    PoisonPowder,
    PoisonSting,
    Pound,
    Psybeam,
    QuickAttack,
    Rage,
    RazorLeaf,
    RestMove,
    RockThrow,
    SandAttack,
    Scratch,
    Screech,
    SelfDestruct,
    Sing,
    SkullBash,
    Slam,
    Slash,
    SleepPowder,
    SolarBeam,
    StringShot,
    StunSpore,
    SuperFang,
    Supersonic,
    Swift,
    Tackle,
    Thunder,
    ThunderShock,
    ThunderWave,
    Twineedle,
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
            Moves::Acid=>ACID,
            Moves::Agility => AGILITY,
            Moves::Bind=>BIND,
            Moves::Bite => BITE,
            Moves::BodySlam=>BODYSLAM,
            Moves::Bubble => BUBBLE,
            Moves::Confusion=>CONFUSION,
            Moves::Earthquake=>EARTHQUAKE,
            Moves::Ember => EMBER,
            Moves::Explosion=>EXPLOSION,
            Moves::DefenseCurl=>DEFENSECURL,
            Moves::Dig=>DIG,
            Moves::Disable=>DISABLE,
            Moves::DoubleEdge=>DOUBLEEDGE,
            Moves::DoubleKick=>DOUBLEKICK,
            Moves::DoubleSlap=>DOUBLESLAP,
            Moves::DrillPeck=>DRILLPECK,
            Moves::FireSpin => FIRESPIN,
            Moves::Flamethrower => FLAMETHROWER,
            Moves::FocusEnergy => FOCUSENERGY,
            Moves::FuryAttack=>FURYATTACK,
            Moves::FurySwipes=>FURYSWIPES,
            Moves::Glare=>GLARE,
            Moves::Growl => GROWL,
            Moves::Growth => GROWTH,
            Moves::Gust => GUST,
            Moves::Harden=>HARDEN,
            Moves::HydroPump => HYDROPUMP,
            Moves::HyperFang => HYPERFANG,
            Moves::Hypnosis => HYPNOSIS,
            Moves::LeechLife=>LEECHLIFE,
            Moves::LeechSeed => LEECHSEED,
            Moves::Leer => LEER,
            Moves::MirrorMove => MIRRORMOVE,
            Moves::Mist=>MIST,
            Moves::Peck=>PECK,
            Moves::PinMissile=>PINMISSILE,
            Moves::PoisonPowder => POISONPOWDER,
            Moves::PoisonSting => POISONSTING,
            Moves::Pound=>POUND,
            Moves::Psybeam=>PSYBEAM,
            Moves::QuickAttack => QUICKATTACK,
            Moves::Rage => RAGE,
            Moves::RazorLeaf => RAZORLEAF,
            Moves::RestMove=>REST,
            Moves::RockThrow=>ROCKTHROW,
            Moves::SandAttack => SANDATTACK,
            Moves::Scratch => SCRATCH,
            Moves::Screech=>SCREECH,
            Moves::SelfDestruct=>SELFDESTRUCT,
            Moves::Sing => SING,
            Moves::SkullBash => SKULLBASH,
            Moves::Slam=>SLAM,
            Moves::Slash => SLASH,
            Moves::SleepPowder => SLEEPPOWDER,
            Moves::SolarBeam => SOLARBEAM,
            Moves::StringShot => STRINGSHOT,
            Moves::StunSpore=>STUNSPORE,
            Moves::SuperFang => SUPERFANG,
            Moves::Supersonic=>SUPERSONIC,
            Moves::Swift => SWIFT,
            Moves::Tackle => TACKLE,
            Moves::TailWhip => TAILWHIP,
            Moves::Thunder => THUNDER,
            Moves::ThunderShock => THUNDERSHOCK,
            Moves::ThunderWave => THUNDERWAVE,
            Moves::Twineedle=>TWINEEDLE,
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
const ACID: MoveData = MoveData{
    name: "Acid",
    base_power: 40,
    accuracy: 100,
    move_type: Poison,
    move_cat: Special,
    effect_type: MoveEffectCat::Poisoned,
    pp: 30,
};
pub const AGILITY: MoveData = MoveData {
    name: "Agility",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Psychic,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 30,
};
const BIND: MoveData = MoveData{
    name: "Bind",
    base_power: 15,
    accuracy: 75,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 20,
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
const BODYSLAM: MoveData = MoveData{
    name: "Body Slam",
    base_power: 85,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 15,
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
const CONFUSION: MoveData =MoveData{
    name: "Confusion",
    base_power: 50,
    accuracy: 100,
    move_type: Psychic,
    move_cat: MoveCat::Special,
    effect_type: MoveEffectCat::None, //TODO
    pp: 25,
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
const DIG: MoveData = MoveData{
    name: "Dig",
    base_power: 100,
    accuracy: 100,
    move_type: Ground,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 10,
};
const DISABLE: MoveData = MoveData{
    name: "Disable",
    base_power: 0,
    accuracy: 55,
    move_type: PokeTypes::Normal,
    move_cat: Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 20,
};
const DOUBLEEDGE: MoveData = MoveData{
    name: "Double-Edge",
    base_power: 100,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 15,
};
const DOUBLEKICK: MoveData = MoveData{
    name: "Double Kick",
    base_power: 30,
    accuracy: 100,
    move_type: Fighting,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 30,
};
const DOUBLESLAP: MoveData = MoveData{
    name: "Double Slap",
    base_power: 15,
    accuracy: 85,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 10,
};
const DRILLPECK: MoveData = MoveData{
    name: "Drill Peck",
    base_power: 80,
    accuracy: 100,
    move_type: Flying,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None,
    pp: 20,
};
const EARTHQUAKE: MoveData = MoveData{
    name: "Earthquake",
    base_power: 100,
    accuracy: 100,
    move_type: Ground,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None,
    pp: 10,
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
const EXPLOSION: MoveData = MoveData{
    name: "Explosion",
    base_power: 170,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 5,
};
pub const FIRESPIN: MoveData = MoveData {
    name: "Fire Spin",
    base_power: 15,
    accuracy: 70,
    move_type: PokeTypes::Fire,
    move_cat: Special,
    effect_type: MoveEffectCat::BurnSideEffect1,
    pp: 15,
};
pub const FLAMETHROWER: MoveData = MoveData {
    name: "Flamethrower",
    base_power: 95,
    accuracy: 100,
    move_type: PokeTypes::Fire,
    move_cat: Special,
    effect_type: MoveEffectCat::BurnSideEffect1,
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
pub const FURYATTACK: MoveData = MoveData{
    name: "Fury Attack",
    base_power: 15,
    accuracy: 85,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 20,
};
const FURYSWIPES: MoveData = MoveData{
    name: "Fury Swipes",
    base_power: 18,
    accuracy: 80,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 15,
};
const GLARE: MoveData = MoveData{
    name: "Glare",
    base_power: 0,
    accuracy: 75,
    move_type: PokeTypes::Normal,
    move_cat: Status,
    effect_type: MoveEffectCat::Paralyzed,
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
    effect_type: MoveEffectCat::DefenseUp1, //TODO
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
const HYPNOSIS: MoveData = MoveData{
    name: "Hypnosis",
    base_power: 0,
    accuracy: 60,
    move_type: Psychic,
    move_cat: Status,
    effect_type: MoveEffectCat::Sleeped,
    pp: 20,
};
const LEECHLIFE: MoveData = MoveData{
    name: "Leech Life",
    base_power: 20,
    accuracy: 100,
    move_type: PokeTypes::Bug,
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
    effect_type: MoveEffectCat::DefenseDown1, //TODO
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
const MIST: MoveData = MoveData{
    name: "Mist",
    base_power: 30,
    accuracy: 100,
    move_type: Ice,
    move_cat: Special,
    effect_type: MoveEffectCat::None,
    pp: 30,
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
const PINMISSILE: MoveData = MoveData{
    name: "Pin Missile",
    base_power: 14,
    accuracy: 85,
    move_type: Bug,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 20,
};
pub const POISONPOWDER: MoveData = MoveData {
    name: "Poison Powder",
    base_power: 0,
    accuracy: 75,
    move_type: PokeTypes::Poison,
    move_cat: Status,
    effect_type: MoveEffectCat::Poisoned, //TODO
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
const POUND: MoveData = MoveData{
    name: "Pound",
    base_power: 40,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None,
    pp: 35,
};
pub const PSYBEAM: MoveData = MoveData{
    name: "Psybeam",
    base_power: 65,
    accuracy: 100,
    move_type: Psychic,
    move_cat: Special,
    effect_type: MoveEffectCat::None, //TODO
    pp: 20,
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
const REST: MoveData = MoveData{
    name: "Rest",
    base_power: 0,
    accuracy: 100,
    move_type: Psychic,
    move_cat: Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 10,
};
const ROCKTHROW: MoveData = MoveData{
    name: "Rock Throw",
    base_power: 50,
    accuracy: 65,
    move_type: Rock,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None,
    pp: 15,
};
pub const SANDATTACK: MoveData = MoveData {
    name: "Sand-Attack",
    base_power: 0,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::AccuracyDown1,
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
const SELFDESTRUCT: MoveData = MoveData{
    name: "Self-Destruct",
    base_power: 130,
    accuracy: 100,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 5,
};
const SING: MoveData=MoveData{
    name: "Sing",
    base_power: 0,
    accuracy: 55,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Status,
    effect_type: MoveEffectCat::Sleeped,
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
const SLAM: MoveData = MoveData{
    name: "Slam",
    base_power: 80,
    accuracy: 75,
    move_type: PokeTypes::Normal,
    move_cat: MoveCat::Physical,
    effect_type: MoveEffectCat::None, //TODO
    pp: 20,
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
    effect_type: MoveEffectCat::Sleeped,
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
    effect_type: MoveEffectCat::EvasionDown1,
    pp: 40,
};
pub const STUNSPORE: MoveData = MoveData{
    name: "Stun Spore",
    base_power: 0,
    accuracy: 75,
    move_type: Grass,
    move_cat: Status,
    effect_type: MoveEffectCat::Paralyzed,
    pp: 30,
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
pub const SUPERSONIC: MoveData =MoveData{
    name: "Supersonic",
    base_power: 0,
    accuracy: 55,
    move_type: PokeTypes::Normal,
    move_cat: Status,
    effect_type: MoveEffectCat::None, //TODO
    pp: 20,
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
    effect_type: MoveEffectCat::Paralyzed,
    pp: 20,
};
pub const TWINEEDLE: MoveData = MoveData{
    name: "Twineedle",
    base_power: 25,
    accuracy: 100,
    move_type: Bug,
    move_cat: MoveCat::Physical,
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