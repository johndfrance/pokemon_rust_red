use crate::mon_base_stats::PokemonSpecies;

struct PokeDexEntry{
    dex_num: u8,
    species: PokemonSpecies,
    is_seen: bool,
    is_caught: bool,
    entry_text: String,
    height: String,
}

impl PokeDexEntry {
    fn seen(&mut self){
        self.is_seen = true;
    }
    fn caught(&mut self){
        self.is_caught = true;
    }

}