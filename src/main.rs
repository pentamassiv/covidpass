use adw::prelude::AdwApplicationWindowExt;
use dgc::DgcContainer;
use gtk::glib::BindingFlags;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, ObjectExt, OrientableExt, ToggleButtonExt};
use qrcode::{EcLevel, QrCode};
use relm4::{
    adw,
    factory::{positions::StackPageInfo, FactoryPrototype, FactoryVec},
    gtk, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets,
};

use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar};
use gtk::{Application, Button, Image, Orientation};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{error::Error, fs::read_to_string};

mod cert;
mod pub_keys;

fn waiting_text(time: u32) -> String {
    format!(
        "You need to wait {} seconds before you can open this door.",
        time
    )
}

#[derive(Debug)]
struct CertificateEntry {
    firstname: String,
    lastname: String,
    time_left: u32,
}

#[derive(Debug)]
struct CertificateWidgets {
    root: gtk::Stack,
    active: gtk::Box,
    waiting: gtk::CenterBox,
    waiting_label: gtk::Label,
}

impl CertificateWidgets {
    // Update widgets to new time
    fn update(&self, time: u32) {
        if time == 0 {
            self.root.set_visible_child(&self.active);
        } else {
            self.waiting_label.set_label(&waiting_text(time));
            self.root.set_visible_child(&self.waiting);
        }
    }
}

impl FactoryPrototype for CertificateEntry {
    type Factory = FactoryVec<CertificateEntry>;
    type Widgets = CertificateWidgets;
    type Root = gtk::Stack;
    type View = gtk::Stack;
    type Msg = AppMsg;

    fn init_view(&self, _key: &usize, _sender: Sender<AppMsg>) -> Self::Widgets {
        // Create widgets.
        let root = gtk::Stack::builder()
            .vexpand(true)
            .transition_type(gtk::StackTransitionType::RotateLeftRight)
            .transition_duration(700)
            .build();

        let active = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(7)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build();
        let verse = gtk::Label::builder()
            .label(self.firstname.as_str())
            .css_classes(vec!["verse".to_string()])
            .wrap(true)
            .selectable(true)
            .build();
        let passage = gtk::Label::builder()
            .label(self.lastname.as_str())
            .selectable(true)
            .halign(gtk::Align::End)
            .build();

        let waiting = gtk::CenterBox::new();
        let waiting_label = gtk::Label::new(Some(&waiting_text(self.time_left)));

        // Connect widgets.
        active.append(&verse);
        active.append(&passage);
        active.set_margin_all(30);

        waiting.set_center_widget(Some(&waiting_label));

        root.add_child(&active);
        root.add_child(&waiting);

        let widgets = CertificateWidgets {
            root,
            waiting_label,
            waiting,
            active,
        };
        widgets.update(self.time_left);

        widgets
    }

    fn position(&self, key: &usize) -> StackPageInfo {
        StackPageInfo {
            name: Some(key.to_string()),
            title: Some(format!("Day {}", key + 1)),
        }
    }

    fn view(&self, _key: &usize, widgets: &CertificateWidgets) {
        widgets.update(self.time_left);
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.root
    }
}

struct AppModel {
    calendar_entries: FactoryVec<CertificateEntry>,
    start_page: u8,
    certificates: HashMap<(String, String, String), DgcContainer>,
    trust_list: dgc::TrustList,
}

impl AppModel {
    pub fn new(calendar_entries: FactoryVec<CertificateEntry>, start_page: u8) -> Self {
        let certificates = HashMap::new();
        // We create a new Trustlist (container of "trusted" public keys)
        let trust_list = dgc::TrustList::default();
        Self {
            calendar_entries,
            start_page,
            certificates,
            trust_list,
        }
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

    pub fn add_certificate(
        &self,
        cert_path: &std::path::Path,
    ) -> Result<((String, String, String), dgc::DgcContainer), Box<dyn Error>> {
        let raw_certificate_data = read_to_string(cert_path)?;
        // Now we can validate the signature (this returns)
        let (mut certificate_container, signature_validity) =
            dgc::validate(&raw_certificate_data, &self.trust_list)
                .expect("Cannot parse certificate data");

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
        let forename = dgc_name.name.forename.clone().unwrap_or("".into());
        let surname = dgc_name.name.surname.clone().unwrap_or("".into());
        let mut full_name = forename.clone();
        full_name.push_str(" ");
        full_name.push_str(&surname);

        Self::generate_qr_code(&raw_certificate_data)?;
        Ok(((forename, surname, full_name), certificate_container))
    }
    fn generate_qr_code(data: &String) -> Result<(), Box<dyn Error>> {
        let qr_code = QrCode::with_error_correction_level(data, EcLevel::L)?;
        let image = qr_code.render::<image::Luma<u8>>().build();
        image.save("/tmp/qrcode.png")?;
        Ok(())
    }
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
                for i in 0..self.calendar_entries.len() {
                    // If counter > 1, count down
                    let needs_update = self.calendar_entries.get(i).unwrap().time_left != 0;

                    if needs_update {
                        let entry = self.calendar_entries.get_mut(i).unwrap();
                        entry.time_left = entry.time_left.saturating_sub(1);
                    }
                }
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
                            append: leaflet = &adw::Leaflet {
                                append: main_view = &gtk::Stack {
                                factory!(model.calendar_entries)
                                },
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

        main_view.set_visible_child_name(&model.start_page.to_string());

        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            send!(sender, AppMsg::Update);
        });
    }
}

