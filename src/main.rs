use gtk4::{prelude::*, *};

use crate::project::*;

pub mod windows;
pub mod project;

fn main() {
    let app = Application::builder()
        .application_id("dev.laurinyev.mighty_bitey")
        .build();

    init_proj();

    app.connect_activate(move |app| {
        let win = windows::main::build(app);
        win.show();
    });

    app.run();
}
