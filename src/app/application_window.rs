

use eframe::{self, run_native, App, NativeOptions, *};
use egui::{
    CentralPanel, Context, Pos2, Rounding, SidePanel, Style, TopBottomPanel, Ui, Window,
};
use std::fs::*;
use std::io::Write;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;

use crate::app::authentication::*;

use super::authentication;

struct ApplicationWindow {
    window_size: egui::Vec2,
    user_input: String,
    validity: bool,
    attempts: u128,
    show_incorrect: bool,
    path: std::path::PathBuf,
    show_new_folder: bool,
    show_new_note: bool,
    folder_name: String,
    note_name: String,
    show_sync_spinner: bool,
    current_focus: std::path::PathBuf,
    current_file_buffer: String,
    input_cache: Vec<String>,
    show_confirmation_dialogue: bool,
    allowed_to_close: bool,
    show_toolbar: bool,
    dark_mode: bool,
}

impl Default for ApplicationWindow {
    fn default() -> Self {
        Self {
            window_size: egui::Vec2::new(0.0, 0.0),
            user_input: "".to_string(), // used for the login screen
            validity: false,            // determines which state the app is in (login or main)
            attempts: 0, // used to determine if the user has tried to login too many times
            show_incorrect: false, // used to show/hide the incorrect message on the login screen
            path: std::path::PathBuf::from_str("src/data").unwrap(), // used to store the path to the data folder (needs to be changed to a config file)
            show_new_folder: false, // used to show/hide the new folder window
            show_new_note: false,   // used to show/hide the new note window
            folder_name: "".to_string(), // used to store the name of the new folder
            note_name: "".to_string(), // used to store the name of the new note
            show_sync_spinner: false, // used to show/hide the sync spinner
            current_focus: std::path::PathBuf::from(String::from("")), // used to store the path to the current file/note
            current_file_buffer: String::new(), // used to store the contents of the current file/note
            input_cache: Vec::new(),            // todo!
            show_confirmation_dialogue: false, // used to show/hide the exit confirmation dialogue
            allowed_to_close: false, // used to determine if the user has confirmed they want to exit
            show_toolbar: true,      // used to show/hide the toolbar
            dark_mode: true, // used to determine if the app should be in dark mode or not
        }
    }
}

impl ApplicationWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Default::default()
    }
}

