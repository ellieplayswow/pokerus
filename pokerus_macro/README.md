# pokerus_macro

This is an internal crate for a set of procedural macros for pokerus. I'm still learning this :)

## Macros

### metang_enum!

```rust
metang_enum!("./enum.txt", repr_type);
```

Generates an enum from a [metang](https://github.com/lhearachel/metang) definition file, which is used a lot in the
`pret/poke*` projects.

The idea is to reduce the amount of code that needs to be written per gen/series, specifically with vars/flags which,
with this approach, can be accessed a lot easier:
```rust
// can we get starter pokemon?
let var_idx: u16 = pokerus::save::data::dppt::enums::Vars::VAR_PLAYER_STARTER.into();

seek(&mut save_file, SeekFrom::Start(0xDAC))?; // 0xDAC is where vars start
seek(&mut save_file, SeekFrom::Current(((var_idx - 16384) * 2) as i64))?; // temporary, vars start at idx 16384
println!("starter: {:?}", Species::from(read_u16(&mut save_file)?));
```

outputs `starter: Chimchar`. that's pretty neat!

Eventually, I will update the save file system so that it's automagically loaded like...
```rust
use pokerus::save::data::dppt::enums::Vars;
let read = read_save("test-files/platinum.sav");
println!("starter: {:?}", read.get_var(Vars::VAR_PLAYER_STARTER));
read.set_var(Vars::VAR_PLAYER_STARTER, Species::Piplup.into());
```