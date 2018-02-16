//! contains functions to work with the `Setting` type alias

use std::collections::HashMap;

use helper;

type Setting = HashMap<String,HashMap<String,String>>;

pub fn set_value(settings : &mut Setting, key_path : &str, value : &str) -> bool {
  //! set the value of a `Setting` type

  if let Some((base,key)) = helper::split_key(key_path) {
    let mut empty_option : Option<HashMap<String,String>> = None;

    // gets a mutable reference to the sub HashMap and adds to it.
    // if there is not sub HashMap then it creates a new hashmap and 
    // adds it to the `empty_option` var to be inserted later
    match settings.get_mut(&base.to_string()) {
      Some(ref mut hash) => { hash.insert(key.to_string(),value.to_string()); return true; }
      None => { 
        let mut hash : HashMap<String,String> = HashMap::new();
        hash.insert(key.to_string(),value.to_string());
        empty_option = Some(hash);
      }
    }

    // if the sub Hashmap didn't exist we can add it here.
    if let Some(hash) = empty_option { settings.insert(base.to_string(),hash); return true; }

  }
  false
}

pub fn get_value(settings : &Setting, key_path : &str) -> Option<String> {
  //! get the value of a `Setting` type.

  if let Some((base,key)) = helper::split_key(key_path) {
    if let Some(section) = settings.get(base) {
      if let Some(value) = section.get(key) { return Some(value.clone()); }
    }
  }
  None
}