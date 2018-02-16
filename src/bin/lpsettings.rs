//! simple basic cli interface for lpsettings library

extern crate clap;
extern crate lpsettings;
extern crate ansi_term;

use lpsettings::interface;

fn main() {  
  // builds the app
  let app = interface::app()
    .get_matches();

  // processess the arguement matches.
  match interface::process(&app) {
    Err(error) => { println!("{}",error); }
    Ok(_) => { }
  }
}