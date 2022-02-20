use adw::prelude::AdwApplicationWindowExt;
use gtk::prelude::{
    BoxExt, ButtonExt, FileChooserExt, FileExt, GtkWindowExt, OrientableExt, WidgetExt,
};
use gtk::Orientation;
use relm4::{
    adw,
    factory::{FactoryPrototype, FactoryVec},
    gtk, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets,
};
use std::{fs::read_to_string, path::PathBuf};

mod cert;
mod pub_keys;
mod qr_code;
mod read_ops;

#[derive(Debug)]
struct CertificateEntry {
    firstname: String,
    full_name: String,
    certificate: String,
    signature_validity: dgc::SignatureValidity,
}

#[derive(Debug)]
struct CertificateWidgets {
    root: adw::Leaflet,
    active: gtk::Button,
}

impl CertificateWidgets {
    fn update(&self) {
        // Update qr code if a "better" one was added
    }
}

impl FactoryPrototype for CertificateEntry {
    type Factory = FactoryVec<CertificateEntry>;
    type Widgets = CertificateWidgets;
    type Root = adw::Leaflet;
    type View = adw::Leaflet;
    type Msg = AppMsg;

    fn init_view(&self, _key: &usize, sender: Sender<AppMsg>) -> Self::Widgets {
        // Create widgets.
        let root = adw::Leaflet::new();
        let button_qr = gtk::Button::new();
        let vbox_cert = gtk::Box::new(Orientation::Vertical, 0);
        let qr = crate::qr_code::QRString::new(&self.certificate).unwrap();
        qr.write_svg("/tmp/qrcode.svg");

        let qr_png = gtk::Image::from_file("/tmp/qrcode.svg");
        qr_png.set_vexpand(true);
        qr_png.set_hexpand(true);
        qr_png.set_margin_top(4);
        qr_png.set_margin_bottom(4);
        qr_png.set_margin_start(4);
        qr_png.set_margin_end(4);

        let squeezer = adw::Squeezer::new();
        let label_full_name = gtk::Label::new(Some(&self.full_name));
        let label_short_name = gtk::Label::new(Some(&self.firstname));

        label_short_name.set_hexpand(true);
        label_full_name.set_hexpand(true);

        squeezer.add(&label_full_name);
        squeezer.add(&label_short_name);
        vbox_cert.append(&qr_png);
        vbox_cert.append(&squeezer);
        button_qr.set_child(Some(&vbox_cert));
        root.append(&button_qr);

        // Connect to "clicked" signal of `button`
        let full_name_clone: String = self.full_name.clone();
        button_qr.connect_clicked(move |_| {
            send!(sender, AppMsg::Clicked(full_name_clone.clone()));
        });

        // Checks the validity of the signature
        match &self.signature_validity {
            dgc::SignatureValidity::Valid => {
                button_qr.set_class_active("verified", true);
            }
            e => {
                println!("Could not validate the signature: {}", e);
                button_qr.set_class_active("unverified", true);
            }
        }

        let widgets = CertificateWidgets {
            root,
            active: button_qr,
        };
        widgets.update();

        widgets
    }

    fn position(&self, key: &usize) -> () {
        // TODO: Sort the QR-codes alphabetically??
    }

    fn view(&self, _key: &usize, widgets: &CertificateWidgets) {
        widgets.update();
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.root
    }
}

struct AppModel {
    certificate_entries: FactoryVec<CertificateEntry>,
    /*first_name:
    last_name:
    std_first_name:
    std_last_name:
    birthdate:
    disease:
    vaccine:
    vaccine_type:
    vaccine_manufacturer:
    no_dose:
    no_doses_needed:
    vaccination_date:
    vaccination_country:
    certificate_issuer:
    certificate_id:
    certificate_expiry_date:*/
    certificate_store: crate::cert::CertificateStore,
    display_page: AppPage,
    toast: Option<adw::Toast>,
}

impl AppModel {
    pub fn new() -> Self {
        let calendar_entries = FactoryVec::new();
        let certificate_store = crate::cert::CertificateStore::new();
        let display_page = AppPage::Start;
        let toast = None;
        let mut app_model = Self {
            certificate_entries: calendar_entries,
            certificate_store,
            display_page,
            toast,
        };
        app_model.certificate_store.load_trust_list();
        app_model
    }

    fn throw_toast(&mut self, toast_type: ToastType) {
        let toast = match toast_type {
            ToastType::Success => adw::Toast::new("Certificate was added!"),
            ToastType::FileInvalid => {
                adw::Toast::new("Selected path is invalid. Adding certificate failed!")
            }
            ToastType::CertInvalid => adw::Toast::new(
                "File does not contain a valid certificate. Adding certificate failed!",
            ),
            ToastType::QrPNGInvalid => {
                adw::Toast::new("File does not contain valid QR code. Adding certificate failed!")
            }
            ToastType::Aborted => adw::Toast::new("No certificate was added!"),
        };
        self.toast = Some(toast);
    }
}

#[derive(Debug)]
enum AppPage {
    CertSelector,
    Start,
    Details,
    Certificate,
}

impl AppPage {
    fn to_str(&self) -> &'static str {
        match self {
            AppPage::CertSelector => "cert_selector",
            AppPage::Start => "start",
            AppPage::Details => "details",
            AppPage::Certificate => "cert",
        }
    }
}

