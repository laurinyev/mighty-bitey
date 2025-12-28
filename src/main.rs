pub mod windows;

use gtk4::{prelude::*, *};
fn main() {
    let app = Application::builder()
            .application_id("dev.laurinyev.mighty_bitey")
            .build();

    app.connect_activate(|app| {
        let win = windows::main::build(app);

        win.show();
    });

    app.run();
}
