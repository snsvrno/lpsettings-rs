//! ## LPSETTINGS
//! ***lpsettings*** is a library design to access and edit *lovepack* settings. It has the ability to load *local* settings and *global* settings, or a stacked combination of the both.
//!
//! Looks for a `lovepack.toml` file by default.
//!
//! This scope contains the highest level functions allowing for easy integration into other libraries and projects.

#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate clap;
extern crate ansi_term;
#[macro_use]
extern crate output;

use std::env;
use std::path::PathBuf;

pub mod interface;
mod structs;
mod initalize;
mod paths;

use structs::settings::Settings;

static SETTINGS_FOLDER : &str = ".lovepack";
static SETTINGS_FILE : &str = "lovepack.toml";

pub fn initalize() {
  //! initalizes a default settings file at the environmentally determined location

  let path = get_path_complex();
  initalize::create_default_settings_input(&path);
}

pub fn get_settings_folder() -> Result<PathBuf,()> {
  let folder = paths::get_global_settings_folder();

  if !folder.exists() { 
    match std::fs::create_dir_all(&folder) {
      Ok(_) => { return Ok(folder); }
      Err(_) => { return Err(()); }
    }
  }

  Ok(folder)
}

pub fn get_value(key_path:&str) -> Option<String> {
  //! gets the value from the environemntally determined location

  let path_global : PathBuf = paths::get_global_settings_path();
  let path_local : PathBuf = paths::get_local_settings_path();

  if let Ok(value) = env::var("LOVEPACK_SETTINGS_LOCATION") { 
    if value == "global" { 
    output_debug!("Using global settings.");

      // only the global setting
      let settings : Settings = Settings::load_from_or_empty(&path_global);
      return settings.get_value(&key_path);
    }
    if value == "local" {
    output_debug!("Using local settings.");

      // only the local one 
      let settings : Settings = Settings::load_from_or_empty(&path_local);
      return settings.get_value(&key_path);
    }
    return None;
  } else {
    output_debug!("Using combined settings.");
    
    // load a combined setting to get the right value
    let settings_local : Settings = Settings::load_from_or_empty(&path_local);
    let mut settings_global : Settings = Settings::load_from_or_empty(&path_global);
    settings_global += settings_local;
    
    return settings_global.get_value(&key_path);
  }
}

pub fn has_value(key_path:&str) -> bool {
  //! checks if a value exists
  
  match get_value(&key_path) {
    Some(_) => { true },
    None => { false }
  }
}

pub fn get_value_or(key_path : &str, default_value : &str) -> String {
  //! returns the value or the supplied default value.

  match get_value(&key_path) {
    Some(value) => { value }
    None => { default_value.to_string() }
  }
}

pub fn set_value(key_path : &str, value : &str) -> bool {
  //! sets the value to the environmentally determined location

  let path = get_path_complex();

  let mut settings : Settings = Settings::load_from_or_empty(&path);
  settings.set_value(&key_path, &value);

  settings.save_to(&path)
}

pub fn set_value_local(key_path : &str, value : &str) -> bool {
  let path = paths::get_local_settings_path();

  let mut settings : Settings = Settings::load_from_or_empty(&path);
  settings.set_value(&key_path, &value);

  settings.save_to(&path)
}

pub fn set_value_global(key_path : &str, value : &str) -> bool {
  let path = paths::get_global_settings_path();

  let mut settings : Settings = Settings::load_from_or_empty(&path);
  settings.set_value(&key_path, &value);

  settings.save_to(&path)
}

fn get_path_complex() -> PathBuf {
  //! determines what path to use based on the environmental variables.

  if let Ok(value) = env::var("LOVEPACK_SETTINGS_LOCATION") { 
    if value == "global" { paths::get_global_settings_path() }
    else if value == "local" { paths::get_local_settings_path() }
    else { paths::get_global_settings_path() }
  } else { paths::get_global_settings_path() }
}