enum AppMsg {
    Update,
    Delete,
    ShowPage(AppPage),
    TrowToast(ToastType),
    AddCertificate(std::path::PathBuf),
    Clicked(String),
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Update => {
                // Check all entries
                for i in 0..self.certificate_entries.len() {}
            }
            AppMsg::TrowToast(toast_type) => {
                self.throw_toast(toast_type);
                self.toast = None;
            }
            AppMsg::Delete => {
                // Delete all entries
                self.certificate_entries.clear();
                println!("Calendar entries cleared");
            }
            AppMsg::AddCertificate(path) => {
                println!("Add certificate from path: {:?}", path);
                let raw_certificate_string = read_to_string(path).unwrap();
                let result = self
                    .certificate_store
                    .add_certificate(&raw_certificate_string);
                let raw_certificate_string_owned = String::from(raw_certificate_string);
                send!(sender, AppMsg::ShowPage(AppPage::Start));

                if let Ok((firstname, full_name, signature_validity)) = result {
                    let certificate_entry = CertificateEntry {
                        firstname,
                        full_name,
                        certificate: raw_certificate_string_owned,
                        signature_validity,
                    };
                    self.certificate_entries.push(certificate_entry);

                    send!(sender, AppMsg::TrowToast(ToastType::Success));
                } else {
                    send!(sender, AppMsg::TrowToast(ToastType::FileInvalid));
                }
            }
            AppMsg::Clicked(full_name) => {
                println!("Button for {} was clicked", full_name);
            }
            AppMsg::ShowPage(page) => {
                //self.view_stack;
                println!("Change to the page {:?} was requested", page);
                self.display_page = page;
            }
        }
        true
    }
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = adw::ApplicationWindow {
            set_default_width:480 , // 720
            set_default_height:720, // 1440

            set_content: main_box = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,

                append = &adw::HeaderBar {
                    set_title_widget = Some(&gtk::Label) {
                        set_label: "Covidpass",
                    },
                },
                append = &adw::ToastOverlay {
                    // add_toast: track!(model.toast.is_some(),&model.toast.as_ref().unwrap()),
                    set_child: view_stack = Some(&adw::ViewStack) {
                        set_visible_child_name: watch!{model.display_page.to_str()},
                        add_named(Some(AppPage::Start.to_str())) = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            append: main_view = &adw::Leaflet {
                                factory!(model.certificate_entries)
                            },
                            append = &gtk::Button::with_label("+") {
                                set_margin_all: 5,
                                connect_clicked(sender) => move |_| {
                                    send!(sender, AppMsg::ShowPage(AppPage::CertSelector));
                                },
                            },
                        },
                        add_named(Some(AppPage::Details.to_str())) = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            append = &gtk::Label {
                                set_margin_all: 5,
                               // set_label: watch! { &format!("Counter: {}", model.counter) },
                            }
                        },
                        add_named(Some(AppPage::Certificate.to_str())) = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            append = &gtk::Label {
                                set_margin_all: 5,
                               // set_label: watch! { &format!("Counter: {}", model.counter) },
                            }
                        },
                        add_named(Some(AppPage::CertSelector.to_str())) : file_chooser_box = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            append: file_chooser = &gtk::FileChooserWidget{
                                set_action: gtk::FileChooserAction::Open,
                            },
                            append: select_file_button = &gtk::Button::with_label("Add") {
                                set_margin_all: 5,
                                connect_clicked(sender, file_chooser) => move |_| {
                                    if let Some(selected_file) = file_chooser.file() {
                                        if let Some(path) = selected_file.path() {
                                            send!(sender, AppMsg::AddCertificate(path));
                                        } else {
                                            // No path returned -> throw Toast
                                        }
                                    } else {
                                        // No file selected -> throw Toast
                                    }
                                },
                            },
                            append: cancel_file_button = &gtk::Button::with_label("Cancel") {
                                set_margin_all: 5,
                                connect_clicked(sender) => move |_| {
                                    send!(sender, AppMsg::ShowPage(AppPage::Start));
                                },
                            },
                        },
                    },
                },
                append = &gtk::Button {
                    set_label: "Delete",
                    connect_clicked(sender) => move |_| {
                        println!("Deleted clicked");
                        send!(sender, AppMsg::Delete);
                    },
                },
            },
        }
    }

    // Connect properties, start update thread and load added certificates.
    fn post_init() {
        relm4::set_global_css(
            b".verified { background: #014FBE;}
        .unverified { background: #D61D21;}
        ",
        );
        let sender_clone = sender.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(21600)); // 6 hrs
            send!(sender_clone, AppMsg::Update);
        });

        send!(
            sender,
            AppMsg::AddCertificate(PathBuf::from("../HealthCertificates/covCert.txt"))
        );
    }
}

fn main() {
    let app_model = AppModel::new();
    let app = RelmApp::new(app_model);
    app.run();
}

enum ToastType {
    Success,
    FileInvalid,
    CertInvalid,
    QrPNGInvalid,
    Aborted,
}

/*

fn handle_file_chooser_response(&self, response: gtk::ResponseType) {
        if response == gtk::ResponseType::Accept {
            if let Some(file) = self.file_chooser.file() {
                if let Some(path) = file.path() {
                    if let Ok(((first_name, surname, full_name), cert)) =
                        self.certificate_store.add_certificate(&path)
                    {
                        self.add_qr_png(&first_name, &full_name);
                        throw_toast(ToastType::Success(first_name), &self.toast_overlay);
                    } else {
                        throw_toast(ToastType::CertInvalid, &self.toast_overlay);
                    }
                } else {
                    throw_toast(ToastType::FileInvalid, &self.toast_overlay);
                }
            } else {
                throw_toast(ToastType::FileInvalid, &self.toast_overlay);
            }
        } else {
            throw_toast(ToastType::Aborted, &self.toast_overlay);
        }
    }

*/
