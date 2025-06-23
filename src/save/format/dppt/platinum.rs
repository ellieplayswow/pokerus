use crate::save::error::ReadError;
use crate::save::format::dppt::Gen4StringVector;
use crate::save::save::{Gender, Pokemon, SaveFile, Trainer};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io;
use std::io::{Cursor, Read, SeekFrom};
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use crate::save::data::dppt::item::DPPTItem;
use crate::save::data::species::Species;
use crate::save::format::dppt::save::{Badges, Gen4Save, Locale, Timestamp};

const PADDING_BETWEEN_ENTRIES: i64 = 0x08;

pub fn read_save(save_file: impl Into<PathBuf>) -> Result<Gen4Save, ReadError> {

    fn read_date(readable: &mut impl io::Read) -> Result<DateTime<Utc>, ReadError> {
        let timestamp = read_i64(readable)?;
        Ok(Timestamp(timestamp).into())
    }

    fn seek(seekable: &mut impl io::Seek, position: SeekFrom) -> Result<u64, ReadError> {
        seekable.seek(position).map_err(|_| ReadError::Generic)
    }

    let mut save_file = match File::open(save_file.into()) {
        Ok(file) => file,
        Err(_e) => return Err(ReadError::FileNotFound)
    };

    // SYSTEM BLOCK
    seek(&mut save_file, SeekFrom::Start(0x00))?;
    let _rtc_offset = read_i64(&mut save_file)?;
    let mut _mac_address = vec![0u8; 6];
    &save_file.read_exact(&mut _mac_address);

    let _owner_month = read_u8(&mut save_file)?;
    let _owner_date = read_u8(&mut save_file)?;
    let _canary = read_u32(&mut save_file)?;

    let _rtc_year = read_u32(&mut save_file)?;
    let _rtc_month = read_u32(&mut save_file)?;
    let _rtc_date = read_u32(&mut save_file)?;
    let _rtc_weekday = read_u32(&mut save_file)?;

    let _rtc_hour = read_u32(&mut save_file)?;
    let _rtc_minute = read_u32(&mut save_file)?;
    let _rtc_second = read_u32(&mut save_file)?;

    let _day = read_u32(&mut save_file)?;


    // PLAYER BLOCK
    let start_date: DateTime<Utc> = read_date(&mut save_file)?;
    let hof_date: DateTime<Utc> = read_date(&mut save_file)?;

    let _save_penalty = read_u32(&mut save_file)?;
    let _mystery_gift_unlocked = read_u8(&mut save_file)?;

    seek(&mut save_file, SeekFrom::Current(0x03))?; // padding_49

    let _network_id = read_i32(&mut save_file)?;

    seek(&mut save_file, SeekFrom::Current(0x0C))?; // unused_50

    seek(&mut save_file, SeekFrom::Current(PADDING_BETWEEN_ENTRIES))?;

    let _options = read_u16(&mut save_file)?;
    let _opts_frame
        = (_options & 0b0_1111_00_0_0_00_0000) >> 10;
    let _opts_button_mode
        = (_options & 0b0_0000_11_0_0_00_0000) >> 8;
    let _opts_battle_scene
        = (_options & 0b0_0000_00_1_0_00_0000) >> 7;
    let _opts_battle_style
        = (_options & 0b0_0000_00_0_1_00_0000) >> 6;
    let _opts_sound_mode
        = (_options & 0b0_0000_00_0_0_11_0000) >> 4;
    let _opts_text_speed
        = _options & 0b0_0000_00_0_0_00_1111;

    seek(&mut save_file, SeekFrom::Current(0x02))?; // padding_02
    
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
    let mut base_save = SaveFile::new(trainer.clone(), trainer_money);

    let _appearance = read_u8(&mut save_file)?;
    let _game_code = read_u8(&mut save_file)?; // @todo: can we assert this is valid?
    let _postgame_flags = read_u8(&mut save_file)?; // isMainStoryCleared, hasNationalDex

    let playtime = (read_u16(&mut save_file)? as u32 * 3660) + (read_u8(&mut save_file)? as u32 * 60) + (read_u8(&mut save_file)? as u32);
    println!("{:?}", playtime);

    seek(&mut save_file, SeekFrom::Current(0x02))?; // padding_04

    seek(&mut save_file, SeekFrom::Current(PADDING_BETWEEN_ENTRIES))?;

    // PARTY BLOCK
    let _max_party_count = read_u8(&mut save_file)?;
    seek(&mut save_file, SeekFrom::Current(0x03))?;
    let _number_in_party = read_u8(&mut save_file)?;
    seek(&mut save_file, SeekFrom::Current(0x03))?;

    seek(&mut save_file, SeekFrom::Start(0xA0))?;
    for _i in 0..6 {
        let mut buf = vec![0u8; 236];
        &save_file.read_exact(&mut buf);

        let mut decrypted_blob = Cursor::new(decrypt_pokemon_blob(buf.clone())?);
        seek(&mut decrypted_blob, SeekFrom::Start(0x08))?;
        let species = read_u16(&mut decrypted_blob)?;
        if species == 0 {
            continue;
        }
        
        let species = Species::from(species);
        let held_item = read_u16(&mut decrypted_blob)?;
        let original_trainer_id = read_u16(&mut decrypted_blob)?;
        let original_secret_id = read_u16(&mut decrypted_blob)?;
        let mut pkmn = Pokemon::new(species);
        
        seek(&mut decrypted_blob, SeekFrom::Start(0x48))?;
        let pokemon_name = read_string(&mut decrypted_blob, 20)?;
        pkmn.set_name(pokemon_name);
        
        if original_trainer_id == trainer_id && original_secret_id == trainer_secret_id {
            pkmn.set_trainer(trainer.clone());
        }
        
        base_save.party.push(pkmn);
    }

    // BAG BLOCK
    seek(&mut save_file, SeekFrom::Start(0x00630))?;
    for _i in 0..(165+50+100+40+64+15+13+12) {
        // @todo: skip offsets when reaching a None item
        let item_id = read_u16(&mut save_file)?;
        let qty = read_u16(&mut save_file)?;

        let item = DPPTItem::from(item_id);
        base_save.add_item(item, qty);
    }

    seek(&mut save_file, SeekFrom::Start(0x0CF2C))?;
    println!("current_box: {:?}", read_u32(&mut save_file)?); // current_box?
    let mut boxes: Vec<crate::save::save::Box> = Vec::with_capacity(18);
    for _i in 0..18 {
        let mut pkmn_box = crate::save::save::Box::new(30);
        for _j in 0..30 {
            let mut buf = vec![0u8; 136];
            &save_file.read_exact(&mut buf);

            let mut decrypted_blob = Cursor::new(decrypt_pokemon_blob(buf.clone())?);
            seek(&mut decrypted_blob, SeekFrom::Start(0x08))?;
            let species = read_u16(&mut decrypted_blob)?;
            if species == 0 {
                continue;
            }

            let species = Species::from(species);
            let held_item = read_u16(&mut decrypted_blob)?;
            let original_trainer_id = read_u16(&mut decrypted_blob)?;
            let original_secret_id = read_u16(&mut decrypted_blob)?;
            let mut pkmn = Pokemon::new(species);

            seek(&mut decrypted_blob, SeekFrom::Start(0x48))?;
            let pokemon_name = read_string(&mut decrypted_blob, 20)?;
            pkmn.set_name(pokemon_name);

            if original_trainer_id == trainer_id && original_secret_id == trainer_secret_id {
                pkmn.set_trainer(trainer.clone());
            }

            pkmn_box.set_pkmn(_j, pkmn);
        }
        boxes.push(pkmn_box);
    }

    // box names
    for i in 0..boxes.len() {
        let current_box: &mut crate::save::save::Box = boxes.get_mut(i).unwrap();
        (*current_box).set_name(read_string(&mut save_file, 20)?);
    }

    // box wallpapers
    for i in 0..boxes.len() {
        let current_box: &mut crate::save::save::Box = boxes.get_mut(i).unwrap();
        (*current_box).set_wallpaper(read_u8(&mut save_file)?);
    }

    base_save.boxes = boxes;

    Ok(Gen4Save {
        save_started: start_date,
        hall_of_fame_entered: hof_date,
        base: base_save,
        locale,
        badges: badges.0,
    })
}

