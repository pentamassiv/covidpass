use adw::prelude::AdwApplicationWindowExt;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{adw, gtk, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

mod cert;
mod pub_keys;

struct AppModel {
    counter: u8,
    certificate_store: cert::CertificateStore,
}

impl AppModel {
    pub fn new() -> Self {
        let counter = 0;
        let certificate_store = cert::CertificateStore::new();
        AppModel {
            counter,
            certificate_store,
        }
    }
}

enum AppMsg {
    Increment,
    Decrement,
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
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
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
                    }
                },

                append = &adw::ToastOverlay {
                    set_child = Some(&adw::ViewStack) {
                        add_named(Some("start_page")) = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            append = &adw::Leaflet {},
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
                                set_label: watch! { &format!("Counter: {}", model.counter) },
                            }
                        }
                    },
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 5,
                    set_spacing: 5,

                    append = &gtk::Button {
                        set_label: "Increment",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::Increment);
                        },
                    },
                    append = &gtk::Button {
                        set_label: "Decrement",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::Decrement);
                        },
                    },
                    append = &gtk::Label {
                        set_margin_all: 5,
                        set_label: watch! { &format!("Counter: {}", model.counter) },
                    }
                }
            },
        }
    }
}

fn main() {
    let model = AppModel::new();
    let app = RelmApp::new(model);
    app.run();
}
