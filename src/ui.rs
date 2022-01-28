use super::*;

pub fn build_ui(app: &Application) {
    /*let row = ActionRow::builder()
        .activatable(true)
        .selectable(false)
        .title("Click me")
        .build();
    row.connect_activated(|_| {
        eprintln!("Clicked!");
    });

    let list = ListBox::builder()
        .margin_top(32)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        // the content class makes the list look nicer
        .css_classes(vec![String::from("content")])
        .build();
    list.append(&row);*/

    // Adwaitas' ApplicationWindow does not include a HeaderBar
    let header_bar = HeaderBar::builder()
        .title_widget(&adw::WindowTitle::new("Covidpass", ""))
        .build();

    let leaflet = adw::Leaflet::new();

    let toast_overlay = adw::ToastOverlay::new();
    toast_overlay.set_child(Some(&leaflet));

    let button = Button::builder()
        .label("+")
        .margin_top(4)
        .margin_bottom(4)
        .margin_start(4)
        .margin_end(4)
        .build();

    // Combine the content in a box
    let vbox = gtk::Box::new(Orientation::Vertical, 0);
    vbox.append(&header_bar);
    vbox.append(&toast_overlay);
    vbox.append(&button);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Covidpass")
        .content(&vbox)
        .build();

    // let path = std::path::Path::new("../covCert.txt");
    // cert::load_certificate(path, &leaflet);

    connect_ui(leaflet, toast_overlay, button);

    // Present window
    window.present();
}

fn connect_ui(leaflet: adw::Leaflet, toast_overlay: adw::ToastOverlay, button: gtk::Button) {
    let file_chooser = gtk::FileChooserNative::new(
        None,                         // file
        Some(&gtk::Window::new()),    // parent
        gtk::FileChooserAction::Open, // action
        None,                         // accept_label
        None,                         // cancel_label
    );

    // Connect to response of the file chooser
    file_chooser.connect_response(move |file_chooser, response| {
        handle_file_chooser_response(file_chooser, response, &leaflet, &toast_overlay);
        file_chooser.hide();
    });

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        file_chooser.show();
    });
}

fn handle_file_chooser_response(
    file_chooser: &gtk::FileChooserNative,
    response: gtk::ResponseType,
    leaflet: &adw::Leaflet,
    toast_overlay: &adw::ToastOverlay,
) {
    if response == gtk::ResponseType::Accept {
        if let Some(file) = file_chooser.file() {
            if let Some(path) = file.path() {
                if let Ok(((first_name, surname, full_name), cert)) = cert::load_certificate(&path)
                {
                    add_qr_png(&first_name, &full_name, &leaflet);
                    throw_toast(ToastType::Success(first_name), toast_overlay);
                } else {
                    throw_toast(ToastType::CertInvalid, toast_overlay);
                }
            } else {
                throw_toast(ToastType::FileInvalid, toast_overlay);
            }
        } else {
            throw_toast(ToastType::FileInvalid, toast_overlay);
        }
    } else {
        throw_toast(ToastType::Aborted, toast_overlay);
    }
}

enum ToastType {
    Success(String),
    FileInvalid,
    CertInvalid,
    QrPNGInvalid,
    Aborted,
}

fn throw_toast(toast_type: ToastType, toast_overlay: &adw::ToastOverlay) {
    let toast = match toast_type {
        ToastType::Success(first_name) => {
            let mut success_text = "Certificate for ".to_owned();
            success_text.push_str(&first_name);
            success_text.push_str(" was added");
            adw::Toast::new(&success_text)
        }
        ToastType::FileInvalid => {
            adw::Toast::new("Selected path is invalid. Adding certificate failed!")
        }
        ToastType::CertInvalid => {
            adw::Toast::new("File does not contain a valid certificate. Adding certificate failed!")
        }
        ToastType::QrPNGInvalid => {
            adw::Toast::new("File does not contain valid QR code. Adding certificate failed!")
        }
        ToastType::Aborted => adw::Toast::new("No certificate was added!"),
    };
    toast_overlay.add_toast(&toast);
}

fn add_qr_png(givenname: &str, full_name: &str, leaflet: &adw::Leaflet) {
    let qr_png = Image::from_file("/tmp/qrcode.png");
    qr_png.set_vexpand(true);
    qr_png.set_hexpand(true);
    qr_png.set_margin_top(4);
    qr_png.set_margin_bottom(4);
    qr_png.set_margin_start(4);
    qr_png.set_margin_end(4);

    let label_short_name = gtk::Label::new(Some(&givenname));
    let label_full_name = gtk::Label::new(Some(&full_name));

    label_short_name.set_hexpand(true);
    label_full_name.set_hexpand(true);

    let squeezer = adw::Squeezer::new();
    squeezer.add(&label_full_name);
    squeezer.add(&label_short_name);

    let vbox_cert = gtk::Box::new(Orientation::Vertical, 0);
    vbox_cert.append(&qr_png);
    vbox_cert.append(&squeezer);

    leaflet.append(&vbox_cert);
}
