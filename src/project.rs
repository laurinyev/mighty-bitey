use std::{cell::RefCell, rc::Rc, sync::OnceLock};

use gtk4::{Application, glib::object::ObjectExt};

#[derive(Debug)]
pub enum ChangeTypeDontUseCuzItsMeantToBeAnonym {
    //todo: cange types
}

#[derive(Debug)]
pub struct Change {
    name: String,
    change: ChangeTypeDontUseCuzItsMeantToBeAnonym
}

#[derive(Debug, Default)]
pub struct Project {
    loaded: bool,
    pub project_dir: Option<String>,
    pub name: Option<String>,
    pub author: Option<String>,
    pub rom_in_hash: Option<String>,
    pub rom_out_name: Option<String>,
    pub changes: Vec<Change>
}

impl Project {
    pub fn load(&mut self, project_dir: &str, name: &str, author: &str, in_hash: &str, out_name: &str, changes: Vec<Change>) {
        self.project_dir = Some(project_dir.to_string());
        self.name = Some(name.to_string());
        self.author = Some(author.to_string());
        self.rom_in_hash = Some(in_hash.to_string());
        self.rom_out_name = Some(out_name.to_string());
        self.changes = changes;
        self.loaded = true;
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }
}

//hella not thread safe but make the code *PRETTY*
static mut PROJ: OnceLock<RefCell<Project>> = OnceLock::new();

#[allow(static_mut_refs)]
pub fn init_proj(){
    unsafe {
        PROJ.set(RefCell::new(Project::default())).expect("Couldn't initialize project")
    }
}

#[allow(static_mut_refs)]
pub fn get_proj<'a>(_: &'a Application) -> std::cell::Ref<'a, Project> {
    unsafe {
        PROJ.get().expect("no project?").borrow()
    }
}

#[allow(static_mut_refs)]
pub fn get_proj_mut<'a>(_: &'a Application) -> std::cell::RefMut<'a, Project> {
    unsafe {
        PROJ.get().expect("no project?").borrow_mut()
    }
}