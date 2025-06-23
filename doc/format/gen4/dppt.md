# Diamond, Pearl, Platinum (DPPt) save format

## Resources
- **[PKHeX](https://github.com/kwsch/PKHeX)**
- **[pret/pokeplatinum](https://github.com/pret/pokeplatinum) & [pret/pokediamond](https://github.com/pret/pokediamond)**
- **[Project Pokémon](https://projectpokemon.org/home/docs/gen-4/dp-save-structure-r74/)**


_note: HGSS is very similar, but i haven't gotten there yet_

The DPPt `.sav` file is split into 2 saves of `0x40000` in length. This is to allow some native rollback capabilities when
one save gets corrupted. (@todo: how do we select which save?)

**@todo: confirm all offsets & lengths**

Each save has 2 blocks: the **general / normal** ("small") block, and the **storage / boxes** ("big") block.

**NOTE:** data is aligned to the nearest word / int boundary, which means sometimes there is padding. These have been noted in the tables.

## General / Normal block

The general block is split into a number of entries, each one containing a group of data. Between each entry is `0x08` bytes of padding.

### System Data

**Purpose**: contain system & RTC data

**Offset**: 0x00

**Length**: 0x5C

| Offset | Length (bytes) | Type    | Contents                  | Notes                                                                          | Example |
|--------|----------------|---------|---------------------------|--------------------------------------------------------------------------------|---------|
| 0x00   | 8              | `i64`   | RTC Offset                |                                                                                |         |
| 0x08   | 6              | `u8[6]` | DS MAC Address            | This is always set, even if you have never used networking                     |         |
| 0x0E   | 1              | `u8`    | DS Profile birthday month |                                                                                |         |
| 0x0F   | 1              | `u8`    | DS Profile birthday date  |                                                                                |         |
| 0x10   | 1              | `bool`  | Canary                    | This is always set to 1                                                        |         |
| 0x11   | 3              |         | **Padding**               |                                                                                |         |
| 0x14   | 4              | `u32`   | RTC Year                  |                                                                                |         |
| 0x18   | 4              | `u32`   | RTC Month                 |                                                                                |         |
| 0x1C   | 4              | `u32`   | RTC Date                  |                                                                                |         |
| 0x20   | 4              | `u32`   | RTC Weekday               |                                                                                |         |
| 0x24   | 4              | `u32`   | RTC Hour                  |                                                                                |         |
| 0x28   | 4              | `u32`   | RTC Minute                |                                                                                |         |
| 0x2C   | 4              | `u32`   | RTC Second                |                                                                                |         |
| 0x30   | 4              | `u32`   | "Day"                     | _This might be the days since Nitro Epoch_                                     |         |
| 0x34   | 8              | `i64`   | Start timestamp           | Number of seconds since `2000-01-01T00:00:00Z`                                 |         |
| 0x3C   | 8              | `i64`   | First HOF timestamp       | Number of seconds since `2000-01-01T00:00:00Z`                                 |         |
| 0x44   | 4              | `u32`   | Clock change penalty      | This is a number of minutes set when you load your save and the RTC mismatches |         |
| 0x48   | 1              | `bool`  | Mystery Gift unlocked     |                                                                                |         |
| 0x49   | 3              |         | **Padding**               |                                                                                |         |
| 0x4C   | 4              | `i32`   | Network profile ID        | This is only set after an initial connection to DS WiFi Communications         |         |
| 0x50   | 12             |         | **Padding**               |                                                                                |         |

### Player Data

**Purpose**: Store a high level overview of the player

**Offset**: 0x64

**Length**: 0x2C

| Offset | Length (bytes) | Type     | Contents           | Notes                | Example |
|--------|----------------|----------|--------------------|----------------------|---------|
| 0x00   | 2              | `u16`    | Options            | Stored as a bitfield |         |
| 0x02   | 2              |          | **Padding**        |                      |         |
| 0x04   | 16             | `string` | Trainer name       |                      |         |
| 0x14   | 2              | `u16`    | Trainer ID         |                      |         |
| 0x16   | 2              | `u16`    | Secret ID          |                      |         |
| 0x18   | 4              | `u32`    | Money              | Capped at 999,999    |         |
| 0x1C   | 1              | `u8`     | Gender             | 0 = Male, 1 = Female |         |
| 0x1D   | 1              | `u8`     | Cartridge Locale   |                      |         |
| 0x1E   | 1              | `u8`     | Badges             | Stored as a bitfield |         |
| 0x1F   | 1              | `u8`     | Multiplayer Avatar |                      |         |
| 0x20   | 1              | `u8`     | Game Version       |                      |         |
| 0x21   | 1              | `u8`     | Postgame Flags     | Stored as a bitfield |         |
| 0x22   | 2              |          | **Padding**        |                      |         |
| 0x24   | 2              | `u16`    | Game corner coins  |                      |         |
| 0x26   | 2              | `u16`    | Hours played       |                      |         |
| 0x28   | 1              | `u8`     | Minutes played     |                      |         |
| 0x29   | 1              | `u8`     | Seconds Played     |                      |         |
| 0x2A   | 2              |          | **Padding**        |                      |         |

### Party Data

**Purpose**: store information on your party

**Offset**: 0x98

**Length**: 0x590

| Offset | Length (bytes) | Type      | Contents               | Notes                                                            | Example |
|--------|----------------|-----------|------------------------|------------------------------------------------------------------|---------|
| 0x00   | 1              | `u8`      | Maximum Capacity       | This is always 6                                                 |         |
| 0x01   | 3              |           | **Padding**            |                                                                  |         |
| 0x04   | 1              | `u8`      | Number in Party        |                                                                  |         |
| 0x05   | 3              |           | **Padding**            |                                                                  |         |
| 0x08   | 1416           | `pkmn[6]` | Party Pokémon          | Party Pokémon include battle stats with takes an extra 100 bytes |         |

#### Pokémon

@todo ...

### Bag Data

**Purpose**: store information on all bag items

**Offset**: 0x630

**Length**: 0x774

| Offset | Length (bytes) | Type        | Contents           | Notes | Example |
|--------|----------------|-------------|--------------------|-------|---------|
| 0x000  | 660            | `item[165]` | Item pocket        |       |         |
| 0x294  | 200            | `item[50]`  | Key Item pocket    |       |         |
| 0x35C  | 400            | `item[100]` | TM/HM pocket       |       |         |
| 0x4EC  | 48             | `item[12]`  | Mail pocket        |       |         |
| 0x51C  | 160            | `item[40]`  | Medicine pocket    |       |         |
| 0x5BC  | 256            | `item[64]`  | Berry pocket       |       |         |
| 0x6BC  | 60             | `item[15]`  | Pokeball pocket    |       |         |
| 0x6F8  | 120            | `item[30]`  | Battle item pocket |       |         |
| 0x770  | 4              | `u32`       | Registered item    |       |         |


#### Items

Items are stored packed as an item ID and a quantity:

| Offset | Length (bytes) | Type   | Contents |
|--------|----------------|--------|----------|
| 0x00   | 2              | `u16`  | Item ID  |
| 0x02   | 2              | `u16`  | Quantity |

### Vars & Flags Data

**Purpose**: general purpose blob

**Offset**: 0xDAC

**Length**: 0x3AC

| Offset | Length (bytes) | Type          | Contents | Notes | Example |
|--------|----------------|---------------|----------|-------|---------|
| 0x000  | 576            | `u16[288]`    | Vars     |       |         |
| 0x240  | 364            | `bool[2912]`  | Flags    |       |         |

#### Vars

Some interesting vars:
- `var[48]`, offset `0x60`: VAR_PLAYER_STARTER (absolute offset `0xE0C`)
- `var[62]` & `var[63]`, offset `0x7C` & `0x7E`: VAR_LOTTERY_TRAINER_ID_LOW_HALF, VAR_LOTTERY_TRAINER_ID_HIGH_HALF
(see vars.txt for the exhaustive list)

#### Flags

Flags are stored as `u8[364]`, where each `u8` is itself a bitfield of 8 `bool`s. To get a specific flag, in pseudocode:
```
flag_id := 292 # example
byte_offset := floor(flag_id / 8) # 36

bit_offset := flag_id % 8 # 4
flag_value := ((1 << bit_offset) & memory[byte_offset]) > 0
```

(see flags.txt for the exhaustive list)

### Poketch Data

**Purpose**: all data relating to the Poketch and its apps

**Offset**: 0x1160

**Length**: 0x0118

| Offset | Length (bytes) | Type               | Contents                  | Notes                                                              | Example |
|--------|----------------|--------------------|---------------------------|--------------------------------------------------------------------|---------|
| 0x00   | 1              | `u8`               | Start Flags               | enabled, pedometer enabled, dotart modified, screen palette colour |         |
| 0x01   | 1              | `i8`               | Number of registered apps |                                                                    |         |
| 0x02   | 1              | `i8`               | Currently selected app    |                                                                    |         |
| 0x03   | 25             | `u8[25]`           | Apps unlocked             | Internally stored as `u8[32]`, but last 7 unused                   |         |
| 0x1C   | 8              |                    | **Padding**               |                                                                    |         |
| 0x24   | 4              | `u32`              | Step count                |                                                                    |         |
| 0x28   | 2              | `u16`              | Alarm flags               | set, hour (5 bits), minute (6 bits)                                |         |
| 0x2A   | 120            | `u8[120]`          | Dotart pixels             |                                                                    |         |
| 0xA2   | 2              |                    | **Padding**               |                                                                    |         |
| 0xA4   | 4              | `u32`              | Calendar mark bitmap      |                                                                    |         |
| 0xA8   | 1              | `u8`               | Calendar month            | Defaults to 1, even when not unlocked                              |         |
| 0xA9   | 12             | `map_marker[6]`    | Map markers               |                                                                    |         |
| 0xB5   | 3              |                    | **Padding**               |                                                                    |         |
| 0xB8   | 96             | `pkmn_history[12]` | Pokémon history           |                                                                    |         |

#### map_marker

| Offset | Length (bytes) | Type | Contents |
|--------|----------------|------|----------|
| 0x00   | 1              | `u8` | x pos    |
| 0x01   | 1              | `u8` | y pos    |

#### pkmn_history

**NOTE**: i _think_ icon & form are always `0` in the save data

| Offset | Length (bytes) | Type  | Contents |
|--------|----------------|-------|----------|
| 0x00   | 2              | `u16` | Species  |
| 0x02   | 2              | `u16` | Icon     |
| 0x04   | 4              | `u32` | Form     |

### Overworld Player State Data

@todo

### Pokedex Data

@todo

### Daycare Data

@todo

### Pal Pad Data

@todo

### Misc Data

@todo

### Overworld Player Save Data

@todo

### Underground Data

@todo

### Regulation Battles Data

@todo

### Image Clip Data

@todo

### Mailbox Data

@todo

### Poffins Data

@todo

### Record Mixed RNG Data

@todo

### Journal Data

@todo

### Trainer Card Data

@todo

### Game Records Data

@todo

### Ball Seals Data

@todo

### Chatot Cry Data

@todo

### Battle Frontier Data

@todo

### Ribbons Data

@todo

### Special Encounter Data

@todo

### GTS Data

@todo

### TV Broadcast Data

@todo

### Rankings Data

@todo

### WiFi List Data

@todo

### WiFi History Data

@todo

### Mystery Gift Data

@todo

### Pal Park Transfer Data

@todo

### Contest Data

@todo

### Sentence Data

@todo

### Email Data

@todo

### WiFi Question Data

@todo

## Storage / Boxes Data

The storage block contains information regarding your boxes. In all generation 4 games, you have 18 boxes each with 30 Pokémon.

| Offset  | Length (bytes) | Type         | Contents                | Notes                     | Example |
|---------|----------------|--------------|-------------------------|---------------------------|---------|
| 0x00    | 4              | `u32`        | Last selected box index |                           |         |
| 0x04    | 73,440         | `box[18]`    | Box Pokémon             | Each Pokémon is 136 bytes |         |
| 0x11EE4 | 720            | `string[18]` | Box names               |                           |         |
| 0x121B4 | 18             | `u8[18]`     | Box wallpapers          |                           |         |

## Footer

The last 0x14 bytes of every General & Storage block is its footer.

| Offset | Length (bytes) | Type  | Contents              | Notes                | Example |
|--------|----------------|-------|-----------------------|----------------------|---------|
| 0x00   | 4              | `u32` | Block link ID         |                      |         |
| 0x04   | 4              | `u32` | Save ID               |                      |         |
| 0x08   | 4              | `u32` | Size of block         | Including footer     |         |
| 0x0C   | 4              |       | Sector Signature      | Always `23 06 06 20` |         |
| 0x10   | 2              |       | **Padding** @todo     |                      |         |
| 0x12   | 2              | `u16` | CRC-16-CCITT checksum |                      |         |
