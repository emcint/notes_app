#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use notes_app::application_window::new_session;

fn main() {
    new_session()
}
