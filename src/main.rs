#![warn(clippy::pedantic)]
#![allow(clippy::used_underscore_binding)]

use relm4::RelmApp;

use crate::ui::Mode;
use crate::ui::Model;

mod ui;

fn main() {
    let relm = RelmApp::new("ewlm4.test.components");
    relm.run::<Model>(Mode::Edit);
}
