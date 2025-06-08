use crate::save::error::ReadError;
use crate::save::format::dppt::{Gen4StringBuffer, Gen4StringVector};
use crate::save::save::{Gender, SaveFile, Trainer};
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io;
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use crate::save::format::dppt::save::{Badges, Gen4Save, Locale, Timestamp};

pub fn read_save(save_file: impl Into<PathBuf>) -> Result<Gen4Save, ReadError> {
    fn read_u8(readable: &mut impl io::Read) -> Result<u8, ReadError> {
        readable.read_u8().map_err(|_| ReadError::Generic)
    }

    fn read_u16(readable: &mut impl io::Read) -> Result<u16, ReadError> {
        readable.read_u16::<LittleEndian>().map_err(|_| ReadError::Generic)
    }

    fn read_u32(readable: &mut impl io::Read) -> Result<u32, ReadError> {
        readable.read_u32::<LittleEndian>().map_err(|_| ReadError::Generic)
    }

    fn read_string(readable: &mut impl io::Read, length: usize) -> Result<String, ReadError> {
        let mut vec: Vec<u16> = Vec::with_capacity(length);
        for i in 0..length {
            vec.push(read_u16(readable)?);
        }

        Ok(String::from(Gen4StringVector(vec)))
    }

    fn read_date(readable: &mut impl io::Read) -> Result<DateTime<Utc>, ReadError> {
        let timestamp = read_u32(readable)?;
        Ok(Timestamp(timestamp).into())
    }

    fn seek(seekable: &mut impl io::Seek, position: SeekFrom) -> Result<u64, ReadError> {
        seekable.seek(position).map_err(|_| ReadError::Generic)
    }

    let mut save_file = match File::open(save_file.into()) {
        Ok(file) => file,
        Err(_e) => return Err(ReadError::FileNotFound)
    };

    // get start date & HOF date
    seek(&mut save_file, SeekFrom::Start(0x34))?;
    let start_date: DateTime<Utc> = read_date(&mut save_file)?;
    seek(&mut save_file, SeekFrom::Current(0x04))?;  // skip 4 bytes
    let hof_date: DateTime<Utc> = read_date(&mut save_file)?;

    seek(&mut save_file, SeekFrom::Start(0x68))?;
    let trainer_name = read_string(&mut save_file, 8)?;

    let trainer_id = read_u16(&mut save_file)?;
    let trainer_secret_id = read_u16(&mut save_file)?;
    let trainer_money = read_u32(&mut save_file)?;
    let trainer_gender = match read_u8(&mut save_file)? {
        0 => Gender::Male,
        1 => Gender::Female,
        _ => return Err(ReadError::Generic)
    };
    let locale = Locale::from(read_u8(&mut save_file)?);
    let badges: Badges = read_u8(&mut save_file)?.into();

    let trainer = Trainer::new(trainer_name, trainer_id, Some(trainer_secret_id), trainer_gender);
    let base_save = SaveFile::new(trainer, trainer_money);

    // skip to playtime
    seek(&mut save_file, SeekFrom::Start(0x8A))?;
    let playtime = (read_u16(&mut save_file)? as u32 * 3660) + (read_u8(&mut save_file)? as u32 * 60) + (read_u8(&mut save_file)? as u32);
    println!("{:?}", playtime);
    Ok(Gen4Save {
        save_started: start_date,
        hall_of_fame_entered: hof_date,
        base: base_save,
        locale,
        badges: badges.0,
    })
}

#[cfg(test)]
mod tests {
    use crate::save::format::dppt::platinum::read_save;

    #[test]
    fn read_platinum() {
        let read = read_save("test-files/platinum.sav");
        println!("{:#?}", read);
    }
}