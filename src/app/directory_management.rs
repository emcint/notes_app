use glob::{glob, GlobError};
use std::env::set_current_dir;
use std::path::PathBuf;

pub fn check_dir(path: &PathBuf) -> Vec<PathBuf> {

    // todo: return all valid entries as a vector of pathbufs (this will pos. be used to populate the sidebar)

    let mut valid_entries = Vec::new();

    set_current_dir(path).expect("Failed to set current directory");

    for entry in glob("/Notes/").expect("Glob failed").filter_map(Result::ok) {
        valid_entries.push(entry);
    }

    return valid_entries;
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
        std::fs::create_dir(&dir).expect("Failed to create directory, check permissions or parents");
    }

    return dir;
    
}
