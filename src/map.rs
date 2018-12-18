use version_lp::Version;
use failure::Error;
use toml;

#[derive(Deserialize)]
pub struct OptionsMap {
    pub key : String,
    pub desc : String,

    pub added : Option<Version>,
    pub removed : Option<Version>,

    pub replaced_by : Option<String>,

    pub init : Option<bool>,
}

#[derive(Deserialize)]
struct Map {
    pub values : Vec<OptionsMap>
}

pub fn create_options_map() -> Result<Vec<OptionsMap>,Error> {

    let map_raw = include_str!("map.toml");
    let map : Map = toml::de::from_str(map_raw)?;
    Ok(map.values)
}

pub fn get_user_input(question : &str) -> String {
  //! shortcut function used to get user input.
  //! 
  //! will take the `question` and print to the screen expecting user input.
  //!
  //! ```rust
  //! get_user_input("Some question for the user");
  //! ```
  //!
  //! will yield
  //!
  //! ```bash
  //! Some question for the user:
  //! ```

  use std::io::{stdin,stdout,Write};

  let mut input = String::new();
  {
    let mut in_put = &mut input;
    print!("{}: ",question);
    let _ = stdout().flush();
    stdin().read_line(&mut in_put).expect("");
  }

  // trims the \r\n at the end of the string.
  input = input.replace("\r","").replace("\n","");
  
  input
}