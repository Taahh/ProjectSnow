pub mod structs {
    use std::borrow::Borrow;

    use tide::prelude::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Entry {
        file_directory: Option<String>,
        path_to_php: Option<String>,
        redirect: Option<RedirectEntry>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct RedirectEntry {
        path: String,
        destination: String,
    }

    impl Entry {
        pub fn new(file_directory: Option<String>, path_to_php: Option<String>, redirect: Option<RedirectEntry>) -> Self {
            Entry {
                file_directory, path_to_php, redirect
            }
        }
        pub fn file_directory(&self) -> Option<String> {
           (&self.file_directory).to_owned()
        }

        pub fn php_path(&self) -> Option<String> {
           (&self.path_to_php).to_owned()
        }

        pub fn redirect(&self) -> Option<RedirectEntry> {
            if self.redirect.is_none() {
                return None;
            }
            RedirectEntry::new((&self.redirect.as_ref().unwrap()).path.to_string(), (&self.redirect.as_ref().unwrap()).destination.to_string())
        }
    }

    impl RedirectEntry {
        pub fn new(path: String, destination: String) -> Option<Self> {
            Some(RedirectEntry {
                path, destination
            })
        }
        pub fn path(&self) -> String {
            (&self.path).to_string()
        }

        pub fn destination(&self) -> String {
            (&self.destination).to_string()
        }
    }
}