pub mod parsing {
    use std::collections::HashMap;
    use crate::structs::structs::Entry;

    pub fn parse(toml_data: String) -> HashMap<String, Entry> {
        return toml::from_str(toml_data.as_str()).unwrap();
    }
}