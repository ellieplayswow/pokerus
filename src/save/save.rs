#[derive(Debug)]
pub struct Trainer {
    name: String,
    id: u16,
    secret_id: Option<u16>,
    gender: Gender
}

#[derive(Debug)]
pub enum Gender {
    Male = 0,
    Female = 1
}

#[derive(Debug)]
pub struct SaveFile {
    trainer: Trainer,
    money: u32,
}

impl SaveFile {
    pub fn new(trainer: Trainer, money: u32) -> Self {
        Self {
            trainer,
            money
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