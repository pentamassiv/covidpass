extern crate qrcodegen;

use adw::prelude::AdwApplicationWindowExt;
use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar};
use dgc::DgcContainer;
use gio::ListModel;
use gtk::glib::BindingFlags;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, ObjectExt, OrientableExt, ToggleButtonExt};
use gtk::{Application, Button, Image, Orientation, ResponseType};
use relm4::{
    adw,
    factory::{FactoryPrototype, FactoryVec},
    gtk, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets,
};
use std::collections::HashMap;
use std::{error::Error, fs::read_to_string};

mod cert;
mod pub_keys;
mod qr_code;

#[derive(Debug)]
struct CertificateEntry {
    firstname: String,
    full_name: String,
    certificate: String,
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
        let qr = crate::qr_code::QRString::new(&self.certificate);
        let text_view = gtk4::TextView::new();
        if let Ok(qr_string) = qr {
            text_view.buffer().set_text(&qr_string.to_string());
            text_view.set_monospace(true);
        }
        let squeezer = adw::Squeezer::new();
        let label_full_name = gtk::Label::new(Some(&self.full_name));
        let label_short_name = gtk::Label::new(Some(&self.firstname));

        label_short_name.set_hexpand(true);
        label_full_name.set_hexpand(true);

        squeezer.add(&label_full_name);
        squeezer.add(&label_short_name);
        vbox_cert.append(&text_view);
        vbox_cert.append(&squeezer);
        button_qr.set_child(Some(&vbox_cert));
        root.append(&button_qr);

        // Connect to "clicked" signal of `button`
        let full_name_clone: String = self.full_name.clone();
        button_qr.connect_clicked(move |_| {
            send!(sender, AppMsg::Clicked(full_name_clone.clone()));
        });

        button_qr.set_class_active("verified", true);

        let widgets = CertificateWidgets {
            root,
            active: button_qr,
        };
        widgets.update();

        widgets
    }

    fn position(&self, key: &usize) -> () {}

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
    certificates: HashMap<String, DgcContainer>,
    trust_list: dgc::TrustList,
    display_page: AppPage,
    toast: Option<adw::Toast>,
}

impl AppModel {
    pub fn new() -> Self {
        let calendar_entries = FactoryVec::new();
        let certificates = HashMap::new();
        // We create a new Trustlist (container of "trusted" public keys)
        let trust_list = dgc::TrustList::default();
        let display_page = AppPage::Start;
        let toast = None;
        let mut app_model = Self {
            certificate_entries: calendar_entries,
            certificates,
            trust_list,
            display_page,
            toast,
        };
        app_model.load_trust_list();
        app_model
    }

    pub fn load_trust_list(&mut self) -> Result<(), Box<dyn Error>> {
        let pub_keys = crate::pub_keys::read_file("trust_list.txt");

        for key in pub_keys {
            // We add the public key in the certificate to the trustlist
            self.trust_list
                .add_key_from_certificate(&key)
                .expect("Failed to add key from certificate");
        }
        Ok(())
    }

    pub fn add_certificate(&mut self, raw_cert_data: &str) -> Result<(), Box<dyn Error>> {
        // Now we can validate the signature (this returns)
        let (mut certificate_container, signature_validity) =
            dgc::validate(raw_cert_data, &self.trust_list).expect("Cannot parse certificate data");

        println!("{:#?}", &certificate_container);

        // Checks the validity of the signature
        match signature_validity {
            dgc::SignatureValidity::Valid => println!("The certificate signature is Valid!"),
            e => println!("Could not validate the signature: {}", e),
        }

        // you can call `expand_values()` to resolve all the IDs against a well known valueset embedded in the library
        certificate_container.expand_values();

        println! {"{:?}",certificate_container};
        let dgc_name = certificate_container.certs.get(&1).unwrap();
        let firstname = dgc_name.name.forename.clone().unwrap_or("".into());
        let surname = dgc_name.name.surname.clone().unwrap_or("".into());
        let mut full_name = firstname.clone();
        full_name.push_str(" ");
        full_name.push_str(&surname);

        let certificate_entry = CertificateEntry {
            firstname,
            full_name: full_name.clone(),
            certificate: String::from(raw_cert_data),
        };
        self.certificates.insert(full_name, certificate_container);
        self.certificate_entries.push(certificate_entry);
        Ok(())
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
                let raw_certificate_data = read_to_string(path).unwrap();
                let result = self.add_certificate(&raw_certificate_data);
                send!(sender, AppMsg::ShowPage(AppPage::Start));
                if result.is_ok() {
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

    // Connect properties and start update thread.
    fn post_init() {
        relm4::set_global_css(b".verified { background: #014FBE; }");

        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(21600)); // 6 hrs
            send!(sender, AppMsg::Update);
        });
    }
}

fn main() {
    let mut app_model = AppModel::new();
    let raw_certificate_data = read_to_string("../HealthCertificates/covCert.txt").unwrap();
    app_model.add_certificate(&raw_certificate_data);
    let app = RelmApp::new(app_model);

    // Style the verse labels
    relm4::set_global_css(
        b"\
        .verse { \
            font-weight: bold; \
            font-size: 1.4em; \
        }",
    );
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
