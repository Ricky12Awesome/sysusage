use std::collections::HashMap;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
  #[serde(flatten)]
  pub sections: HashMap<String, String>,
  pub output: String,
}

impl Config {
  pub fn from_str(str: &str) -> serde_ini::de::Result<Self> {
    serde_ini::from_str(str)
  }

  pub fn from_reader(r: impl Read) -> serde_ini::de::Result<Self>{
    serde_ini::from_read(r)
  }
}