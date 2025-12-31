use std::sync::*;

use gtk4::{gio::prelude::*, *};
use serde::*;

use crate::patching::baserom::*;

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
    pub name: String,
    pub author: String,
    pub baserom_type: BaseromType,
    pub changes: Vec<Change>,
}

#[derive(Debug, Default)]
pub struct Global {
    loaded: bool,
    pub project: Option<Project>,
    pub baserom: Option<Baserom>,
    pub changes_display: Option<gio::ListStore>,
    pub properties_display: Option<gtk4::Stack>,
    pub selected_change_idx: Option<usize>
}

impl Global {
    pub fn create_proj(&mut self, name: &str, author: &str, baserom_type: BaseromType) {
        self.project = Some(Project{
            name: name.to_string(),
            author: author.to_string(),
            baserom_type: baserom_type,
            changes: vec![],
        });
        self.loaded = true;
    }

    pub fn load(&mut self, proj: Project) {
        self.project = Some(proj);
        self.loaded = true;
    }

    pub fn is_proj_loaded(&self) -> bool {
        self.loaded
    }

    //TODO: deduplicate names
    pub fn add_change(&mut self, change: &Change) {
        if self.loaded {
            self.project.as_mut().unwrap().changes.push(change.clone());
            if let Some(changes_display) = &self.changes_display {
                changes_display.remove(changes_display.n_items() - 1);
                changes_display.append(&StringObject::new(&change.name));
                changes_display.append(&StringObject::new(""));
            }
        }
    }

    pub fn delete_change(&mut self, idx: usize) {
        if self.loaded {
            self.project.as_mut().unwrap().changes.remove(idx);
        }
    }

    pub fn search_change(&self, name: &str) -> Option<&Change> {
        if self.loaded {
            for c in &self.project.as_ref().unwrap().changes {
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