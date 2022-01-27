use gtk4 as gtk;
use libadwaita as adw;

use adw::prelude::*;
use adw::{ActionRow, ApplicationWindow, HeaderBar};
use gtk::{Application, Button, Image, ListBox, Orientation};
use qrcode::{EcLevel, QrCode};
use std::{error::Error, fs::read_to_string};

mod cert;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new application
    let app = Application::builder()
        .application_id("org.covidpass")
        .build();

    // Initialize libadwaita
    app.connect_startup(|_| {
        adw::init();
    });

    // Connect to "activate" signal of `app`
    app.connect_activate(ui::build_ui);

    // Run the application
    app.run();

    Ok(())
}
