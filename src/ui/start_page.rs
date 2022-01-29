use super::*;

pub struct StartPage {
    persons: usize,
    pub content: gtk::Box,
    leaflet: adw::Leaflet,
    button: gtk::Button,
    toast_overlay: adw::ToastOverlay,
    file_chooser: gtk::FileChooserNative,
}

impl StartPage {
    pub fn new(toast_overlay: adw::ToastOverlay) -> Self {
        let leaflet = adw::Leaflet::new();

        let button = Button::builder()
            .label("+")
            .margin_top(4)
            .margin_bottom(4)
            .margin_start(4)
            .margin_end(4)
            .build();

        // Combine the content in a box
        let content = gtk::Box::new(Orientation::Vertical, 0);
        content.append(&leaflet);
        content.append(&button);

        let file_chooser = gtk::FileChooserNative::new(
            None,                         // file
            Some(&gtk::Window::new()),    // parent
            gtk::FileChooserAction::Open, // action
            None,                         // accept_label
            None,                         // cancel_label
        );

        let start_page = Self {
            content,
            persons: 0,
            leaflet: leaflet.clone(),
            button: button.clone(),
            toast_overlay: toast_overlay.clone(),
            file_chooser: file_chooser.clone(),
        };
        start_page.connect();
        start_page
    }

    fn connect(&self) {
        let leaflet = self.leaflet.clone();
        let toast_overlay = self.toast_overlay.clone();
        let button = self.button.clone();
        let file_chooser = self.file_chooser.clone();

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
                    throw_toast(ToastType::Success(first_name), &toast_overlay);
                } else {
                    throw_toast(ToastType::CertInvalid, &toast_overlay);
                }
            } else {
                throw_toast(ToastType::FileInvalid, &toast_overlay);
            }
        } else {
            throw_toast(ToastType::FileInvalid, &toast_overlay);
        }
    } else {
        throw_toast(ToastType::Aborted, &toast_overlay);
    }
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
