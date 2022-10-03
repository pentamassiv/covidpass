use gtk::prelude::{DialogExt, GtkWindowExt, WidgetExt};
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct Model {
    hidden: bool,
}

#[derive(Debug)]
pub enum Input {
    Show,
    Accept,
    Cancel,
}

#[derive(Debug)]
pub enum Output {
    Close,
}

#[relm4::component(visibility = pub)]
impl SimpleComponent for Model {
    type Input = Input;

    type Output = Output;

    type Init = bool;

    type Widgets = DialogWidgets;

    view! {
        gtk::MessageDialog {
            set_modal: true,
            #[watch]
            set_visible: !model.hidden,
            set_text: Some("Do you want to close before saving?"),
            set_secondary_text: Some("All unsaved changes will be lost"),
            add_button: ("Close", gtk::ResponseType::Accept),
            add_button: ("Cancel", gtk::ResponseType::Cancel),
            connect_response[sender] => move |_, resp| {
                sender.input(if resp == gtk::ResponseType::Accept {
                    Input::Accept
                } else {
                    Input::Cancel
                });
            }
        }
    }

    fn init(
        params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Model { hidden: params };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            Input::Show => self.hidden = false,
            Input::Accept => {
                self.hidden = true;
                sender.output(Output::Close);
            }
            Input::Cancel => self.hidden = true,
        }
    }
}
