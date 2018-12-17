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

// settings stuff
extern crate settingsfile;
use settingsfile::{ SupportedType, ShadowSettings };
pub use settingsfile::Type as Type;

pub mod interface;

mod settings;
mod location;
mod theme;

pub fn get_value(key : &str) -> Result<Option<Type>,Error> {
    let mut settings = ShadowSettings::new(settings::Configuration{});
    settings.load()?;
    Ok(settings.get_value(key))
}

pub fn get_value_or<A>(key : &str, default_value : &A) -> Type 
    where A : SupportedType
{
    match get_value(key) {
        Err(error) => { 
            //error!("{}",error);
            default_value.wrap()
            }
        Ok(option) => { 
            match option {
                Some(value) => value,
                None => default_value.wrap()
            } 
        },
    }
}

pub fn get_value_local(key : &str) -> Result<Option<Type>,Error> {
    let mut settings = ShadowSettings::new(settings::Configuration{});
    settings.load()?;
    Ok(settings.get_value_local(key))
}

pub fn get_value_global(key : &str) -> Result<Option<Type>,Error> {
    let mut settings = ShadowSettings::new(settings::Configuration{});
    settings.load()?;
    Ok(settings.get_value_global(key))
}

pub fn set_value<A>(key : &str, value : &A) -> Result<Option<Type>,Error> 
    where A : SupportedType
{
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
    let mut settings = ShadowSettings::new(settings::Configuration{});
    settings.load()?;
    let old_value = settings.get_value_local(key);
    settings.set_value_local(key,value)?;
    settings.save()?;

    Ok(old_value)
}