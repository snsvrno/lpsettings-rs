use std::ops::{Add,AddAssign};
use std::io::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::File;

use structs::subsetting::Subsetting;

use toml;
use ansi_term::Colour::{Red,Yellow};

#[derive(Serialize,Deserialize)]
pub struct Settings {
  parts : HashMap<String,Subsetting>
}

impl Settings {
  pub fn new() -> Settings { Settings { parts : HashMap::new() } }
  pub fn load_from(path : &PathBuf) -> Result<Settings,()> {

    // loads the raw file into a buffer
    let mut buf : String = String::new();
    match File::open(&path) {
      Err(error) => { output_error!("Cannot open settings file {}: {}",Red.paint(path.display().to_string()),Yellow.paint(error.to_string())); return Err(()); }
      Ok(mut file) => { 
        match file.read_to_string(&mut buf) {
          Ok(_) => { },
          Err(error) => { output_error!("Cannot read file {}: {}",Red.paint(path.display().to_string()),Yellow.paint(error.to_string())); return Err(()); }
        } 
      }
    }

    // parses the string
    if buf.len() > 0 {
      let hash : Result<HashMap<String,Subsetting>,_> = toml::from_str(&buf);
      match hash {
        Err(error) => { output_error!("Error processing toml buffer: {}",Yellow.paint(error.to_string())); return Err( () ); }
        Ok(parts) => { return Ok(Settings { parts: parts }); }
      }
    } else { Err(()) }
  }
  pub fn load_from_or_empty(path : &PathBuf) -> Settings {
    match Settings::load_from(&path) {
      Ok(settings) => { settings }
      Err(_) => { Settings::new() }
    }
  }

  pub fn get_value(&self, key : &str) -> Option<String> {
    let path_tree : Vec<&str> = key.split(".").collect();

    let mut subtree : &Subsetting = &Subsetting::Single("Empty".to_string());

    for i in 0..path_tree.len() {
      if i == 0 { 
        if let Some(ref part) = self.parts.get(&path_tree[i].to_string()) {
          subtree = part;
        } else { return None }
      } else {
        match *subtree {
          Subsetting::Complex(ref hash) => { 
            if let Some(ref part) = hash.get(&path_tree[i].to_string()) {
              subtree = part;
            } else { return None }
          },
          _ => { return None }
        }
      }
    }

    match *subtree {
      Subsetting::Complex(ref _hash) => { return Some("is complex".to_string()); },
      Subsetting::Single(ref string) => { return Some(string.to_string()); }
    }
  }

  pub fn set_value(&self, key : &str, value : &str) {

  }
}

impl Add for Settings {
  type Output = Settings;

  fn add(self, other: Settings) -> Settings {
    Settings::new()
  }
}

impl AddAssign for Settings {
  fn add_assign(&mut self, other:Settings) {
    *self = other;
  }
}