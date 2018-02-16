//!  collection of io commands for interacting with settings files on the disk 

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;

use toml;

use paths;
use helper;

type Setting = HashMap<String,HashMap<String,String>>;

pub fn load_settings_raw(src : &PathBuf) -> Result<Setting,&'static str> {
  //! loads `Setting` directly from a file.
  //!
  //! A bypass function that doesn't check where the file is, instead will load directly from the supplied path.
  //! Expects a `path` with complete filename and extension.

  let mut buf : String = String::new();
  match File::open(src) {
    Err(_) => { return Err("Can't open settings file"); }
    Ok(mut file) => { 
      match file.read_to_string(&mut buf) {
        Ok(_) => { },
        Err(_) => { return Err("Cannot read file to buffer."); }
      } 
    }
  }

  if buf.len() > 0 {
    match toml::from_str(&buf) {
      Err(_) => { return Err("Can't process string buffer to TOML"); }
      Ok(settings) => {
        return Ok(settings);
      }
    }
  }

  Err("Reached end of load_settings_raw wihtout result")
}

pub fn load_settings_raw_or_empty(src : &PathBuf) -> Result<Setting,&'static str> {
  //! a wrapper for `load_settings_raw` that will return an empty `Setting` if failed to load.
  //!
  //! Expects a `path` with complete filename and extension.

  match load_settings_raw(&src) {
    Err(_) => { 
      let empty_settings : Setting = HashMap::new();
      Ok(empty_settings)
    }
    Ok(settings) => { Ok(settings) }
  }
}

pub fn save_settings_map(settings : &Setting, path : &PathBuf) -> Result<(),&'static str> {
  //! saves the `Setting` object to the given `path`
  //!
  //! Expects a `path` with complete filename and extension.

  let settings_string = toml::to_string(&settings).unwrap();

  let file = File::create(path);
  match file {
    Err(_) => { return Err("Error creating file."); }
    Ok(mut file) => {
      match file.write_all(settings_string.as_bytes()) {
        Err(_) => { return Err("Error writting buffer to file"); }
        Ok(_) => { return Ok( () ); }
      }
    }
  }
}


pub fn load_settings_map() -> Result<Setting,&'static str> {
  //! high level load function that checks env variables to determine where to load.
  //!
  //! Loads the *local* file, the *global* file, or a combined form based on the environment settings.

  if let Ok(value) = env::var("LOVEPACK_SETTINGS_LOCATION") { 
    if value == "global" { return load_settings_map_global(); }
    else if value == "local" { return load_settings_map_local(); }
  } 
  load_settings_map_combined()
}

pub fn load_settings_map_local() -> Result<Setting,&'static str> {
  //! loads the *local* settings file

  let path = paths::get_local_settings_path();
  return load_settings_raw(&path);
}

pub fn load_settings_map_global() -> Result<Setting,&'static str> {
  //! loads the *global* settings file

  let path = paths::get_global_settings_path();
  return load_settings_raw(&path);
}

pub fn load_settings_map_combined() -> Result<Setting,&'static str> {
  //! loads a combined version of the *local* and *global* settings file.
  //!
  //! *local* settings will override `global` settings.
  let mut settings_map : Setting = HashMap::new();
  
  helper::add_to(&mut settings_map,load_settings_map_global());
  helper::add_to(&mut settings_map,load_settings_map_local());

  Ok(settings_map)
}