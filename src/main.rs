#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::PathBuf;

use argon2::{Argon2, PasswordVerifier};
use notes_app::app::*;

fn main() {
    // directory_management::initialisation();
    // application_window::new_session();

    // -----------------------------

    // let x = authentication::generate_new("password".to_string());
    // println!("{}", &x);

    // let filex = std::fs::File::create(PathBuf::from("hash.txt")).expect("Unable to create file");
    // std::fs::write(PathBuf::from("hash.txt"), &x).expect("Unable to write to file");

    // let xfromfile = std::fs::read_to_string(PathBuf::from("hash.txt")).expect("Unable to read file");
    

    // let y = argon2::PasswordHash::new(&xfromfile).expect("Unable to parse hash");
    // println!("{:?}", y);

    // let z = Argon2::default().verify_password("password".as_bytes(), &y).is_ok();
    // println!("{}", z);

    // -----------------------------
    let path = PathBuf::from("hash.txt");
    let x = authentication::generate_new("password".to_string());
    std::fs::write(path.clone(), &x).expect("Unable to write to file");
    let y = authentication::authenticate("password".to_string(), path);
    println!("{}", y);


}
