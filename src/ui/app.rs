use gtk::prelude::GtkWindowExt;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

use super::dialog;
use super::header;

#[derive(Debug)]
pub enum Mode {
    View,
    Edit,
    Export,
}

#[derive(Debug)]
pub enum Input {
    SetMode(Mode),
    CloseRequest,
    Close,
}

pub struct Model {
    mode: Mode,
    header: Controller<header::Model>,
    dialog: Controller<dialog::Model>,
}

#[relm4::component(visibility = pub)]
impl SimpleComponent for Model {
    type Input = Input;

    type Output = ();

    type Init = Mode;

    type Widgets = AppWidgets;

    view! {
        main_window = gtk::Window {
            set_default_width: 500,
            set_default_height: 250,
            set_titlebar: Some(model.header.widget()),

            gtk::Label {
                #[watch]
                set_label: &format!("Placeholder for {:?}", model.mode),
            },
            connect_close_request[sender] => move |_| {
                sender.input(Input::CloseRequest);
                gtk::Inhibit(true)
            }
        }
    }

    fn init(
        params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Model {
            mode: params,
            header: header::Model::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    header::Output::View => Input::SetMode(Mode::View),
                    header::Output::Edit => Input::SetMode(Mode::Edit),
                    header::Output::Export => Input::SetMode(Mode::Export),
                }),
            dialog: dialog::Model::builder()
                .launch(true)
                .forward(sender.input_sender(), |msg| match msg {
                    dialog::Output::Close => Input::Close,
                }),
        };
        model.dialog.widget().set_transient_for(Some(root));
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Input::SetMode(mode) => {
                self.mode = mode;
            }
            Input::CloseRequest => {
                self.dialog.sender().send(dialog::Input::Show);
            }
            Input::Close => {} // UNIMPLEMENTED
        }
    }
}
