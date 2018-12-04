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