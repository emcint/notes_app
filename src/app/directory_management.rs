

use glob::{glob, GlobError};
use std::env::set_current_dir;
use std::path::PathBuf;

pub fn check_path(path: PathBuf) -> bool {
    let dir_found = match set_current_dir(path) {
        Ok(_) => true,
        Err(_) => false,
    };

    return dir_found;
}

pub fn check_dir(path: &PathBuf) -> Vec<PathBuf> {
    // todo: return all valid entries as a vector of pathbufs (this will pos. be used to populate the sidebar)

    let mut valid_entries = Vec::new();

    set_current_dir(path).expect("Failed to set current directory");

    for entry in glob("/Notes/").expect("Glob failed").filter_map(Result::ok) {
        valid_entries.push(entry);
    }

    return valid_entries;
}

pub fn check_initialisation() {
    todo!() // check if the data folder exists, if not create it etc etc
}
