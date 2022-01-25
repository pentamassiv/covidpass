use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Image, Orientation};
use gtk4 as gtk;
use gtk4::Button;
use qrcode::{EcLevel, QrCode};
use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
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
    let vbox = gtk::Box::new(Orientation::Vertical, 0);

    // Create a button with label and margins
    let button = Button::builder()
        .label("+")
        .margin_top(4)
        .margin_bottom(4)
        .margin_start(4)
        .margin_end(4)
        .build();

    vbox.append(&button);
    let path = "../covCert.txt";
    load_certificates(path, &vbox);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Covidpass")
        .child(&vbox)
        .build();

    let file_chooser = gtk::FileChooserNative::new(
        None,                         // file
        Some(&gtk::Window::new()),    // parent
        gtk::FileChooserAction::Open, // action
        None,                         // accept_label
        None,                         // cancel_label
    );

    // Connect to response of the file chooser
    file_chooser.connect_response(|file_chooser, response| {
        if response == gtk4::ResponseType::Accept {
            let filename = file_chooser.file().unwrap().path();
            println!("{:?}", filename);
        }
        println!("{:?}", response);
        file_chooser.hide();
    });

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        add_certificate(&file_chooser);
    });

    // Present window
    window.present();
}

fn add_certificate(file_chooser: &gtk::FileChooserNative) -> Result<(), Box<dyn Error>> {
    file_chooser.show();
    Ok(())
}

fn load_certificates(path: &str, vbox: &gtk::Box) {
    let _buf_str = read_to_string(path).unwrap();
    #[cfg(feature = "crates")]
    {
        let health_cert = greenpass_crates::parse(&_buf_str).unwrap();
        println!("{:#?}", health_cert);
    }

    #[cfg(feature = "local")]
    {
        let health_cert = greenpass::parse(&_buf_str).unwrap();
        println!("{:#?}", health_cert);
    }

    let qr_code = QrCode::with_error_correction_level(_buf_str, EcLevel::L).unwrap();
    let image = qr_code.render::<image::Luma<u8>>().build();
    image.save("/tmp/qrcode.png").unwrap();
    let qr_png = Image::from_file("/tmp/qrcode.png");
    qr_png.set_vexpand(true);
    qr_png.set_hexpand(true);
    qr_png.set_margin_top(4);
    qr_png.set_margin_bottom(4);
    qr_png.set_margin_start(4);
    qr_png.set_margin_end(4);
    vbox.prepend(&qr_png);
}
