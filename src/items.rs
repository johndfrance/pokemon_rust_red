struct Backpack{

}
struct StdItem{
    item_type: Item,
    price: u16,
    description: String,
}

trait ItemFeatures{
    fn use_item(){}
    fn get_description(){}
}
enum Item{
    PokeBall(PokeBallType),
    Potions(PotionType),
    //Heals(HealType),
    EvolutionStones(EvolutionStones),
    TmHmTypes(TmHmTypes),
    //KeyItems(KeyItems)
}
enum PokeBallType{
    PokeBall,
    GreatBall,
    UltraBall,
    MasterBall,
}
enum EvolutionStones{
    FireStone,
    WaterStone,
    ThunderStone,
    LeafStone,
    MoonStone,
}
enum PotionType{
    Potion,
    SuperPotion,
    HyperPotion,
    MaxPotion,
    FullRestore,
}
enum HealType{
    Antidote,
    BurnHeal,
    IceHeal,
    Awakening,
    ParlyzHeal,
}
enum RepelType{
    Repel,
    SuperRepel,
    MaxRepel,
}
enum TmHmTypes{
    TM1,TM2,TM3,TM4,TM5,TM6,TM7,TM8,TM9,TM10,TM11,TM12,TM13,TM14,TM15,TM16,TM17,TM18,TM19,TM20,TM21,TM22,TM23,TM24,TM25,TM26,TM27,TM28,TM29,TM30,TM31,TM32,TM33,TM34,TM35,TM36,TM37,TM38,TM39,TM40,TM41,TM42,TM43,TM44,TM45,TM46,TM47,TM48,TM49,TM50,TM51,TM52,TM53,TM54,TM55,HM1,HM2,HM3,HM4,HM5,
}
enum KeyItems{
    BikeVoucher,
    CardKey,
    CoinCase,
    DomeFossil,
    HelixFossil,
    GoldTeeth,
    OldRod,
    GoodRod,
    SuperRod,
    LiftKey,
    OaksParcel,
    OldAmber,
    PokeFlute,
    SecretKey,
    SilphScope,
    SSTicket,
    Map,
}
enum StatBooster{
    RareCandy,
    PPUp,
    Protein,
    Iron,
    Carbos,
    Calcium,
}

enum OddItems{
    PokeDoll,
    Nugget,
}

enum Badges{
    Boulder,
    Cascade,
    Thunder,
    Rainbow,
    Soul,
    Marsh,
    Volcano,
    Earth,
}