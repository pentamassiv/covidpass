use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;
use gtk4::Button;
use qrcode::render::unicode;
use qrcode::{EcLevel, QrCode};
use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    // Read a Base45 payload extracted from a QR code
    let _buf_str = read_to_string("../covCert.txt")?;
    #[cfg(feature = "crates")]
    {
        let health_cert = greenpass_crates::parse(&_buf_str)?;
        println!("{:#?}", health_cert);
    }

    #[cfg(feature = "local")]
    {
        let health_cert = greenpass::parse(&_buf_str)?;
        println!("{:#?}", health_cert);
    }

    let code = QrCode::with_error_correction_level(_buf_str, EcLevel::L).unwrap();

    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);

    // Create a new application
    let app = Application::builder()
        .application_id("org.covidcert")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();

    Ok(())
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Covid Certificate")
        .child(&button)
        .build();

    // Present window
    window.present();
}
