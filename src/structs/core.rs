use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Entry {
    directory_path: Option<String>
}