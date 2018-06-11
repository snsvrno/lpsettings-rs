//! ## LPSETTINGS
//! ***lpsettings*** is a library design to access and edit *lovepack* settings. It has the ability to load *local* settings and *global* settings, or a stacked combination of the both.
//!
//! Looks for a `lovepack.toml` file by default.
//!
//! This scope contains the highest level functions allowing for easy integration into other libraries and projects.

// external crates
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate clap;
extern crate ansi_term;
#[macro_use]
extern crate output;

// standard library
use std::env;
use std::path::PathBuf;

// lp crates
extern crate lperror;

// internal modules
pub mod interface;
mod structs; use structs::settings::Settings;
mod initalize;
mod paths;

// constant for where and what file to use for the settings
// currently not override-able
pub static SETTINGS_FOLDER : &str = ".lovepack"; // the folder in user_folder to use
pub static SETTINGS_FILE : &str = "lovepack.toml"; // the filename for the settings, both global and local

pub fn initalize() {
  //! initalizes a default settings file at the environmentally determined location.

  let path = get_path();
  initalize::create_default_settings_input(&path);
}

pub fn get_settings_folder_path() -> Result<PathBuf,lperror::LovepackError> {
  //! returns the *PathBuf* for the folder in which the settings are located.

  let folder_path = paths::get_global_settings_folder();

  if !folder_path.exists() { 
    match std::fs::create_dir_all(&folder_path) {
      Ok(_) => { 
        // create the folder(s) and returning the PathBuf
        return Ok(folder_path); 
      },
      Err(error) => { 
        // failed to create the folder for some reason
        return Err(lperror::LovepackError::Error(error.to_string())); 
      }
    }
  }

  Ok(folder_path)
}

pub fn get_value(key_path:&str) -> Option<String> {
  //! gets the value from the environemntally determined location

  let raw_settings : structs::settings::Settings = get_raw_settings();
  return raw_settings.get_value(&key_path);
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
  //!
  //! tries to set the value to the environmentally determined location, if none is defined it will default to global

  let path = get_path();
  let mut value = value.to_string();

  // process the path to give the complete absolute path with expansions (.., ~, etc...)
  if paths::check_if_a_path(&value) {
    let abs_path = paths::get_absolute_path_from_str(&value);
    value = abs_path.display().to_string();
  }

  let mut settings : Settings = Settings::load_from_or_empty(&path);
  settings.set_value(&key_path, &value);

  match settings.save_to(&path) {
    Ok(_) => { return true; }
    Err(lperror) => { output_error!("{}",lperror); return false; }
  }
}

pub fn set_value_local(key_path : &str, value : &str) -> bool {
  //! sets the value to the local settings path.

  let path = paths::get_local_settings_path();

  let mut settings : Settings = Settings::load_from_or_empty(&path);
  settings.set_value(&key_path, &value);

  match settings.save_to(&path) {
    Ok(_) => { return true; }
    Err(lperror) => { output_error!("{}",lperror); return false; }
  }
}

pub fn set_value_global(key_path : &str, value : &str) -> bool {
  //! sets the value to the global settings path.

  let path = paths::get_global_settings_path();

  let mut settings : Settings = Settings::load_from_or_empty(&path);
  settings.set_value(&key_path, &value);

  match settings.save_to(&path) {
    Ok(_) => { return true; }
    Err(lperror) => { output_error!("{}",lperror); return false; }
  }
}

pub fn get_raw_local(key_path : Option<&str>) -> Option<structs::subsetting::Subsetting> {
  //! returns the raw local Settings Tree as a *SubSetting* struct

  let path = paths::get_local_settings_path();

  let raw_settings : Settings = Settings::load_from_or_empty(&path);

  match key_path {
    None => { return Some(raw_settings.as_subsetting_consume()); }
    Some(key_path) => { return raw_settings.get_raw(&key_path); }
  }
}

pub fn get_raw(key_path : Option<&str>) -> Option<structs::subsetting::Subsetting> {
  //! returns the substring with respect to the keypath.

  let raw_settings : structs::settings::Settings = get_raw_settings();
  match key_path {
    None => { return Some(raw_settings.as_subsetting_consume()); }
    Some(key_path) => { return raw_settings.get_raw(&key_path); }
  }
}

fn get_path() -> PathBuf {
  //! determines what path to use based on the environmental variables.
  //! 
  //! will check if the *--global* or *--local* parameter was passed and then returns the correct *PathBuf*
  //!
  //! first checks if the environmental variable of *LOVEPACK_SETTINGS_LOCATION* was set, if it was it will check if it is either *local* or *global*
  //! then it it doesn't match it will default to the global location.

  if let Ok(value) = env::var("LOVEPACK_SETTINGS_LOCATION") { 
    if value == "global" { paths::get_global_settings_path() }
    else if value == "local" { paths::get_local_settings_path() }
    else { paths::get_global_settings_path() }
  } else { paths::get_global_settings_path() }
}

fn get_raw_settings() -> structs::settings::Settings {
  //! gets the raw settins object from the environmentally determined location, can combine settings
  //!
  //! will first look for the environmental varialbe *LOVEPACK_SETTINGS_LOCATION* to determine what settings
  //! to use, if it can't find either then it will attempt to combine the two locations into one settings
  //! object

  let path_global : PathBuf = paths::get_global_settings_path();
  let path_local : PathBuf = paths::get_local_settings_path();

  if let Ok(value) = env::var("LOVEPACK_SETTINGS_LOCATION") { 
    if value == "global" { 
    output_debug!("Using global settings.");

      // only the global setting
      return Settings::load_from_or_empty(&path_global);
    }
    if value == "local" {
    output_debug!("Using local settings.");

      // only the local one 
      return Settings::load_from_or_empty(&path_local);
    }
    return Settings::new();
  } else {
    output_debug!("Using combined settings.");
    
    // load a combined setting to get the right value
    let settings_local : Settings = Settings::load_from_or_empty(&path_local);
    let mut settings_global : Settings = Settings::load_from_or_empty(&path_global);
    settings_global += settings_local;
    
    return settings_global;
  }
}