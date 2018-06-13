//! simple basic cli interface for lpsettings library
extern crate clap;
extern crate lpsettings;
extern crate ansi_term;
extern crate updater;

use std::env;
use lpsettings::interface;

static UPDATER_URL : &str = "https://github.com/snsvrno/lpsettings-rs";

fn main() {  
  // builds the app
  let app = interface::app()
    .arg(clap::Arg::with_name("debug").long("debug").help("Shows additional information about commands run."))
    .get_matches();

  // this will be in the parent program, so its only here in the bin app
  if app.is_present("debug") { env::set_var("OUTPUT_DEBUG_ENABLED","true"); }

  // checks if there are updates
  check_for_updates();

  // processess the arguement matches.
  match interface::process(&app) {
    Err(error) => { println!("{}",error); }
    Ok(_) => { }
  }
}

fn check_for_updates() {
  match updater::check_for_updates("lpsettings",UPDATER_URL) {
    Err(_) => { }
    Ok(version) => {
      if let Some(version) = version { println!("New version { } available, use 'lpsettings update' to update.",version); }
    }
  }
}