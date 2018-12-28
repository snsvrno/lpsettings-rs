/// contains the struct to use for configuration
/// and settings physical file definition

// settings stuff
use settingsfile::{ Format, SettingsRaw, SupportedType };
// for implementations
use serde;
use failure::Error;
// format to use
use toml;

/// implementation of the LPSETTINGS configuration
/// these are settings that are shared across all
/// of the LOVEPACK apps.
/// 
/// this implementation is very basic, just using 
/// toml-rs and serde-rs to write and read to toml
#[derive(Clone)]
pub struct Configuration { 
}

impl Format for Configuration {
    fn filename(&self) -> String { "lovepack".to_string() }
    fn folder(&self) -> String { ".lovepack".to_string() }
    fn extension(&self) -> Option<String> { Some("toml".to_string()) }

    fn from_str<T>(&self,buffer:&str) -> Result<SettingsRaw,Error> 
        where T : Format + Clone 
    {
        match toml::de::from_str(&buffer) {
            Ok(result) => Ok(result),
            Err(error) => Err(format_err!("{}",error)),
        }
    }

    fn to_string<T:Sized>(&self,object:&T) -> Result<String,Error>
        where T : SupportedType + serde::ser::Serialize, 
    {
        match toml::ser::to_string(object) {
                Ok(result) => Ok(result),
                Err(error) => Err(format_err!("{}",error)),
        }
    }
}