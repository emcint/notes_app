#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::PathBuf;

use argon2::{Argon2, PasswordVerifier};
use notes_app::app::*;

fn main() {

    directory_management::initialisation();
    application_window::new_session();

}
