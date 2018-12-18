//! organzies all the functions used to interface with a cli application
//! provides a ready made CLAP app for easy integration

use clap;
use location::Location;
use theme;
use std::env;
use failure::Error;

pub fn process(matches : &clap::ArgMatches) -> Result<(),Error> {
    //! process function to be used with [CLAP.RS](https://clap.rs/)'s `.get_matches()`.
    //!
    //! should be called with the subset of matches from clap's `.get_matches()` if used as a subcommand, or all the matches if used as the main app.
    //!
    //! ```rust
    //! // example of using as a subcommand, this is called after .get_matches() 
    //! match app.subcommand() {
    //!     ("settings", Some(matches)) => { interface::process(matches); },
    //!     _ => {},
    //! }
    //! ```

    // checks where it should perform the commands. these are not compatible so 
    // both should never run at the same time.
    if matches.is_present("local") { env::set_var("LOVEPACK_SETTINGS_LOCATION","local"); }
    if matches.is_present("global") { env::set_var("LOVEPACK_SETTINGS_LOCATION","global"); }
    
    // checks if it should run the initalization instead
    if let Some(init_matches) = matches.subcommand_matches("init") { 
        let desc = if init_matches.is_present("desc") { true } else { false };
        super::initalize(desc)?; 
    }

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
    //! displays the value to the cli, internal to interface.rs
    
    // checks where we want to read this value
    let location = Location::get_location();
    let value = match location {
        Location::Local => super::get_value_local(key),
        Location::Global => super::get_value_global(key),
        Location::Best => super::get_value(key),
    };

    // the output
    match value {
        Err(error) => {
            println!("Error reading setting \'{}\': {} {}",
                theme::key(key),
                theme::error_message(error.to_string()),
                location
            ); 
        },
        Ok(value) => {
            match value {
                Some(value) => { 
                    println!("{}: {} {}",
                        theme::key(key),
                        theme::key_value(&format!("{}",value)),
                        location
                    );
                },
                None => {
                    println!("{} is not defined",
                        theme::error(key)
                    ); 
                }
            }
        }    
    }
}

fn set_value(key : &str, value: &str) {
    //! sets the value, internal to interface.rs

    // chooses where to write the settings
    let location = Location::get_location();    
    let result = match location {
        Location::Local => super::set_value_local(&key,&value),
        _ => super::set_value(&key,&value)
    };

    match result { 
        Ok(old_value) => { 
            let old = match old_value {
                Some(old_value) => format!("overwriting {}",theme::key_value(format!("{}",old_value))),
                None => format!(""),
            };
            
            println!("{}{} {} to {} {}",
                location.to_string_cap(),
                if location == Location::Best { "Set" } else { " set" },
                theme::key(key),
                theme::key_value_set(value),
                old
            );
        },
        Err(error) => {
            println!("Error setting \'{}\': {}",
                theme::key(key),
                theme::error(error.to_string()),
            ); 
        }
    } 
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
    //!     .get_matches();
    //!
    //! match interface::process(&app) {
    //!     Err(error) => { println!("{}",error); }
    //!     Ok(_) => { }
    //! }
    //! ```
    //!
    //! Using ***lpsettings*** as part of another app.
    //!
    //! ```rust
    //! let app = clap::App("newapp")
    //!     .subcommand(interface::app().name("settings"))
    //!     .get_matches();
    //!
    //! match app.subcommand() {
    //!     ("settings", Some(matches)) => { interface::process(matches); },
    //!     _ => {},
    //! }
    //! ```

    clap::App::new("lpsettings")

    // general application information
        .version(env!("CARGO_PKG_VERSION"))
        .author("snsvrno<snsvrno@tuta.io>")
        .about("Lovepack tool for getting and setting data from lovepack.toml files.")
        .name("lpsettings")

    // functions
        .subcommand(clap::SubCommand::with_name("init")
            .about("Initalizes the settings file")
            .arg(clap::Arg::with_name("desc")
                .short("d")
                .long("desc")
                .help("Includes descriptions")))

    // switches
        .arg(clap::Arg::with_name("local")
            .long("local")
            .help("Apply action to local settings file"))

        .arg(clap::Arg::with_name("global")
            .long("global")
            .help("Apply action to global settings file; default")
            .conflicts_with("local"))

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