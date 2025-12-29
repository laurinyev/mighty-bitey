use std::{sync::*};

#[derive(Debug)]
pub enum Change {
    
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
    fn new(&mut self, project_dir: &str, name: &str, author: &str, in_hash: &str, out_name: &str) {
        self.project_dir = Some(project_dir.to_string());
        self.name = Some(name.to_string());
        self.author = Some(author.to_string());
        self.rom_in_hash = Some(in_hash.to_string());
        self.rom_out_name = Some(out_name.to_string());
        self.loaded = true;

    }
}

pub static PROJ: OnceLock<Project> = OnceLock::<Project>::new();



