use std::collections::HashMap;
use crate::save::data::dppt::item::DPPTItem;
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

//pub type BoxDecoration = HashMap<u32, u32>;
#[derive(Debug)]
pub struct Box {
    name: String,
    pokemon: HashMap<usize, Pokemon>,
    wallpaper: u8
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
    inventory: HashMap<DPPTItem, u16>, // @todo: convert to a Vec<Pocket>, where a Pocket -> HashMap<Item, u16>
    pub boxes: Vec<Box>
}

impl SaveFile {
    pub fn new(trainer: Trainer, money: u32) -> Self {
        Self {
            trainer,
            money,
            party: Vec::new(),
            inventory: HashMap::new(),
            boxes: Vec::new()
        }
    }

    /// Sets the `qty` of `item` in the save's inventory.
    ///
    /// # Examples
    /// ```
    /// use pokerus::save::data::dppt::item::DPPTItem;
    /// use pokerus::save::save::{Gender, SaveFile, Trainer};
    /// let mut save_file = SaveFile::new(Trainer::new("Trainer".into(), 123, Some(456), Gender::Female), 0);
    ///
    /// save_file.add_item(DPPTItem::MasterBall, 10);
    /// let mut qty = save_file.get_item(DPPTItem::MasterBall);
    /// assert_eq!(*qty.unwrap(), 10);
    ///
    /// save_file.add_item(DPPTItem::MasterBall, 100);
    /// qty = save_file.get_item(DPPTItem::MasterBall);
    /// assert_eq!(*qty.unwrap(), 100);
    /// ```
    pub fn set_item(&mut self, item: DPPTItem, qty: u16) {
        self.inventory.insert(item, qty);
    }

    /// Adds `qty` of `item` to the save's inventory.
    ///
    /// # Examples
    /// ```
    /// use pokerus::save::data::dppt::item::DPPTItem;
    /// use pokerus::save::save::{Gender, SaveFile, Trainer};
    /// let mut save_file = SaveFile::new(Trainer::new("Trainer".into(), 123, Some(456), Gender::Female));
    ///
    /// save_file.add_item(DPPTItem::MasterBall, 10);
    /// let mut qty = save_file.get_item(DPPTItem::MasterBall);
    /// assert_eq!(*qty.unwrap(), 10);
    ///
    /// save_file.add_item(DPPTItem::MasterBall, 100);
    /// qty = save_file.get_item(DPPTItem::MasterBall);
    ///  assert_eq!(*qty.unwrap(), 110);
    /// ```
    pub fn add_item(&mut self, item: DPPTItem, qty: u16) {
        let qty = match self.inventory.get(&item) {
            Some(old_qty) => qty + old_qty,
            None => qty
        };

        self.inventory.insert(item, qty);
    }

    pub fn get_item(&self, item: DPPTItem) -> Option<&u16> {
        self.inventory.get(&item)
    }

    pub fn has_item(&self, item: DPPTItem) -> bool {
        self.inventory.contains_key(&item)
    }

    pub fn get_box(&self, box_index: usize) -> &Box {
        &self.boxes[box_index]
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

impl Box {
    pub fn new(box_size: usize) -> Self {
        Self {
            name: String::new(),
            pokemon: HashMap::new(),
            wallpaper: 0
        }
    }

    pub fn set_pkmn(&mut self, index: usize, pokemon: Pokemon) {
        self.pokemon.insert(index, pokemon);
    }

    pub fn get_pkmn(&self, index: usize) -> Option<&Pokemon> {
        self.pokemon.get(&index)
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn name(self) -> String {
        self.name
    }

    pub fn pkmn(self) -> HashMap<usize, Pokemon> {
        self.pokemon
    }

    pub fn set_wallpaper(&mut self, wallpaper: u8) {
        self.wallpaper = wallpaper;
    }

    pub fn wallpaper(self) -> u8 {
        self.wallpaper
    }
}