//! simple basic cli interface for lpsettings library,
//! creates the lpsettins binary.

extern crate clap;
extern crate lpsettings;
extern crate ansi_term;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate updater_lp;
extern crate chrono;

use lpsettings::interface;

// upstream for the repository, used as the source of the releases
static UPDATER_URL : &str = "https://github.com/snsvrno/lpsettings-rs";

fn main() {

    // builds the app, adding cli specific switches for 
    // - debug
    // - update
    let app = interface::app()
        .arg(clap::Arg::with_name("debug").long("debug").help("Shows additional information about commands run."))
        .arg(clap::Arg::with_name("update").long("update").help("Updates application"))
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
    check_for_updates();

    // checks if the user wants to update
    if app.is_present("update") {
        update_app();
    }

    // processess the arguement matches.
    match interface::process(&app) {
        Err(error) => { error!("{}",error); }
        Ok(_) => { }
    }
}

fn update_app() {
    //! performs the actual update,
    //! 
    //! the user request to update the app.

    match update_get_version_link() {
        None => {
            println!("No update available");
        },
        Some(link) => {
            match updater_lp::update_from_link(&link) {
                Err(error) => error!("{}",error),
                Ok(_) => println!("Update complete."),
            }
        }
    }
}

fn check_for_updates() {
    //! automatic checking for update loop,
    //! 
    //! writes to the configuration to keep track of somethings
    
    // if an update is already available, then no need to do this part.
    if let Ok(Some(lpsettings::Type::Switch(true))) = lpsettings::get_value("lpsettings.update.available") {
        println!("Update available, use `lpsettings update` to update.");
        return;
    }

    // getting the update frequency, defaults to 1 day but can be overwritten
    // in the config / settings
    let mut frequency : i64 = 1;
    if let Ok(Some(result)) = lpsettings::get_value("lpsettings.update.freq") {
        match result {
            lpsettings::Type::Int(int) => { frequency = int as i64; },
            lpsettings::Type::Float(float) => { frequency = float as i64; },
            _ => { }
        }
    }

    let now = chrono::Utc::now();

    // checks if should update based on the frequency
    if let Ok(Some(last_check)) = lpsettings::get_value("lpsettings.update.last_check") {
        if let lpsettings::Type::Text(text_date) = last_check {
            match chrono::DateTime::parse_from_rfc3339(&text_date){
                Err(error) => { error!("{}",error); },
                Ok(date) => {
                    if now.signed_duration_since(date).num_days() < frequency { return; }
                },
            }
        }
    }

    // we are still here, so time to check for an update.
    update_get_version_link();
}

fn update_get_version_link() -> Option<String> {
    //! returns the link for the most recent version,
    //! 
    //! also does some setting of the settings file based on update frequency
    //! and if there is an update available or not.
    
    let now = chrono::Utc::now();

    let pkg_ver = env!("CARGO_PKG_VERSION");
    match updater_lp::create_version(pkg_ver) {
        None => { warn!("Cannot create app version from {}, will not be checking for updates.",pkg_ver) },
        Some(app_version) => {
            info!("Checking for update, currently version {}",app_version);
            match updater_lp::get_link_for_latest(UPDATER_URL) {
                Err(error) => { error!("{}",error); },
                Ok((link,version)) => {
                    if version > app_version {
                        println!("Update available.");
                        info!("update found: {}",version);
                        lpsettings::set_value("lpsettings.update.last_check",&now.to_rfc3339());
                        lpsettings::set_value("lpsettings.update.available",&true);
                        return Some(link);
                    } else {
                        info!("no update found.");
                        lpsettings::set_value("lpsettings.update.last_check",&now.to_rfc3339());
                        lpsettings::set_value("lpsettings.update.available",&false);
                    }
                }
            }
        }
    }

    None
}