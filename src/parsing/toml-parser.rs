use crate::core_structs::Entry;

pub fn parse(str: String) -> Entry {
    return toml::from_str(&str).unwrap();
}