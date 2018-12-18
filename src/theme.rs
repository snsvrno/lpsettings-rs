//! an attempt to make a cli theme. the idea is that there 
//! will be a couple of lovepack apps that need to share the
//! same "identity" across so that when they are unified in 
//! in "lovepack.exe" they all output and operate the same.
//! 
//! this is the first one though, so we shall see how this 
//! changes and evolves over time.

use ansi_term::Colour;

pub fn key<T: AsRef<str>>(text : T) -> String {
    format!("{}",Colour::Yellow.paint(text.as_ref()))
}

pub fn key_value<T: AsRef<str>>(text : T) -> String {
    format!("{}",Colour::Blue.paint(text.as_ref()))
}

pub fn key_value_set<T: AsRef<str>>(text : T) -> String {
    format!("{}",Colour::Green.paint(text.as_ref()))
}

pub fn error<T: AsRef<str>>(text : T) -> String {
    format!("{}",Colour::Red.paint(text.as_ref()))
}
pub fn error_message<T: AsRef<str>>(text : T) -> String {
    format!("{}",Colour::Red.paint(text.as_ref()))
}

pub fn comment<T: AsRef<str>>(text : T) -> String {
    format!("{}",Colour::Cyan.paint(text.as_ref()))
}