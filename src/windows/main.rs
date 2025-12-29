use gtk4::{gio::*, glib::*, prelude::*, *};

mod actions {
    use gtk4::{gio::*, glib::*, prelude::*, *};

    use crate::{windows::*,project::*};

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

        let sub_win = create_proj::build(&app,&win);
        sub_win.set_modal(true);
        sub_win.set_transient_for(Some(win));
        sub_win.show();
    }

    fn act_set_state_loaded(win: &ApplicationWindow,_: &SimpleAction,_: Option<&Variant>) {
        let app = win.application().expect("no application?");
        let proj = get_proj(&app);
        if proj.is_loaded() {
            println!("Path: {:?}",proj.project_dir);
            println!("Name: {:?}",proj.name);
            println!("Author: {:?}",proj.author);
            println!("Output ROM name: {:?}",proj.rom_out_name);

            win.set_title(Some(format!("{} - Mighty Bitey ROM editor",proj.name.clone().unwrap_or("".to_string())).as_str()));
        } else {
            println!("WARNING! no project loaded, yet set_state_loaded has been called")
        }

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

    pub fn register(win: &gtk4::ApplicationWindow) {
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
    }

}

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

fn make_content_loaded() -> gtk4::Box {
    let toret = gtk4::Box::new(Orientation::Horizontal, 10);

    let factory = SignalListItemFactory::new();

    factory.connect_bind(|_, item| {
        let string = item.item().unwrap().downcast::<StringObject>().unwrap().string();

        if string != "" {
            let label = Label::new(None);
            item.connect_selected_notify(|a| {
                if a.is_selected() {
                    println!("selected {}!",a.child().expect("no child?").downcast::<Label>().expect("no downcast3?").text())
                }
            });

            label.set_text(string.as_str());

            item.set_child(Some(&label));
        } else {
            let butt = Button::new();

            butt.set_label("New Change");
            butt.set_halign(Align::Center);

            item.set_child(Some(&butt));
        }
    });

    let store = gio::ListStore::new::<StringObject>();

    store.append(&StringObject::new("There"));
    store.append(&StringObject::new("Will"));
    store.append(&StringObject::new("Be"));
    store.append(&StringObject::new("Changes"));
    store.append(&StringObject::new("Here"));
    store.append(&StringObject::new(""));

    let smodel: SelectionModel = SingleSelection::new(Some(store)).into();

    let right = gtk4::Box::new(Orientation::Horizontal, 10);

    right.append(&make_content_unloaded());

    let left = ListView::new(Some(smodel),Some(factory));

    left.set_hexpand(true);
    left.set_vexpand(true);

    toret.append(&left);
    toret.append(&right);

    //connnect the tab switch event
    toret.connect_map(move |_ob| {
        println!("laoded :3");
    });

    return toret;
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

pub fn build(app: &gtk4::Application) -> ApplicationWindow{
    let win = ApplicationWindow::new(app);

    win.set_title(Some("Mighty bitey ROM editor"));
    win.set_default_width(400);
    win.set_default_height(300);
    win.set_show_menubar(true);

    actions::register(&win);

    let content = make_content(app);
    win.set_child(Some(&content));

    return win;
}