use super::*;

pub mod start_page;

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

    let toast_overlay = adw::ToastOverlay::new();
    let start_page = start_page::StartPage::new(toast_overlay.clone());
    toast_overlay.set_child(Some(&start_page.content));

    // Combine the content in a box
    let vbox = gtk::Box::new(Orientation::Vertical, 0);
    vbox.append(&header_bar);
    vbox.append(&toast_overlay);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Covidpass")
        .content(&vbox)
        .build();

    // Present window
    window.present();
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
