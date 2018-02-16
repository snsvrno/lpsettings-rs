//!  collection of small helper functions used throughout

use std::collections::HashMap;

type Setting = HashMap<String,HashMap<String,String>>;

pub fn add_to(original : &mut Setting, new : Result<Setting,&'static str>) {
  //! adds one setting object to another
  //!
  //! a setting object is a type alias for a complex HashMap in HashMop setup used to describe the settings.
  //!
  //! ```rust
  //! type Setting = HashMap<String,HashMap<String,String>>;
  //! ```
  //!
  //! `add_to` will take the values inside the `new` hashmap and will write them into the `original` map, overwriting any duplicate values.
  //! This function returns nothing as the operation is applied to the mutable reference `original`.

  // checks if there is actually a second `Setting` supplied. 
  if let Ok(new) = new {
    for (main_key,main_hash) in new.iter() {
      // first will check if the sub HashMap exists inside the `original`. If it doesn't then it must create it.
      match original.get(&main_key.to_string()) {
        None => {
          let mut hash : HashMap<String,String> = HashMap::new();
          for (key,value) in main_hash.iter() { hash.insert(key.to_string(),value.to_string()); }
          original.insert(main_key.to_string(),hash);
        },
        _ => { }
      }
      // now that we confirmed that a hashmap exists inside `original` we can write the new values to it.
      match original.get_mut(&main_key.to_string()) {
        None => { },
        Some(ref mut hash) => { 
          for (key,value) in main_hash.iter() {
            // insert will update the values (overwrite them) so all values from the `original` will be overwritten with the `new` values if present.
            // there are no checks 
            hash.insert(key.to_string(),value.to_string());
          }
        }
      }
    }
  }

}

pub fn split_key(key_path : &str) -> Option<(&str,&str)> {
  //! splits a "a.b" into an option wrapped tuple (a,b)

  let slice = key_path.split(".").collect::<Vec<&str>>();
  if slice.len() == 2 {
    return Some((slice[0],slice[1]));
  } else { return None; }
}