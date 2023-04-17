use glob::{glob};
use std::env::set_current_dir;
use std::fs::{create_dir, File};
use std::path::PathBuf;

pub fn check_dir(path: &PathBuf) -> Vec<PathBuf> {

    // todo: return all valid entries as a vector of pathbufs (this will pos. be used to populate the sidebar)

    let mut valid_entries = Vec::new();

    set_current_dir(path).expect("Failed to set current directory");

    for entry in glob("/Notes/").expect("Glob failed").filter_map(Result::ok) {
        valid_entries.push(entry);
    }

    valid_entries
}

pub fn initialisation() -> PathBuf {

    // check if the data folder exists, if not create it etc etc

    let mut dir: PathBuf = directories::UserDirs::new()
    .expect("Failed to get user directories")
    .document_dir()
    .expect("Failed to get document directory")
    .to_path_buf();
    
    dir.push("Notes");

    if !dir.exists() {
        create_dir(&dir).expect("Failed to create directory, check permissions or parents");
    }

    let mut pass_file = dir.clone();
    pass_file.push("password.txt");
    if !pass_file.exists() {
        File::create(pass_file).expect("Error creating password file");
    }

    dir
    
}
