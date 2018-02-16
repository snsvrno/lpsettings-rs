//! determines different paths

use std::path::{PathBuf,Path};
use std::env;

pub fn get_global_settings_path() -> PathBuf {
  //! determines what the *global* file path should be.

  let mut home_dir = get_global_settings_folder();
  home_dir.push(super::SETTINGS_FILE);
  home_dir
}

pub fn get_global_settings_folder() -> PathBuf {
  //! builds the *global* settings folder.

  let mut home_dir = env::home_dir().unwrap();
  home_dir.push(super::SETTINGS_FOLDER);
  home_dir
}

pub fn get_local_settings_path() -> PathBuf {
  //! determines what the *local* file path should be.
  
  if let Ok(mut dir) = env::current_dir() {
    dir.push(super::SETTINGS_FILE);
    dir
  } else {
    let mut path = Path::new(".").to_path_buf();
    path.push(super::SETTINGS_FILE);
    path
  }
}
