use super::*;

pub struct StartPage {
    content: gtk::Box,
    leaflet: adw::Leaflet,
    button: gtk::Button,
    toast_overlay: adw::ToastOverlay,
    file_chooser: gtk::FileChooserNative,
    view_stack: adw::ViewStack,
}

impl StartPage {
    pub fn new(view_stack: &adw::ViewStack, toast_overlay: adw::ToastOverlay) -> Self {
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
            leaflet: leaflet.clone(),
            button: button.clone(),
            toast_overlay: toast_overlay.clone(),
            file_chooser: file_chooser.clone(),
            view_stack: view_stack.clone(),
        };
        start_page.connect();
        start_page
    }

    fn connect(&self) {
        let leaflet = self.leaflet.clone();
        let toast_overlay = self.toast_overlay.clone();
        let button = self.button.clone();
        let file_chooser = self.file_chooser.clone();
        let view_stack = self.view_stack.clone();

        // Connect to response of the file chooser
        file_chooser.connect_response(move |file_chooser, response| {
            handle_file_chooser_response(
                file_chooser,
                response,
                &leaflet,
                &toast_overlay,
                &view_stack,
            );
            file_chooser.hide();
        });

        // Connect to "clicked" signal of `button`
        button.connect_clicked(move |_| {
            file_chooser.show();
        });
    }

    pub fn content(&self) -> &gtk::Box {
        &self.content
    }
}

fn handle_file_chooser_response(
    file_chooser: &gtk::FileChooserNative,
    response: gtk::ResponseType,
    leaflet: &adw::Leaflet,
    toast_overlay: &adw::ToastOverlay,
    view_stack: &adw::ViewStack,
) {
    if response == gtk::ResponseType::Accept {
        if let Some(file) = file_chooser.file() {
            if let Some(path) = file.path() {
                if let Ok(((first_name, surname, full_name), cert)) = cert::load_certificate(&path)
                {
                    add_qr_png(&first_name, &full_name, &leaflet, view_stack);
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

fn add_qr_png(
    givenname: &str,
    full_name: &str,
    leaflet: &adw::Leaflet,
    view_stack: &adw::ViewStack,
) {
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

    let button_qr = gtk::Button::new();
    button_qr.set_child(Some(&vbox_cert));

    leaflet.append(&button_qr);

    // Connect to "clicked" signal of `button`
    let view_stack = view_stack.clone();
    let givenname_clone: String = givenname.into();
    button_qr.connect_clicked(move |_| {
        println!("Pushed qr for {}", givenname_clone);
        println!(
            "Currently visible: {}",
            view_stack.visible_child_name().unwrap()
        );
        view_stack.set_visible_child_name("detail_page");
    });
}
