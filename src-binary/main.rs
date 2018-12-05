//! simple basic cli interface for lpsettings library
extern crate clap;
extern crate lpsettings;
extern crate ansi_term;
extern crate log;
extern crate pretty_env_logger;
//extern crate updater;

use lpsettings::interface;

//static UPDATER_URL : &str = "https://github.com/snsvrno/lpsettings-rs";

fn main() {    
    // builds the app
    let app = interface::app()
        .arg(clap::Arg::with_name("debug").long("debug").help("Shows additional information about commands run."))
        .get_matches();

    // starts the loggers & sets the filter level for the logs
    match pretty_env_logger::formatted_builder() {
        Err(error) => { println!("Failed to start logging: {}",error); },
        Ok(mut builder) => {
            let level = if app.is_present("debug") { 
                log::LevelFilter::Info 
            } else { 
                log::LevelFilter::Error 
            };

            builder
                .filter(None,level)
                .init();
        }
    }

    // checks if there are updates
    // check_for_updates();

    // processess the arguement matches.
    match interface::process(&app) {
        Err(error) => { println!("{}",error); }
        Ok(_) => { }
    }
}
/*
fn check_for_updates() {
    match updater::check_for_updates("lpsettings",UPDATER_URL) {
        Err(_) => { }
        Ok(version) => {
            if let Some(version) = version { 
                println!("New version {} available, use 'lpsettings update' to update.",version); 
            }
        }
    }
}*/