use gtk4 as gtk;
use libadwaita as adw;

use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar};
use gtk::{Application, Button, Image, Orientation};
use std::error::Error;

mod cert;
mod pub_keys;
mod ui;

/*

    calendar_entries: FactoryVec<CertificateEntry>,
    certificates: HashMap<(String, String, String), DgcContainer>,
    trust_list: dgc::TrustList,

*/

fn main() -> Result<(), Box<dyn Error>> {
    // pub_keys::fetch_public_keys();

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
