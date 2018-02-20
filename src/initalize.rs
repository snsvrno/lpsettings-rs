//!  contains functions with respect to initalizing a settings configuration on the disk.

use std::path::PathBuf;
use std::collections::HashMap;

use io;
use setting;
use ansi_term::Colour;

type Setting = HashMap<String,HashMap<String,String>>;

pub fn create_default_settings_input(path : &PathBuf) -> bool {
  //! creates a default settings file at the `path` with the help of user input.
  //!
  //! the app will ask cli questions to fill in some of the required default data.

  let mut default_settings : Setting = HashMap::new();
  build_inital_settings(&mut default_settings);

  match io::save_settings_map(&default_settings,&path) {
    Err(_) => { false },
    Ok(_) => { true }
  }
}

fn get_user_input(question : &str) -> String {
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
    print!("{}: ",Colour::Yellow.paint(question));
    let _ = stdout().flush();
    stdin().read_line(&mut in_put).expect("");
  }

  // trims the \r\n at the end of the string.
  input = input.replace("\r","").replace("\n","");
  
  input
}

fn build_inital_settings(mut settings : &mut Setting) {
  //! asks all the right questions to build a `Setting`.
  
  for s in &["user.name","user.email"] {
    setting::set_value(&mut settings,&s,&get_user_input(&s));
  }
}