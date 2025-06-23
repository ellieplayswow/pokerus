pub mod item;

pub mod enums {
    pokerus_macro::metang_enum!("./metafiles/gen4/platinum_vars.txt", u16, Vars);
}