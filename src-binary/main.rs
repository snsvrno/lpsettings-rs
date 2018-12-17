//! simple basic cli interface for lpsettings library
extern crate clap;
extern crate lpsettings;
extern crate ansi_term;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate updater_lp;
extern crate chrono;

use lpsettings::interface;

static UPDATER_URL : &str = "https://github.com/snsvrno/lpsettings-rs";

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
    check_for_updates();

    // processess the arguement matches.
    match interface::process(&app) {
        Err(error) => { error!("{}",error); }
        Ok(_) => { }
    }
}

fn check_for_updates() {
    
    // if an update is already available, then no need to do this part.
    if let Ok(Some(lpsettings::Type::Switch(true))) = lpsettings::get_value("lpsettings.update.available") {
        println!("Update available.");
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
                        /*match updater_lp::update_from_link(&link) {
                            Err(error) => { error!("{}",error) },
                            Ok(_) => { 
                                info!("Update successful.");
                                lpsettings::set_value("lpsettings.update.last_check",&now.to_rfc3339());
                            }
                        }*/
                        lpsettings::set_value("lpsettings.update.last_check",&now.to_rfc3339());
                        lpsettings::set_value("lpsettings.update.available",&true);
                    } else {
                        info!("no update found.");
                        lpsettings::set_value("lpsettings.update.last_check",&now.to_rfc3339());
                        lpsettings::set_value("lpsettings.update.available",&false);
                    }
                }
            }
        }
    }
}