#![allow(unused)]
use std::io::Write;

use colored::Colorize;
pub use log::*;
use pretty_env_logger::env_logger::fmt::Formatter;

fn log_format(fmt: &mut Formatter, record: &log::Record) -> std::io::Result<()> {
  let args = record.args().to_string();
  let args = args.as_str();

  let (level, args) = match record.level() {
    Level::Error => ("Error".red(), args.bright_red()),
    Level::Warn => ("Warn".yellow(), args.bright_yellow()),
    Level::Info => ("Info".white(), args.bright_white()),
    Level::Debug => ("Debug".magenta(), args.bright_magenta()),
    Level::Trace => ("Trace".blue(), args.bright_blue()),
  };

  writeln!(fmt, "[{}] {}", level, args)
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum LogMode {
  /// None
  Silent = LevelFilter::Off as usize,
  /// All
  Debug = LevelFilter::Trace as usize,
  /// Warn, Error
  Default = LevelFilter::Warn as usize,
}

impl LogMode {
  fn as_level_filter(&self) -> LevelFilter {
    unsafe {
      std::mem::transmute(*self)
    }
  }
}

impl Default for LogMode {
  fn default() -> Self {
    LogMode::Default
  }
}

pub fn init(mode: LogMode) {
  pretty_env_logger::env_logger::builder()
    .format(log_format)
    .filter_level(mode.as_level_filter())
    .init();
}