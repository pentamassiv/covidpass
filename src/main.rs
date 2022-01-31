use adw::prelude::AdwApplicationWindowExt;
use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar};
use dgc::DgcContainer;
use gtk::glib::BindingFlags;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, ObjectExt, OrientableExt, ToggleButtonExt};
use gtk::{Application, Button, Image, Orientation};
use qrcode::{EcLevel, QrCode};
use relm4::{
    adw,
    factory::{positions::StackPageInfo, FactoryPrototype, FactoryVec},
    gtk, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets,
};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{error::Error, fs::read_to_string};

mod cert;
mod pub_keys;

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
    // Update widgets to new time
    fn update(&self) {}
}

impl FactoryPrototype for CertificateEntry {
    type Factory = FactoryVec<CertificateEntry>;
    type Widgets = CertificateWidgets;
    type Root = adw::Leaflet;
    type View = adw::Leaflet;
    type Msg = AppMsg;

    fn init_view(&self, _key: &usize, _sender: Sender<AppMsg>) -> Self::Widgets {
        // Create widgets.
        let root = adw::Leaflet::new();

        let button_qr = gtk::Button::new();
        let vbox_cert = gtk::Box::new(Orientation::Vertical, 0);

        generate_qr_code(&self.certificate);
        let qr_png = Image::from_file("/tmp/qrcode.png");
        let squeezer = adw::Squeezer::new();
        let label_full_name = gtk::Label::new(Some(&self.firstname));
        let label_short_name = gtk::Label::new(Some(&self.full_name));

        qr_png.set_vexpand(true);
        qr_png.set_hexpand(true);
        qr_png.set_margin_top(4);
        qr_png.set_margin_bottom(4);
        qr_png.set_margin_start(4);
        qr_png.set_margin_end(4);
        label_short_name.set_hexpand(true);
        label_full_name.set_hexpand(true);

        squeezer.add(&label_full_name);
        squeezer.add(&label_short_name);
        vbox_cert.append(&qr_png);
        vbox_cert.append(&squeezer);
        button_qr.set_child(Some(&vbox_cert));
        root.append(&button_qr);

        // Connect to "clicked" signal of `button`
        let firstname_clone: String = self.firstname.clone();
        button_qr.connect_clicked(move |_| {
            println!("Pushed qr for {}", firstname_clone);
        });

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
    calendar_entries: FactoryVec<CertificateEntry>,
    certificates: HashMap<String, DgcContainer>,
    trust_list: dgc::TrustList,
}

impl AppModel {
    pub fn new() -> Self {
        let calendar_entries = FactoryVec::new();
        let certificates = HashMap::new();
        // We create a new Trustlist (container of "trusted" public keys)
        let trust_list = dgc::TrustList::default();
        let mut app_model = Self {
            calendar_entries,
            certificates,
            trust_list,
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
        self.calendar_entries.push(certificate_entry);
        Ok(())
    }
}

fn generate_qr_code(data: &str) -> Result<(), Box<dyn Error>> {
    let qr_code = QrCode::with_error_correction_level(data, EcLevel::L)?;
    let image = qr_code.render::<image::Luma<u8>>().build();
    image.save("/tmp/qrcode.png")?;
    Ok(())
}

enum AppMsg {
    Update,
    Delete,
    AddCertificate,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Update => {
                // Check all entries
                for i in 0..self.calendar_entries.len() {}
            }
            AppMsg::Delete => {
                // Check all entries
                println!("Calendar entries before: {:?}", self.calendar_entries);
                self.calendar_entries.clear();
                println!("Calendar entries cleared");
                println!("Calendar entries after: {:?}", self.calendar_entries);
            }
            AppMsg::AddCertificate => {
                println!("AddCertificate");
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
                    set_child: view_stack = Some(&adw::ViewStack) {
                        add_named(Some("start_page")) = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            append: main_view = &adw::Leaflet {
                                factory!(model.calendar_entries)
                            },
                            append = &gtk::Button {
                                set_label: "+",
                                set_margin_all: 5,
                                connect_clicked(sender) => move |_| {
                                    send!(sender, AppMsg::AddCertificate);
                                },
                            },
                        },
                        add_named(Some("detail_page")) = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            append = &gtk::Label {
                                set_margin_all: 5,
                               // set_label: watch! { &format!("Counter: {}", model.counter) },
                            }
                        }
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
        /*
                let givenname = String::from("GivenName");
                let full_name = String::from("FullName");

                let qr_png = gtk::Image::from_file("/tmp/qrcode.png");
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
                let view_stack_clone = view_stack.clone();
                let givenname_clone: String = givenname.into();
                button_qr.connect_clicked(move |_| {
                    println!("Pushed qr for {}", givenname_clone);
                    println!(
                        "Currently visible: {}",
                        view_stack_clone.visible_child_name().unwrap()
                    );
                    view_stack_clone.set_visible_child_name("detail_page");
                });
        */

        //main_view.set_visible_child_name(&model.start_page.to_string());

        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
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
