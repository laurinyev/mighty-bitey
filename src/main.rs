use gtk4::{prelude::*, *};

use crate::global::*;

pub mod windows;
pub mod global;
pub mod patching;

fn main() {
    let app = Application::builder()
        .application_id("dev.laurinyev.mighty_bitey")
        .build();

    init_glob();

    app.connect_activate(move |app| {
        let win = windows::main::build(app);
        win.show();
    });

    app.run();
}
