use gtk4::{gio::*, glib::property::PropertyGet, prelude::*, *};

fn build_menu_model() -> MenuModel {
    let menu = Menu::new();

    let file = Menu::new();

    let load = MenuItem::new(Some("Load ROM"), Some("win.open"));
    let save = MenuItem::new(Some("Save"), None);
    let quit = MenuItem::new(Some("Quit"), Some("win.close"));

    file.append_item(&load);
    file.append_item(&save);
    file.append_item(&quit);

    let file_model: MenuModel = file.into();

    menu.append_submenu(Some("file"),&file_model);

    return menu.into();
}

fn make_content(app: &gtk4::Application) -> gtk4::Box {
    let menubar = PopoverMenuBar::from_model(Some(&build_menu_model()));

    let lab = Label::new(Some("Welcome to the Mighty Bitey ROM editor\nNothing to see here lul"));
    lab.set_vexpand(true);
    lab.set_justify(Justification::Center);

    let cont = gtk4::Box::new(Orientation::Vertical, 0);

    cont.set_vexpand(true);

    cont.append(&menubar);
    cont.append(&lab);

    return cont;
}

pub fn build(app: &gtk4::Application) -> ApplicationWindow{
    let win = ApplicationWindow::new(app);

    win.set_title(Some("Mighty bitey ROM editor"));
    win.set_default_width(400);
    win.set_default_height(300);
    win.set_show_menubar(true);

    let act_open = ActionEntryBuilder::new("open")
        .activate(|win: &ApplicationWindow ,_ ,_| {
            let dia = FileChooserNative::builder()
                    .action(FileChooserAction::Open)
                    .build();

            dia.connect_response(|d,r| {
                if r == ResponseType::Accept {
                    println!("{:?}",d.file());
                } else {
                    println!("Cancelled")
                }
            });

            dia.show();
        })
        .build();

    let act_close = ActionEntryBuilder::new("close")
        .activate(|win: &ApplicationWindow ,_ ,_| {
            win.close();
        })
        .build();

    win.add_action_entries([act_close,act_open]);

    let content = make_content(app);
    win.set_child(Some(&content));

    return win;
}