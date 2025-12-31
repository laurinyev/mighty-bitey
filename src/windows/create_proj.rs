use gtk4::{prelude::*, *};

use crate::global::*;

fn make_content(mainwin: gtk4::ApplicationWindow,popupwin: gtk4::ApplicationWindow) -> gtk4::Box {
    //main layers
    let winchild = gtk4::Box::new(Orientation::Vertical, 0);

    //inputs
    let inp_name = Entry::new();
    inp_name.set_placeholder_text(Some("Name"));
    let inp_author = Entry::new();
    inp_author.set_placeholder_text(Some("Author"));

    let store = gio::ListStore::new::<StringObject>();

    store.append(&StringObject::new("Japanese"));
    store.append(&StringObject::new("English 1.0"));
    store.append(&StringObject::new("English 1.1"));
    store.append(&StringObject::new("English 1.2"));
    store.append(&StringObject::new("English 1.3"));

    let baseromtype = DropDown::builder()
        .name("ROM type")
        .model(&store)
        .selected(4)
        .build();

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
    let baseromtype_clone = baseromtype.clone();

    cont.connect_clicked(move |_| {
        get_glob().create_proj(
            &inp_name_clone.text(),
            &inp_author_clone.text(),
            match baseromtype_clone.selected() {
                0 => crate::patching::baserom::BaseromType::JP,
                1 => crate::patching::baserom::BaseromType::En1_0_0,
                2 => crate::patching::baserom::BaseromType::En1_1_0,
                3 => crate::patching::baserom::BaseromType::En1_2_0,
                4 => crate::patching::baserom::BaseromType::En1_3_0,
                _ => crate::patching::baserom::BaseromType::En1_3_0
            }
        );

        let resp = alerta::alerta()
            .title("Base ROM")
            .message("You must load a base ROM for supplying default assets/values!")
            .icon(alerta::Icon::Info)
            .button_preset(alerta::ButtonPreset::OkCancel)
            .show()
            .expect("Couldn't show popup");

        if resp == alerta::Answer::Button(0) { // 1 = OK button
            mainwin.lookup_action("open_rom").expect("faild to get state loader").activate(None);
        }

        popupwin.close();
    });

    //bottom bar
    let bottombar = gtk4::Box::new(Orientation::Horizontal, 0);
    bottombar.set_halign(Align::End);
    bottombar.append(&close);
    bottombar.append(&cont);

    winchild.append(&inp_name);
    winchild.append(&inp_author);
    winchild.append(&baseromtype);
    winchild.append(&bottombar);
    winchild.set_vexpand(true);

    return winchild;
}

pub fn build(app: &gtk4::Application,mainwin: &gtk4::ApplicationWindow) -> ApplicationWindow{
    let win = ApplicationWindow::new(app);

    win.set_title(Some("Create New Project"));
    win.set_default_width(300);
    win.set_default_height(300);

    let content = make_content(mainwin.clone(),win.clone());
    win.set_child(Some(&content));

    return win;
}