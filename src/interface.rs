//! organzies all the functions used to interface with a cli application
//! provides a ready made CLAP app for easy integration

use clap;
use std::env;
use ansi_term::Colour;

pub fn process(matches : &clap::ArgMatches) -> Result<(),&'static str> {
  //! process function to be used with [CLAP.RS](https://clap.rs/)'s `.get_matches()`.
  //!
  //! should be called with the subset of matches from clap's `.get_matches()` if used as a subcommand, or all the matches if used as the main app.
  //!
  //! ```rust
  //! // example of using as a subcommand, this is called after .get_matches() 
  //! match app.subcommand() {
  //!   ("settings", Some(matches)) => { interface::process(matches); },
  //!   _ => {},
  //! }
  //! ```

  // checks where it should perform the commands. these are not compatible so 
  // both should never run at the same time.
  if matches.is_present("local") { env::set_var("LOVEPACK_SETTINGS_LOCATION","local"); }
  if matches.is_present("global") { env::set_var("LOVEPACK_SETTINGS_LOCATION","global"); }
  
  // checks if it should run the initalization instead
  if matches.is_present("init") { super::initalize(); }

  let mut new_value : Option<String> = None;
  let mut key : Option<String> = None;

  if let Some(in_key) = matches.value_of("KEY") { key = Some(in_key.to_string()); }
  if let Some(in_value) = matches.value_of("VALUE") { new_value = Some(in_value.to_string()); }

  // checks based on the Options if it needs to get or set a value.
  if key.is_some() && new_value.is_none() { display_value(key.as_ref().unwrap()); }
  if key.is_some() && new_value.is_some() { set_value(key.as_ref().unwrap(),new_value.as_ref().unwrap()); }

  // success!
  Ok( () )
}

fn display_value(key : &str) {
  //! displays the value to the cli
  if let Some(value) = super::get_value(&key) { println!("{}: {}",Colour::Yellow.paint(key),Colour::Blue.paint(value));} 
  else { println!("{} is not defined",Colour::Red.paint(key)); }
}

fn set_value(key : &str, value: &str) {
  //! sets the value
  let which_setting_string : String = if let Ok(wheres) = env::var("LOVEPACK_SETTINGS_LOCATION") { (wheres + "ly").to_string() } else { "globally".to_string() };

  if super::set_value(&key,&value) { println!("Set {} to {} {}",Colour::Yellow.paint(key),Colour::Green.paint(value),which_setting_string); }
  else { println!("Error setting \'{}\'",Colour::Yellow.paint(key)); }
}


pub fn app() -> clap::App<'static,'static> {
  //! [CLAP.RS](https://clap.rs/) app for easy integration.
  //!
  //! Can be easily added to any CLAP app to extend funcionality.
  //!
  //! Using ***lpsettings*** by itself.
  //!
  //! ```rust
  //! let app = interface::app()
  //!   .get_matches();
  //!
  //! match interface::process(&app) {
  //!   Err(error) => { println!("{}",error); }
  //!   Ok(_) => { }
  //! }
  //! ```
  //!
  //! Using ***lpsettings*** as part of another app.
  //!
  //! ```rust
  //! let app = clap::App("newapp")
  //!   .subcommand(interface::app().name("settings"))
  //!   .get_matches();
  //!
  //! match app.subcommand() {
  //!   ("settings", Some(matches)) => { interface::process(matches); },
  //!   _ => {},
  //! }
  //! ```

  clap::App::new("lpsettings")

  // general application information
    .version(env!("CARGO_PKG_VERSION"))
    .author("snsvrno<snsvrno@tuta.io>")
    .about("Lovepack tool for getting and setting data from lovepack.toml files.")
    .name("lpsettings")

  // switches
    .arg(clap::Arg::with_name("local")
      .long("local")
      .group("location")
      .help("Apply action to local settings file")
      .conflicts_with("global"))

    .arg(clap::Arg::with_name("global")
      .long("global")
      .group("location")
      .help("Apply action to global settings file; default"))

    .arg(clap::Arg::with_name("init")
      .long("init")
      .help("Initalizes settings file")
      .long_help("Initalizer for the settings file, goes through an inital prompt to set important values."))

  // parameters
    .arg(clap::Arg::with_name("KEY")
      .help("Setting's address / path")
      .value_name("KEY")
      .index(1))

    .arg(clap::Arg::with_name("VALUE")
      .help("Setting's value")
      .value_name("VALUE")
      .index(2))


}