fn read_u8(readable: &mut impl io::Read) -> Result<u8, ReadError> {
    readable.read_u8().map_err(|_| ReadError::Generic)
}

fn read_u16(readable: &mut impl io::Read) -> Result<u16, ReadError> {
    readable.read_u16::<LittleEndian>().map_err(|_| ReadError::Generic)
}

fn read_u32(readable: &mut impl io::Read) -> Result<u32, ReadError> {
    readable.read_u32::<LittleEndian>().map_err(|_| ReadError::Generic)
}

fn read_i64(readable: &mut impl io::Read) -> Result<i64, ReadError> {
    readable.read_i64::<LittleEndian>().map_err(|_| ReadError::Generic)
}

fn read_i32(readable: &mut impl io::Read) -> Result<i32, ReadError> {
    readable.read_i32::<LittleEndian>().map_err(|_| ReadError::Generic)
}

fn read_string(readable: &mut impl io::Read, length: usize) -> Result<String, ReadError> {
    let mut vec: Vec<u16> = Vec::with_capacity(length);
    for _i in 0..length {
        vec.push(read_u16(readable)?);
    }

    Ok(String::from(Gen4StringVector(vec)))
}

fn decrypt_pokemon_blob(blob: Vec<u8>) -> Result<Vec<u8>, ReadError> {
    let blob_len = blob.len();
    let num_words = (blob_len - 8) / 2;
    let is_party = blob_len == 236;

    let mut cursor = Cursor::new(blob);

    let pv = read_u32(&mut cursor)?;
    let flags = read_u16(&mut cursor)?;
    let checksum = read_u16(&mut cursor)?;
    let shift = (pv >> 13) & 31;

    let mut decrypted_blob: Vec<u16> = Vec::with_capacity(num_words);

    let mut prng: u32 = checksum as u32;
    for _i in 0..num_words {
        prng = u32::wrapping_mul(0x41C64E6D, prng) + 0x00006073;
        let mut xor: u16 = (prng >> 16) as u16;

        #[cfg(target_endian = "big")]
        {
            xor = xor.to_le();
        }

        let word = read_u16(&mut cursor)?;
        decrypted_blob.push(word ^ xor);
    }

    // now shuffle
    let num_blocks = 4;
    let idx = shift * 4;
    let block_positions: [u8; 128] = [
        0, 1, 2, 3,
        0, 1, 3, 2,
        0, 2, 1, 3,
        0, 3, 1, 2,
        0, 2, 3, 1,
        0, 3, 2, 1,
        1, 0, 2, 3,
        1, 0, 3, 2,
        2, 0, 1, 3,
        3, 0, 1, 2,
        2, 0, 3, 1,
        3, 0, 2, 1,
        1, 2, 0, 3,
        1, 3, 0, 2,
        2, 1, 0, 3,
        3, 1, 0, 2,
        2, 3, 0, 1,
        3, 2, 0, 1,
        1, 2, 3, 0,
        1, 3, 2, 0,
        2, 1, 3, 0,
        3, 1, 2, 0,
        2, 3, 1, 0,
        3, 2, 1, 0,

        // duplicates of 0-7 to eliminate modulus
        0, 1, 2, 3,
        0, 1, 3, 2,
        0, 2, 1, 3,
        0, 3, 1, 2,
        0, 2, 3, 1,
        0, 3, 2, 1,
        1, 0, 2, 3,
        1, 0, 3, 2,
    ];

    let res: Vec<u8> = Vec::with_capacity(blob_len);
    let mut res_cursor = Cursor::new(res);
    res_cursor.write_u32::<LittleEndian>(pv).map_err(|_| ReadError::Generic)?;
    res_cursor.write_u16::<LittleEndian>(flags).map_err(|_| ReadError::Generic)?;
    res_cursor.write_u16::<LittleEndian>(checksum).map_err(|_| ReadError::Generic)?;

    let start_pos: usize = 0;
    for i in 0..num_blocks {
        let src_idx = start_pos + (16 * block_positions[(idx + i) as usize]) as usize;
        let src_blob: &[u16] = &decrypted_blob[src_idx..(src_idx + 16)];
        src_blob.iter().for_each(|&x| res_cursor.write_u16::<LittleEndian>(x).unwrap());
    }

    if is_party {
        let party_blob = &decrypted_blob[64..];
        party_blob.iter().for_each(|&x| res_cursor.write_u16::<LittleEndian>(x).unwrap());
    }
    
    crate::save::data::dppt::enums::Vars::VAR_AMITY_SQUARE_GIFT_ID;

    Ok(res_cursor.into_inner())
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