//! determines different paths

use std::path::{PathBuf,Path};
use std::env;
use std::ffi;

use ansi_term::Colour::Yellow;

fn get_settings_file_name() -> String {
  match env::var("LPSETTINGS_FILE") {
    Err(_) => super::SETTINGS_FILE.to_string(),
    Ok(val) => val.to_string(),
  }
}

fn get_settings_folder_name() -> String {
  match env::var("LPSETTINGS_USERFOLDER") {
    Err(_) => super::SETTINGS_FOLDER.to_string(),
    Ok(val) => val.to_string(),
  }
}

pub fn get_global_settings_path() -> PathBuf {
  //! determines what the *global* file path should be.

  let mut home_dir = get_global_settings_folder();
  home_dir.push(get_settings_file_name());
  home_dir
}

pub fn get_global_settings_folder() -> PathBuf {
  //! builds the *global* settings folder.

  let mut home_dir = env::home_dir().unwrap();
  home_dir.push(get_settings_folder_name());
  home_dir
}

pub fn get_local_settings_path() -> PathBuf {
  //! determines what the *local* file path should be.
  
  if let Ok(mut dir) = env::current_dir() {
    dir.push(get_settings_file_name());
    dir
  } else {
    let mut path = Path::new(".").to_path_buf();
    path.push(get_settings_file_name());
    path
  }
}

pub fn check_if_a_path(string : &str) -> bool {
  string.contains("/") || string.contains("\\")
}

pub fn get_absolute_path_from_str(string : &str) -> PathBuf {
  let path = PathBuf::from(string);
  if path.is_absolute() { return path; }

  match env::current_dir() {
    Err(error) => { output_error!("Cannot access current directory: {}",Yellow.paint(error.to_string())); }
    Ok(mut path) => {
      path.push(string);
      return compress_path(&path);
    }
  }

  path
}

fn compress_path(path : &PathBuf) -> PathBuf{
  //! removes the ../../../ sections in a path

  let mut new_string : String = String::new();
  let mut parts : Vec<ffi::OsString> = Vec::new();
  
  for part in path.iter() {
    if part == ".." { parts.pop(); }
    else if part == "\\" { }
    else { parts.push(part.to_os_string()); }
  }

  for part in parts { new_string = format!("{}{}\\",new_string,part.to_str().unwrap()); }

  PathBuf::from(new_string)
}