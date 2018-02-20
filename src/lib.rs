//! ## LPSETTINGS
//! ***lpsettings*** is a library design to access and edit *lovepack* settings. It has the ability to load *local* settings and *global* settings, or a stacked combination of the both.
//!
//! Looks for a `lovepack.toml` file by default.
//!
//! This scope contains the highest level functions allowing for easy integration into other libraries and projects.
extern crate toml;
extern crate clap;
extern crate ansi_term;

use std::env;
use std::io::Error;
use std::path::PathBuf;

pub mod interface;
mod initalize;
mod io;
mod helper;
mod paths;
mod setting;

static SETTINGS_FOLDER : &str = ".lovepack";
static SETTINGS_FILE : &str = "lovepack.toml";

pub fn initalize() {
  //! initalizes a default settings file at the environmentally determined location

  let path = get_path_complex();
  initalize::create_default_settings_input(&path);
}

pub fn get_settings_folder() -> Result<PathBuf,Error> {
  let folder = paths::get_global_settings_folder();

  if !folder.exists() { 
    match std::fs::create_dir_all(&folder) {
      Ok(_) => { return Ok(folder); }
      Err(error) => { return Err(error); }
    }
  }

  Ok(folder)
}

pub fn get_value(key_path:&str) -> Option<String> {
  //! gets the value from the environemntally determined location

  match io::load_settings_map() {
    Err(_) => { None }
    Ok(settings) => { setting::get_value(&settings,&key_path) }
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

  match io::load_settings_raw_or_empty(&path) {
    Err(_) => { return false; }
    Ok(mut settings) => {
      setting::set_value(&mut settings, &key_path, &value);

      match io::save_settings_map(&settings,&path) {
        Err(_) => { return false; }
        Ok(_) => { return true; }
      }
    }

  }
}

fn get_path_complex() -> PathBuf {
  //! determines what path to use based on the environmental variables.

  if let Ok(value) = env::var("LOVEPACK_SETTINGS_LOCATION") { 
    if value == "global" { paths::get_global_settings_path() }
    else if value == "local" { paths::get_local_settings_path() }
    else { paths::get_global_settings_path() }
  } else { paths::get_global_settings_path() }
}