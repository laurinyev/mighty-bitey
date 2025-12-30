use gtk4::{gio::*, prelude::*, *};

use crate::project::*;

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
            }
        });

        dia.show();
    }

    fn act_open_proj(win: &ApplicationWindow,_: &SimpleAction,_: Option<&Variant>) {
        let filter = FileFilter::new();

        filter.set_name(Some("Mighty-Bitey projects (*.mbproj)"));
        filter.add_pattern("*.mbproj");

        let dia = FileChooserNative::builder()
            .action(FileChooserAction::Open)
            .title("Open project")
            .build();

        dia.add_filter(&filter);

        let win_clone = win.clone();
        dia.connect_response(move |d,r| {
            if r == ResponseType::Accept {
                let filename = d.file().expect("no file wtf").path().expect("no path wtf");

                if let Ok(file) = std::fs::File::open(&filename) {
                    get_glob_mut().load(serde_yaml::from_reader(file).expect("Couldnt't read config"));

                    win_clone.lookup_action("set_state_loaded").expect("faild to get state loader").activate(None);

                    println!("Opened {:?}",filename);

                } else {
                    println!("Couldn't load project from file \"{filename:?}\"!");
                }
            }
        });

        dia.show();
    }

    fn act_save_proj(win: &ApplicationWindow,_: &SimpleAction,_: Option<&Variant>) {
        let glob = get_glob();

        if !glob.is_proj_loaded() {
            let pu = Dialog::with_buttons(Some("Can't save unloaded project"), Some(win), DialogFlags::MODAL, &[("Ok",ResponseType::Ok)]);

            pu.set_width_request(400);
            pu.set_height_request(100);

            let lab = Label::new(Some("Can't save project because no project is loaded"));
            lab.set_vexpand(true);
            lab.set_justify(Justification::Center);

            pu.set_child(Some(&lab));

            pu.show();
            return;
        }

        let filter = FileFilter::new();

        filter.set_name(Some("Mighty-Bitey projects (*.mbproj)"));
        filter.add_pattern("*.mbproj");

        let dia = FileChooserNative::builder()
            .action(FileChooserAction::Save)
            .title("Save project")
            .build();

        dia.set_current_name("project.mbproj");
        dia.add_filter(&filter);

        dia.connect_response(move |d,r| {
            if r == ResponseType::Accept {
                let file = d.file().expect("no file wtf");
                let str = serde_yaml::to_string(&glob.project).expect("couldnt serialize project");

                let outstream = file
                    .create(FileCreateFlags::REPLACE_DESTINATION, None::<&Cancellable>)
                    .expect("no output stream?");

                outstream.write(str.as_bytes(), None::<&Cancellable>).expect("Couldnt write");

                outstream.close(None::<&Cancellable>).expect("couldnt close");

                println!("Saved to {:?}",file.path());
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
        let glob = get_glob();
        if glob.is_proj_loaded() {
            println!("Name: {:?}",glob.project.name);
            println!("Author: {:?}",glob.project.author);

            win.set_title(Some(format!("{} - Mighty-Bitey ROM editor",glob.project.name.clone().unwrap_or("".to_string())).as_str()));
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

        let act_save_proj: ActionEntry<ApplicationWindow> = ActionEntryBuilder::new("save_proj")
            .activate(act_save_proj)
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

        win.add_action_entries([act_close,act_new_proj,act_open_rom,act_save_proj,act_open_proj,act_set_state_loaded]);
    }

}

fn build_menu_model() -> MenuModel {
    let menu = Menu::new();

    let file = Menu::new();

    let new = MenuItem::new(Some("New Proj"), Some("win.new_proj"));
    let open = MenuItem::new(Some("Open Proj"), Some("win.open_proj"));
    let save = MenuItem::new(Some("Save Proj"), Some("win.save_proj"));
    let quit = MenuItem::new(Some("Quit"), Some("win.close"));

    let quitsec = Menu::new();

    file.append_item(&new);
    file.append_item(&open);
    file.append_item(&save);

    quitsec.append_item(&quit);
    file.append_section(None, &quitsec);

    let edit = Menu::new();

    let undo = MenuItem::new(Some("Undo"), None);
    let redo = MenuItem::new(Some("Redo"), None);

    edit.append_item(&undo);
    edit.append_item(&redo);

    let export = Menu::new();

    let exp_rom = MenuItem::new(Some("Build ROM"), None);
    let exp_armips = MenuItem::new(Some("Export Armips script"), None);

    export.append_item(&exp_rom);
    export.append_item(&exp_armips);

    let file_model: MenuModel = file.into();
    let edit_model: MenuModel = edit.into();
    let export_model: MenuModel = export.into();

    menu.append_submenu(Some("File"),&file_model);
    menu.append_submenu(Some("Edit"),&edit_model);
    menu.append_submenu(Some("Export"),&export_model);

    return menu.into();
}

fn make_content_unloaded() -> gtk4::Label {
    let lab = Label::new(Some("Welcome to the Mighty-Bitey ROM editor\nPlease create or open a project"));
    lab.set_vexpand(true);
    lab.set_justify(Justification::Center);

    return lab;
}

fn make_left_pane() -> gtk4::ListView {
    let factory = SignalListItemFactory::new();

    let store = gio::ListStore::new::<StringObject>();
    factory.connect_bind(move |_, item| {
        let string = item.item().unwrap().downcast::<StringObject>().unwrap().string();

        if string != "" {
            let label = Label::new(None);
            item.connect_selected_notify(move |a| {
                let glob = &mut get_glob_mut();
                if a.is_selected() {
                    glob.properties_display.as_ref().expect("no prop display :(").set_visible_child_name("select");
                } else {
                    glob.properties_display.as_ref().expect("no prop display :(").set_visible_child_name("unselect");
                }
            });

            label.set_text(string.as_str());

            item.set_child(Some(&label));
        } else {
            let butt = Button::new();

            butt.set_label("New Change");
            butt.set_halign(Align::Center);

            butt.connect_clicked(move |_| {
                let glob = &mut get_glob_mut();
                glob.add_change(&Change 
                    { 
                        name: "Test change".to_string(),
                        change: ChangeTypeDontUseCuzItsMeantToBeAnonym::Dummy
                    });
            });

            item.set_child(Some(&butt));
        }
    });

    {
        let glob = &mut get_glob_mut();
        for c in &glob.project.changes {
            store.append(&StringObject::new(c.name.as_str()));
        }
        store.append(&StringObject::new(""));
        glob.changes_display = Some(store.clone());
    }

    let smodel: SelectionModel = SingleSelection::new(Some(store)).into();

    let left = ListView::new(Some(smodel),Some(factory));

    left.set_width_request(300);
    left.set_vexpand(true);

    return left;
}

fn make_right_pane() -> gtk4::Stack {
    let toret = Stack::new();

    let unselect = Label::new(Some("Nothing selected :P"));
    unselect.set_vexpand(true);
    unselect.set_halign(Align::Center);

    let select = Label::new(Some("Something selected xD"));
    select.set_vexpand(true);
    select.set_halign(Align::Center);

    toret.add_named(&unselect, Some("unselect"));
    toret.add_named(&select, Some("select"));

    toret.set_visible_child_name("unselect");

    {
        let glob = &mut get_glob_mut();
        glob.properties_display = Some(toret.clone());
    }

    return toret;
}

fn make_content_loaded() -> gtk4::Paned {
    let toret = gtk4::Paned::new(Orientation::Horizontal);

    toret.set_start_child(Some(&make_left_pane()));
    toret.set_end_child(Some(&make_right_pane()));

    return toret;
}

fn make_content() -> gtk4::Box {
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

    win.set_title(Some("Mighty-bitey ROM editor"));
    win.set_default_width(400);
    win.set_default_height(300);
    win.set_show_menubar(true);

    actions::register(&win);

    let content = make_content();
    win.set_child(Some(&content));

    return win;
}