impl App for ApplicationWindow {
    fn persist_native_window(&self) -> bool {
        true
    }
    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn on_close_event(&mut self) -> bool {
        if self.validity == true {
            self.show_confirmation_dialogue = true;
        } else {
            self.allowed_to_close = true;
        }

        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.window_size = ctx.screen_rect().size(); // keeps track of the window size

        // let mut style: egui::Style = (*ctx.style()).clone();
        // style.visuals.window_rounding = Rounding::same(20.0);
        // style.visuals.dark_mode = self.dark_mode;
        // ctx.set_style(style); doesnt work idk why

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(1.0);
                ui.horizontal(|ui: &mut Ui| {
                    ui.hyperlink_to("Source code", "github.com");
                    ui.add_space(5.0);
                    ui.label("Version 0.1.0");
                    ui.add_space(50.0);

                    if self.validity {
                        if ui.button("Sync").clicked() {
                            self.show_sync_spinner = true;
                            for i in 1..=3 {
                                println!("Syncing {}", i); // todo: replace with actual sync code
                            }
                        }
                        if self.show_sync_spinner {
                            ui.spinner();
                        }
                    }
                });
                ui.add_space(1.0);
            });
        });

        if self.show_toolbar && self.validity {
            TopBottomPanel::top("toolbar").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New Folder").clicked() {
                            self.show_new_folder = true;
                        }
                        if ui.button("New Note").clicked() {
                            self.show_new_note = true;
                        }
                        if ui.button("Exit").clicked() {
                            // code to check if there for unsaved changes (same code needs to be implemented later - modularise)
                            todo!()
                        }
                    });

                    ui.menu_button("Edit", |ui| {
                        if ui.button("Undo").clicked() {
                            todo!()
                        }

                        if ui.button("Redo").clicked() {
                            todo!()
                        }

                        if ui.button("Select All").clicked() {
                            todo!()
                        }
                    });

                    ui.menu_button("Appearance", |ui| {
                        if ui.button("Hide Toolbar").clicked() {
                            self.show_toolbar = false;
                        }

                        ui.toggle_value(&mut self.dark_mode, "Dark Mode"); // shift to theme based system?
                    });

                    ui.menu_button("Settings", |ui| {
                        if ui.button("Account").clicked() {
                            todo!()
                        }

                        if ui.button("Security").clicked() {
                            todo!()
                        }

                        if ui.button("Perisistance").clicked() {
                            todo!()
                        }
                    });
                });
            });
        }

        if self.validity == false {
            CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui: &mut Ui| {
                    ui.heading("Enter Password:");
                    ui.add_space(5.0);
                    let input_field = ui.text_edit_singleline(&mut self.user_input);
                    if input_field.gained_focus() {
                        self.show_incorrect = false;
                    };
                    ui.add_space(5.0);
                    if input_field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        ui.add_space(1.0);
                        ui.spinner();

                        self.validity = super::authentication::authenticate(self.user_input.clone());

                        if self.validity == false {
                            self.show_incorrect = true;
                            self.attempts += 1;
                        };
                        /*
                        if self.attempts >= 3 {
                            ui.add_space(1.0);
                            ui.label("Too many attempts");

                            frame.close();
                        };
                        */
                    };
                    if self.show_incorrect == true {
                        ui.label("Incorrect");
                        ui.add_space(5.0);
                    };
                    if ui.button("Continue").clicked() {
                        ui.add_space(1.0);
                        ui.spinner();

                        self.validity = super::authentication::authenticate(self.user_input.clone());

                        if self.validity == false {
                            self.show_incorrect = true;
                            self.attempts += 1;
                        };
                        /*
                        if self.attempts >= 3 {
                            ui.add_space(1.0);
                            ui.label("Too many attempts");

                            frame.close();
                        };
                        */
                        ui.add_space(1.0);
                    };
                });
            });
        } else {
            SidePanel::left("left_panel").show(ctx, |ui| {
                ui.make_persistent_id("left_panel");
                self.persist_egui_memory();
                ui.add_space(5.0);
                ui.horizontal(|ui: &mut Ui| {
                    if self.show_new_folder {
                        Window::new("New folder").show(ctx, |ui| {
                            ui.label("Enter folder name:");
                            ui.add_space(5.0);
                            ui.text_edit_singleline(&mut self.folder_name);
                            ui.add_space(5.0);
                            ui.horizontal(|ui: &mut Ui| {
                                if ui.button("Create").clicked() {
                                    ui.spinner();
                                    let mut path = self.path.clone();
                                    path.push(self.folder_name.clone());
                                    match create_dir(&path) {
                                        Ok(_) => println!("Created folder"),
                                        Err(e) => println!("Error: {}", e),
                                    }
                                    self.show_new_folder = false;
                                };
                                if ui.button("Cancel").clicked() {
                                    self.show_new_folder = false;
                                    self.folder_name = "".to_string();
                                };
                            });
                        });
                    };
                    if ui.small_button("📁+").clicked() {
                        self.show_new_folder = true;
                    };

                    if self.show_new_note {
                        Window::new("New File").show(ctx, |ui| {
                            ui.label("Enter file name:");
                            ui.add_space(5.0);
                            ui.text_edit_singleline(&mut self.note_name);
                            ui.add_space(5.0);
                            ui.horizontal(|ui: &mut Ui| {
                                if ui.button("Create").clicked() {
                                    ui.spinner();
                                    let mut path = self.path.clone();
                                    path.push(self.note_name.clone());
                                    match OpenOptions::new()
                                        .write(true)
                                        .create(true)
                                        .open(&path)
                                    {
                                        Ok(_) => println!("Created file"),
                                        Err(e) => println!("Error: {}", e),
                                    }
                                    self.show_new_note = false;
                                };
                                if ui.button("Cancel").clicked() {
                                    self.show_new_note = false;
                                    self.note_name = "".to_string();
                                };
                            });
                        });
                    };
                    if ui.small_button("📄+").clicked() {
                        self.show_new_note = true;
                    };
                });

                ui.vertical(|ui: &mut Ui| {
                    ui.add_space(1.0);
                    ui.separator();

                    let available_notes = read_dir(&self.path).unwrap();
                    for entry in available_notes {
                        let entry = match entry {
                            Ok(entry) => entry,
                            Err(e) => {
                                println!("error1: {}", e);
                                continue;
                            }
                        };

                        let path = entry.path();
                        let file_name = path.file_name().unwrap().to_str().unwrap();
                        let object_icon = ui.button(file_name);

                        if object_icon.clicked() && path.is_file() {
                            self.current_focus = path.clone();
                            self.current_file_buffer =
                                read_to_string(path.clone()).expect("Corrupt path");
                        } else {
                            self.current_focus = path.clone(); //mk
                        }
                    }
                });
            });

            CentralPanel::default().show(ctx, |ui| {
                if self.current_focus == PathBuf::from("") {
                    ui.label("Select a note to edit, or create a new one.");
                } else {
                    /* self.current_file
                    .read_to_string(&mut self.current_file_buffer)
                    .unwrap(); */
                    let focus_box = ui.text_edit_multiline(&mut self.current_file_buffer);

                    ui.vertical_centered(|ui: &mut Ui| {
                        if ui.button("Save").clicked() {
                            ui.spinner();
                            // save the file ethan
                            // set_permissions(self.current_focus, Permissions::set_readonly(&mut self, false) ); // this needs to be moved outside of the update loop
                            let mut file = File::create(self.current_focus.clone()).unwrap();
                            file.write(self.current_file_buffer.clone().as_bytes())
                                .unwrap();
                            // self.validity = false;
                        };
                    });
                }
            });
        };

        if self.show_confirmation_dialogue && self.validity {
            egui::Window::new("Save changes?")
                .movable(false)
                .resizable(false)
                .collapsible(false)
                .title_bar(false)
                .fixed_rect(egui::Rect::from_center_size(
                    Pos2::new(self.window_size.x / 2.0, self.window_size.y / 2.0),
                    egui::Vec2::new(150.0, 150.0),
                ))
                // .current_pos(Pos2::new((self.window_size.x / 2.0), (self.window_size.y / 2.0)))
                .show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        if ui.small_button("Save and quit").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }

                        if ui.small_button("Quit without saving").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }

                        if ui.small_button("Cancel").clicked() {
                            self.allowed_to_close = false;
                            self.show_confirmation_dialogue = false;
                        }
                    });
                });
        }
    }
}

pub fn new_session() {
    let mut options = NativeOptions::default();
    options.always_on_top = false; //example customisation
                                    // options.centered = true;
    options.resizable = true;
    options.min_window_size = Some(egui::Vec2::new(300.0, 400.0));
    options.decorated = true; //should eventually be false with custom bar
                                /* let standard_icon = IconData {
                                    rgba: include_bytes!("assets\\icons8-note-96.rgba").to_vec(),
                                    width: 96,
                                    height: 96,
                                };
                                options.icon_data = Some(standard_icon);*/
    run_native(
        "Glacier",
        options,
        Box::new(|cc| Box::new(ApplicationWindow::new(cc))),
    )
    .expect("Failed to create application window");
}
