use gtk4::{prelude::*, *};

pub mod windows;
pub mod project;

fn main() {
    let app = Application::builder()
            .application_id("dev.laurinyev.mighty_bitey")
            .build();

    app.connect_activate(|app| {
        project::PROJ.set(project::Project::default()).expect("Failed to set unloaded project");

        let win = windows::main::build(app);
        win.show();
    });

    app.run();
}