fn main() {
    let mut calendar_entries = FactoryVec::new();
    let mut start_page = 0;

    // Time since midnight December the 1st.
    let time_since_first_dec = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - 1638313200;
    let seconds_per_day: i64 = 86400;

    let verses = vec![
        ("Come unto me, all ye that labour and are heavy laden, and I will give you rest. Take my yoke upon you, and learn of me; for I am meek and lowly in heart: and ye shall find rest unto your souls. For my yoke is easy, and my burden is light.", "Matthew 11, 28-30"),
        ("For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. For God sent not his Son into the world to condemn the world; but that the world through him might be saved.", "John 3, 16-17"),
        ("If any man thirst, let him come unto me, and drink. He that believeth on me, as the scripture hath said, out of his belly shall flow rivers of living water.", "John 8, 37-38"),
        ("For the LORD God is a sun and shield: the LORD will give grace and glory: no good thing will he withhold from them that walk uprightly.", "Psalm 84, 11"),
        ("Trust in the LORD with all thine heart; and lean not unto thine own understanding. In all thy ways acknowledge him, and he shall direct thy paths.", "Proverbs 3, 5-6"),
        ("Blessed be the God and Father of our Lord Jesus Christ, who has blessed us with every spiritual blessing in the heavenly places in Christ, just as He chose us in Him before the foundation of the world, that we should be holy and without blame before Him in love.", "Ephesians 1,3-4"),
        ("Jesus saith unto him, I am the way, the truth, and the life: no man cometh unto the Father, but by me.", "John 14, 6"),
        ("Peace I leave with you, my peace I give unto you: not as the world giveth, give I unto you. Let not your heart be troubled, neither let it be afraid.", "John 14, 27"),
        ("But the fruit of the Spirit is love, joy, peace, longsuffering, gentleness, goodness, faith, meekness, temperance: against such there is no law.", "Romans 8, 22-23"),
        ("Then spake Jesus again unto them, saying, I am the light of the world: he that followeth me shall not walk in darkness, but shall have the light of life.", "John 8, 12"),
        ("Rejoice in the Lord alway: and again I say, Rejoice. Let your moderation be known unto all men. The Lord is at hand.", "Philippians 4, 4-5"),
        ("For the word of God is quick, and powerful, and sharper than any twoedged sword, piercing even to the dividing asunder of soul and spirit, and of the joints and marrow, and is a discerner of the thoughts and intents of the heart.", "Hebrews 4, 12"),
        ("Commit to the Lord whatever you do, and he will establish your plans.", "Proverbs 16:3"),
        ("Let the morning bring me word of your unfailing love, for I have put my trust in you. Show me the way I should go, for to you I entrust my life.", "Psalm 143:8"),
        ("Be completely humble and gentle; be patient, bearing with one another in love.", "Ephesians 4:2"),
        ("Whatever you do, work at it with all your heart, as working for the Lord, not for human masters, since you know that you will receive an inheritance from the Lord as a reward. It is the Lord Christ you are serving.", "Colossians 3:23-24"),
        ("Let love and faithfulness never leave you;bind them around your neck, write them on the tablet of your heart. Then you will win favor and a good name in the sight of God and man.", "Proverbs 3:3-4"),
        ("So do not fear, for I am with you; do not be dismayed, for I am your God. I will strengthen you and help you; I will uphold you with my righteous right hand.", "Isaiah 41:10"),
        ("But blessed is the one who trusts in the Lord, whose confidence is in him. They will be like a tree planted by the water that sends out its roots by the stream. It does not fear when heat comes; its leaves are always green. It has no worries in a year of drought and never fails to bear fruit.", "Jeremiah 17:7-8"),
        ("Love is patient, love is kind. It does not envy, it does not boast, it is not proud. It does not dishonor others, it is not self-seeking, it is not easily angered, it keeps no record of wrongs.", "1 Corinthians 13:4-5"),
        ("Do not be anxious about anything, but in every situation, by prayer and petition, with thanksgiving, present your requests to God. And the peace of God, which transcends all understanding, will guard your hearts and your minds in Christ Jesus.", "Philippians 4:6-7"),
        ("These commandments that I give you today are to be on your hearts. Impress them on your children. Talk about them when you sit at home and when you walk along the road, when you lie down and when you get up.", "Deuteronomy 6:6-7"),
        ("But blessed is the one who trusts in the Lord, whose confidence is in him. They will be like a tree planted by the water that sends out its roots by the stream. It does not fear when heat comes; its leaves are always green. It has no worries in a year of drought and never fails to bear fruit.", "Jeremiah 17:7-8"),
        ("The Lord bless you and keep you; the Lord make his face shine on you and be gracious to you; the Lord turn his face toward you and give you peace.", "Numbers 6:24-26"),
    ];

    // Fill factory with the verses
    for (idx, (verse, passage)) in verses.iter().enumerate() {
        let time_difference = seconds_per_day * idx as i64 - time_since_first_dec as i64;
        let time_left = if time_difference > 0 {
            time_difference as u32
        } else {
            start_page = idx as u8;
            0
        };

        calendar_entries.push(CertificateEntry {
            firstname: verse.to_string(),
            lastname: passage.to_string(),
            time_left,
        });
    }

    let app = RelmApp::new(AppModel::new(calendar_entries, start_page));

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
