use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;
use gtk4::Button;
use image::Luma;
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

    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("/tmp/qrcode.png").unwrap();

    // Create a new application
    let app = Application::builder()
        .application_id("org.covidpass")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();

    Ok(())
}

fn build_ui(app: &Application) {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // Create a button with label and margins
    let button = Button::builder()
        .label("+")
        .margin_top(4)
        .margin_bottom(4)
        .margin_start(4)
        .margin_end(4)
        .build();

    /*let file_chooser = gtk::FileChooserNative::new(
        None,                         // file
        Some(&gtk::Window::new()),    // parent
        gtk::FileChooserAction::Open, // action
        None,                         // accept_label
        None,                         // cancel_label
    );
    let filename = file_chooser.file().unwrap().basename();
    file_chooser.show();*/

    vbox.append(&button);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Covidpass")
        .child(&vbox)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        add_certificate(&vbox);
    });

    // Present window
    window.present();
}

fn add_certificate(vbox: &gtk::Box) {
    let qr_png = gtk::Image::from_file("/tmp/qrcode.png");
    qr_png.set_vexpand(true);
    qr_png.set_hexpand(true);
    qr_png.set_margin_top(4);
    qr_png.set_margin_bottom(4);
    qr_png.set_margin_start(4);
    qr_png.set_margin_end(4);
    vbox.prepend(&qr_png);
}
