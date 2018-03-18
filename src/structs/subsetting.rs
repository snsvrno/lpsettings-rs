use std::collections::HashMap;

#[derive(Serialize,Deserialize,Debug)]
#[serde(untagged)]
pub enum Subsetting {
  Single(String),
  Complex(HashMap<String,Subsetting>)
}