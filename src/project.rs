use std::{sync::{Mutex, MutexGuard, OnceLock}};

use gtk4::{gio::prelude::*, *};
use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChangeTypeDontUseCuzItsMeantToBeAnonym {
    Dummy //for testing, still TODO
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Change {
    pub name: String,
    pub change: ChangeTypeDontUseCuzItsMeantToBeAnonym
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Project {
    pub name: Option<String>,
    pub author: Option<String>,
    pub rom_in_hash: Option<String>,
    pub changes: Vec<Change>,
}

#[derive(Debug, Default)]
pub struct Global {
    loaded: bool,
    pub project: Project,
    pub changes_display: Option<gio::ListStore>,
    pub properties_display: Option<gtk4::Stack>,
    pub selected_change_idx: Option<usize>
}

impl Global {
    pub fn create_proj(&mut self, name: &str, author: &str, in_hash: &str) {
        self.project.name = Some(name.to_string());
        self.project.author = Some(author.to_string());
        self.project.rom_in_hash = Some(in_hash.to_string());
        self.project.changes = vec![];
        self.loaded = true;
    }

    pub fn load(&mut self, proj: Project) {
        self.project = proj;
        self.loaded = true;
    }

    pub fn is_proj_loaded(&self) -> bool {
        self.loaded
    }

    //TODO: deduplicate names
    pub fn add_change(&mut self, change: &Change) {
        if self.loaded {
            self.project.changes.push(change.clone());
            if let Some(changes_display) = &self.changes_display {
                changes_display.remove(changes_display.n_items() - 1);
                changes_display.append(&StringObject::new(&change.name));
                changes_display.append(&StringObject::new(""));
            }
        }
    }

    pub fn delete_change(&mut self, idx: usize) {
        if self.loaded {
            self.project.changes.remove(idx);
        }
    }

    pub fn search_change(&self, name: &str) -> Option<&Change> {
        if self.loaded {
            for c in &self.project.changes {
                if c.name == name {
                    return Some(c);
                };
            };
        };

        return None;
    }

}

//hella not thread safe but make the code *PRETTY*
static mut GLOB: OnceLock<Mutex<Global>> = OnceLock::new();

#[allow(static_mut_refs)]
pub fn init_glob(){
    unsafe {
        GLOB.set(Mutex::new(Global::default())).expect("Couldn't initialize project")
    }
}

#[allow(static_mut_refs)]
pub fn get_glob<'a>() -> MutexGuard<'a,Global>{
    unsafe {
        GLOB.get().unwrap().lock().unwrap()
    }
}