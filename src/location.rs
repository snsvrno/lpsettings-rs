use std::fmt;
use std::env;

#[derive(PartialEq)]
pub enum Location {
    Best,
    Local,
    Global
}

impl Location {
    pub fn get_location() -> Location {
        match env::var("LOVEPACK_SETTINGS_LOCATION") { 
            Err(_) => Location::Best,
            Ok(value) => {
                if value == "global" { Location::Global }
                else { Location::Local }
            }
        }
    }
    
    pub fn to_string_cap(&self) -> String {
        match self {
            Location::Best => "".to_string(),
            Location::Local => "Locally".to_string(),
            Location::Global => "Globally".to_string(),
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Location::Best => write!(f,""),
            Location::Local => write!(f,"locally"),
            Location::Global => write!(f,"globally"),
        }
    }
}
