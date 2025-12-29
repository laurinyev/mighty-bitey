use gtk4::{prelude::*, *};

fn make_content(mainwin: gtk4::ApplicationWindow,popupwin: gtk4::ApplicationWindow) -> gtk4::Box {
    let winchild = gtk4::Box::new(Orientation::Vertical, 0);

    let lab = Label::new(Some("This popup window will have\nall the project controls"));
    lab.set_vexpand(true);
    lab.set_justify(Justification::Center);

    let butt = Button::with_label("Create project");

    butt.set_halign(Align::End);
    butt.set_margin_bottom(10);
    butt.set_margin_end(10);

    butt.connect_clicked(move |_| {
        mainwin.lookup_action("set_state_loaded").expect("faild to get state loader").activate(None);
        popupwin.close();
    });

    winchild.append(&lab);
    winchild.append(&butt);
    winchild.set_vexpand(true);

    return winchild;
}

pub fn build(app: &gtk4::Application,mainwin: &gtk4::ApplicationWindow) -> ApplicationWindow{
    let win = ApplicationWindow::new(app);

    win.set_title(Some("Mighty bitey ROM editor"));
    win.set_default_width(300);
    win.set_default_height(300);

    let content = make_content(mainwin.clone(),win.clone());
    win.set_child(Some(&content));

    return win;
}