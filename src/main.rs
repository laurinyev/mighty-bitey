use std::{cell::RefCell, rc::Rc};

use gtk4::{prelude::*, *};

use crate::project::Project;

pub mod windows;
pub mod project;

fn main() {
    let app = Application::builder()
        .application_id("dev.laurinyev.mighty_bitey")
        .build();

    let proj = Rc::new(RefCell::new(Project::default()));

    app.connect_activate(move |app| {
        let win = windows::main::build(app,proj.clone());
        win.show();
    });

    app.run();
}
