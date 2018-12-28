/// LPSETTINGS
/// 
/// a library and a binary
/// the library contains functions to read and write to
/// a local configuration (in working directory or global directory)
/// for all the lovepack applications. you should not be using this
/// unless you are writing something part of the lovepack family.
/// 
/// the binary just uses the interface(.rs) and creates a terminal
/// application where you can read and write settings

#[macro_use] extern crate failure;
use failure::Error;
extern crate toml;
extern crate serde;

// terminal stuff
extern crate ansi_term;
extern crate clap;

// for update helpers
extern crate chrono;

// settings stuff
extern crate settingsfile;
use settingsfile::{ SupportedType, ShadowSettings };
pub use settingsfile::Type as Type;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate version_lp;

use std::path::PathBuf;

pub mod interface;

mod settings;
mod location;
mod theme;
mod map;
pub mod update;


pub fn get_folder() -> PathBuf {
    PathBuf::from(settingsfile::Format::get_path(&settings::Configuration{}))
}

pub fn get_value(key : &str) -> Result<Option<Type>,Error> {
    //! Get the value.

    let mut settings = ShadowSettings::new(settings::Configuration{});
    settings.load()?;
    Ok(settings.get_value(key))
}

pub fn get_value_or<A>(key : &str, default_value : &A) -> Type 
    where A : SupportedType
{
    //! Get the value or return a default value.
    match get_value(key) {
        Err(_) => default_value.wrap(),
        Ok(option) => { 
            match option {
                Some(value) => value,
                None => default_value.wrap()
            } 
        },
    }
}

pub fn get_value_local(key : &str) -> Result<Option<Type>,Error> {
    //! Get the value in the local configuration only. 
    //! 
    //! Can return empty if the local file doesn't have a value but
    //! the global one does.
    
    let mut settings = ShadowSettings::new(settings::Configuration{});
    settings.load()?;
    Ok(settings.get_value_local(key))
}

pub fn get_value_global(key : &str) -> Result<Option<Type>,Error> {
    //! Get the value in the global configuration only.
    //! 
    //! Can return empty if the global file doesn't have a value but
    //! the local one does.
    
    let mut settings = ShadowSettings::new(settings::Configuration{});
    settings.load()?;
    Ok(settings.get_value_global(key))
}

pub fn set_value<A>(key : &str, value : &A) -> Result<Option<Type>,Error> 
    where A : SupportedType
{
    //! Sets the value on the global level.

    let mut settings = ShadowSettings::new(settings::Configuration{});
    settings.load()?;
    let old_value = settings.get_value(key);
    settings.set_value_global(key,value)?;
    settings.save()?;

    Ok(old_value)
}

pub fn set_value_local<A>(key : &str, value : &A) -> Result<Option<Type>,Error> 
    where A : SupportedType
{
    //! Sets the value on the local level.

    let mut settings = ShadowSettings::new(settings::Configuration{});
    settings.load()?;
    let old_value = settings.get_value_local(key);
    settings.set_value_local(key,value)?;
    settings.save()?;

    Ok(old_value)
}

pub fn initalize(desc : bool) -> Result<(),Error>{
    //! initalizes the global settings (only global)
    //! 
    //! uses the map.toml file from source to determine what
    //! should be initalized and gives descriptions.
    //! 
    //! if the user leaves the entry blank it will not be editied or 
    //! created, so the existing values will stay the same or new values
    //! will not be created.
    
    let map =  map::create_options_map()?;
    let lib_version = version_lp::Version::from_str(env!("CARGO_PKG_VERSION")).unwrap();

    let mut settings = ShadowSettings::new(settings::Configuration{});
    settings.load()?;

    println!("Initializing settings, leave empty to keep existing / not set.");

    for m in map {
        if let Some(true) = m.init {
            
            if let Some(added) = m.added {
                // if the added version is greater than the current version
                // then this isn't valid
                if added > lib_version { continue; }
            }
            if let Some(removed) = m.removed {
                // if the removed version is less or equal than the current version
                // then it isn't valid
                if removed <= lib_version { continue; }
            }

            let old_value = settings.get_value_global(&m.key);
            
            let mut question = if let Some(old_value) = old_value {
                format!("{}({})",
                    theme::key(&m.key),
                    theme::key_value(format!("{}",old_value)))
            } else {
                format!("{}",
                    theme::key(&m.key))
            };

            // adds the descriptions if requested.
            if desc {
                question = format!("{} - {}",
                    question,
                    theme::comment(&m.desc));
            }

            let new_value = map::get_user_input(&question);
            if new_value.len() > 0 {
                settings.set_value_global(&m.key,&new_value)?;
            }
        }
    }

    // saves the new settings
    settings.save()?;

    Ok(())
}

pub fn list_possible() -> Result<(),Error> {
    let map =  map::create_options_map()?;
    let lib_version = version_lp::Version::from_str(env!("CARGO_PKG_VERSION")).unwrap();

    for m in map {
        if let Some(added) = m.added {
            // if the added version is greater than the current version
            // then this isn't valid
            if added > lib_version { continue; }
        }
        if let Some(removed) = m.removed {
            // if the removed version is less or equal than the current version
            // then it isn't valid
            if removed <= lib_version { continue; }
        }

        println!("{} - {}",
            theme::key(&m.key),
            theme::comment(&m.desc)
        );
    }

    Ok(())
}

pub fn list_current() -> Result<(),Error> {
    let mut settings = ShadowSettings::new(settings::Configuration{});
    settings.load()?;  

    let local_keys = { 
        let mut keys = settings.keys_local();
        keys.sort();
        keys 
    };

    // only put the heading if there is local settings too.
    if local_keys.len() > 0 {
        println!("{}",theme::heading("Global Settings"));
    }

    let global_keys = { 
        let mut keys = settings.keys_global();
        keys.sort();
        keys 
    };

    for k in global_keys {
        if let Some(value) = settings.get_value_global(&k) {
            println!("{}: {}",
                theme::key(k),
                theme::key_value(format!("{}",value))
            );
        } else {
            println!("{}",
                theme::key(k)
            );
        }
    }

    if local_keys.len() > 0 {
        println!("{}",theme::heading("Local Settings"));

        for k in local_keys {
            if let Some(value) = settings.get_value_local(&k) {
                println!("{}: {}",
                    theme::key(k),
                    theme::key_value(format!("{}",value))
                );
            } else {
                println!("{}",
                    theme::key(k)
                );
            }
        }
    }

    Ok(())
}