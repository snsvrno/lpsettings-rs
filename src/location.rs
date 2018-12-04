use std::fmt;
use std::env;

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
