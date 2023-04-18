#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod app {
    pub mod application_window;
    pub mod authentication;
    pub mod directory_management;
}

fn main() {

    app::directory_management::initialisation();
    app::application_window::new_session();

}
