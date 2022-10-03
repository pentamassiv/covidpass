use gtk::prelude::{ButtonExt, ToggleButtonExt, WidgetExt};
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct Model;

#[derive(Debug)]
pub enum Output {
    View,
    Edit,
    Export,
}

#[relm4::component(visibility = pub)]
impl SimpleComponent for Model {
    type Input = ();

    type Output = Output;

    type Init = ();

    type Widgets = HeaderWidgets;

    view! {
        #[root]
        gtk::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &gtk::Box {
                add_css_class: "linked",
                #[name = "group"]
                gtk::ToggleButton {
                    set_label: "View",
                    set_active: true,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(Output::View);
                        }
                    },
                },
                gtk::ToggleButton {
                    set_label: "Edit",
                    set_group: Some(&group),
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(Output::Edit);
                        }
                    },
                },
                gtk::ToggleButton {
                    set_label: "Export",
                    set_group: Some(&group),
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(Output::Export);
                        }
                    },
                },
            }
        }
    }

    fn init(
        _params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Model;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
