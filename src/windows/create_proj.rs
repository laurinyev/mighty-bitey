use std::{rc::*,cell::*};
use gtk4::{prelude::*, *};

use crate::project::*;

fn make_content(mainwin: gtk4::ApplicationWindow,popupwin: gtk4::ApplicationWindow) -> gtk4::Box {
    //main layers
    let winchild = gtk4::Box::new(Orientation::Vertical, 0);

    //inputs
    let inp_name = Entry::new();
    inp_name.set_placeholder_text(Some("Name"));
    let inp_author = Entry::new();
    inp_author.set_placeholder_text(Some("Author"));
    let inp_dir = Entry::new();
    inp_dir.set_placeholder_text(Some("Project DIR"));


    //close button
    let close = Button::with_label("Close");
    close.set_halign(Align::End);
    close.set_margin_bottom(10);
    close.set_margin_end(10);
    let popupwin_clone = popupwin.clone();
    close.connect_clicked(move |_| {
        popupwin_clone.close();
    });

    //continue button
    let cont = Button::with_label("Create project");
    cont.set_halign(Align::End);
    cont.set_margin_bottom(10);
    cont.set_margin_end(10);

    let inp_name_clone = inp_name.clone();
    let inp_author_clone = inp_author.clone();
    let inp_dir_clone = inp_dir.clone();

    cont.connect_clicked(move |_| {
        let app = mainwin.application().expect("no application?");
        {
            get_proj_mut(&app).load(
                &inp_dir_clone.text(),
                &inp_name_clone.text(),
                &inp_author_clone.text(),
                "",
                "",
                vec![]
            );
        }

        mainwin.lookup_action("set_state_loaded").expect("faild to get state loader").activate(None);
        popupwin.close();
    });

    //bottom bar
    let bottombar = gtk4::Box::new(Orientation::Horizontal, 0);
    bottombar.set_halign(Align::End);
    bottombar.append(&close);
    bottombar.append(&cont);

    winchild.append(&inp_name);
    winchild.append(&inp_author);
    winchild.append(&inp_dir);
    winchild.append(&bottombar);
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