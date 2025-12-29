use gtk4::{gio::*, glib::*, prelude::*, *};

fn build_menu_model() -> MenuModel {
    let menu = Menu::new();

    let file = Menu::new();

    let new = MenuItem::new(Some("New Proj"), Some("win.new_proj"));
    let open = MenuItem::new(Some("Open Proj"), Some("win.open_proj"));
    let quit = MenuItem::new(Some("Quit"), Some("win.close"));

    file.append_item(&open);
    file.append_item(&new);
    file.append_item(&quit);

    let file_model: MenuModel = file.into();

    menu.append_submenu(Some("file"),&file_model);

    return menu.into();
}

fn make_content_unloaded() -> gtk4::Label {
    let lab = Label::new(Some("Welcome to the Mighty Bitey ROM editor\nPlease create or open a project"));
    lab.set_vexpand(true);
    lab.set_justify(Justification::Center);

    return lab;
}

fn make_content_loaded() -> gtk4::Button {
    let butt = Button::new();

    butt.set_label("I am a button that does something via magic");
    butt.set_margin_top(100);
    butt.set_margin_bottom(100);
    butt.set_margin_start(100);
    butt.set_margin_end(100);

    butt.connect_clicked(|_| {
        println!("How did you do that?!")
    });

    return butt;
}

fn make_content(_: &gtk4::Application) -> gtk4::Box {
    let menubar = PopoverMenuBar::from_model(Some(&build_menu_model()));

    let winchild = gtk4::Box::new(Orientation::Vertical, 0);

    winchild.set_vexpand(true);

    let content_unloaded = make_content_unloaded();
    let content_loaded   = make_content_loaded();

    let stack = gtk4::Stack::new();

    stack.add_named(&content_unloaded,Some("unloaded"));
    stack.add_named(&content_loaded,Some("loaded"));

    stack.set_visible_child_name("unloaded");

    winchild.append(&menubar);
    winchild.append(&stack);

    return winchild;
}

fn act_open_rom(_: &ApplicationWindow,_: &SimpleAction,_: Option<&Variant>) {
    let filter = FileFilter::new();

    filter.set_name(Some("Mother 3 ROMs (*.gba)"));
    filter.add_pattern("*.gba");

    let dia = FileChooserNative::builder()
        .action(FileChooserAction::Open)
        .title("Open a ROM")
        .build();

    dia.add_filter(&filter);

    dia.connect_response(|d,r| {
        if r == ResponseType::Accept {
            println!("{:?}",d.file().expect("no file wtf").path());
        } else {
            println!("Cancelled")
        }
    });

    dia.show();
}

fn act_open_proj(_: &ApplicationWindow,_: &SimpleAction,_: Option<&Variant>) {
    let dia = FileChooserNative::builder()
        .action(FileChooserAction::SelectFolder)
        .title("Open a project directory")
        .build();

    dia.connect_response(|d,r| {
        if r == ResponseType::Accept {
            println!("{:?}",d.file().expect("no file wtf").path());
        } else {
            println!("Cancelled")
        }
    });

    dia.show();
}

fn act_new_proj(win: &ApplicationWindow,_: &SimpleAction,_: Option<&Variant>) {
    let app = win.application().expect("no app? WTF");

    let sub_win = super::create_proj::build(&app,&win);
    sub_win.set_modal(true);
    sub_win.set_transient_for(Some(win));
    sub_win.show();
}

fn act_set_state_loaded(win: &ApplicationWindow,_: &SimpleAction,_: Option<&Variant>) {
    //just set the window state for now
    win.child()
        .expect("no child?")
        .downcast::<gtk4::Box>()
        .expect("no downcast?")
        .last_child()
        .expect("no last child?")
        .downcast::<gtk4::Stack>()
        .expect("no downcast2?")
        .set_visible_child_name("loaded");

    win.set_width_request(600);
    win.set_height_request(650);
}

pub fn build(app: &gtk4::Application) -> ApplicationWindow{
    let win = ApplicationWindow::new(app);

    win.set_title(Some("Mighty bitey ROM editor"));
    win.set_default_width(400);
    win.set_default_height(300);
    win.set_show_menubar(true);

    let act_open_rom: ActionEntry<ApplicationWindow> = ActionEntryBuilder::new("open_rom")
        .activate(act_open_rom)
        .build();

    let act_open_proj: ActionEntry<ApplicationWindow> = ActionEntryBuilder::new("open_proj")
        .activate(act_open_proj)
        .build();

    let act_new_proj = ActionEntryBuilder::new("new_proj")
        .activate(act_new_proj)
        .build();

    let act_set_state_loaded = ActionEntryBuilder::new("set_state_loaded")
        .activate(act_set_state_loaded)
        .build();

    let act_close = ActionEntryBuilder::new("close")
        .activate(|win: &ApplicationWindow ,_ ,_| {
            win.close();
        })
        .build();

    win.add_action_entries([act_close,act_new_proj,act_open_rom,act_open_proj,act_set_state_loaded]);

    let content = make_content(app);
    win.set_child(Some(&content));

    return win;
}