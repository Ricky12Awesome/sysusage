#![allow(unused)]

use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, Read};

#[derive(Default, Debug)]
pub struct Config {
  pub sections: Vec<(String, String)>,
  pub output: String,
}

#[derive(Debug)]
pub enum ConfigError {
  IO(std::io::Error),
  InvalidLine(usize, String),
  Other,
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(self, f)
  }
}

impl Error for Config {}

pub type ConfigResult<T> = std::result::Result<T, ConfigError>;

impl Config {
  fn new(sections: Vec<(String, String)>, output: String) -> Self {
    Self { sections, output }
  }

  pub fn from_str(str: &str) -> ConfigResult<Self> {
    let mut _self = Self::default();

    for (line_num, line) in str.lines().enumerate() {
      _self.read_line(line_num, line)?;
    }

    Ok(_self)
  }

  pub fn from_read(buf: impl BufRead) -> ConfigResult<Self> {
    let mut _self = Self::default();

    for (line_num, line) in buf.lines().enumerate() {
      let line = line.map_err(ConfigError::IO)?;
      _self.read_line(line_num, &line)?;
    }

    Ok(_self)
  }

  fn read_line(&mut self, line_num: usize, line: &str) -> ConfigResult<()> {
    if line.is_empty()
      || line.starts_with("#")
      || line.starts_with("//") {
      return Ok(());
    }

    let (key, value) = line
      .split_once("=")
      .ok_or_else(|| ConfigError::InvalidLine(line_num, line.to_string()))?;

    let key_value = (key.trim().to_string(), value.trim().to_string());

    if key_value.0 == "output" {
      self.output = key_value.1
    } else {
      self.sections.push(key_value)
    }

    Ok(())
  }
}
