use crate::save::data::species::Species;

/// A trainer
///
/// Depending on the context, we may not be able to retrieve all the information of a `Trainer` when
/// reading a save file.
#[derive(Debug, Clone)]
pub struct Trainer {
    name: String,
    id: u16,
    secret_id: Option<u16>,
    gender: Gender
}

#[derive(Debug, Clone)]
pub enum Gender {
    Male = 0,
    Female = 1
}

/// A generic, non-generation specific Pokemon
///
/// The base Pokemon struct should be used wherever possible, and if it needs to be 'upgraded' to a
/// specific generation then do so via (eventually provided) `GenXPokemon::from(pokemon)`
///
/// # Examples
///
/// The only thing to create a `Pokemon` is a `Species`:
///
/// ```
/// use pokerus::save::data::species::Species;
/// use pokerus::save::save::Pokemon;
/// let pkmn = Pokemon::new(Species::Piplup);
/// // if you have a national dex number
/// let pkmn = Pokemon::new(Species::from(393));
/// ```
///
#[derive(Debug)]
pub struct Pokemon {
    name: String,
    species: Species,
    trainer: Option<Trainer>,
    experience: u32,
    friendship: u8,
}

impl Pokemon {
    pub fn new(species: Species) -> Self {
        Self {
            name: String::new(),
            species,
            trainer: None,
            experience: 0,
            friendship: 0
        }
    }

    pub fn set_trainer(&mut self, trainer: Trainer) {
        self.trainer = Some(trainer);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

/// A generic, non-generation specific save file
#[derive(Debug)]
pub struct SaveFile {
    trainer: Trainer,
    money: u32,
    pub party: Vec<Pokemon>,
}

impl SaveFile {
    pub fn new(trainer: Trainer, money: u32) -> Self {
        Self {
            trainer,
            money,
            party: Vec::new()
        }
    }
}

impl Trainer {
    pub fn new(name: String, id: u16, secret_id: Option<u16>, gender: Gender) -> Self {
        Self {
            name,
            id,
            secret_id,
            gender
        }
    }
}