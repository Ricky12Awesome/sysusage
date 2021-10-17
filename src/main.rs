#![feature(format_args_capture)]

use std::io::Write;
use std::ops::Add;
use std::process::id;

use colored::Colorize;
use log::{Level, LevelFilter};
use pretty_env_logger::env_logger::fmt::Formatter as LogFormatter;

mod bytes;
mod fixed_system;
mod util;

struct Data {}

fn replace_a(data: &Data) -> String {
  String::from("A")
}

static mut PLACEHOLDER_PREFIX: &str = "${";
static mut PLACEHOLDER_SUFFIX: &str = "}";
static PLACEHOLDERS: phf::Map<&'static str, fn(&Data) -> String> = phf::phf_map! {
  "p_a" => crate::replace_a,
};

#[inline]
fn placeholder_prefix<'a>() -> &'a str {
  unsafe { PLACEHOLDER_PREFIX }
}

#[inline]
fn placeholder_suffix<'a>() -> &'a str {
  unsafe { PLACEHOLDER_SUFFIX }
}

fn expand_placeholders(data: &Data, str: &str) -> String {
  let mut out = String::with_capacity(str.len() * 2);
  let prefix = placeholder_prefix();
  let suffix = placeholder_suffix();
  let mut idx = 0;

  while idx < str.len() {
    let remaining = &str[idx..];

    match remaining.find(prefix) {
      Some(start) => {
        let value = &remaining[start + prefix.len()..];

        match value.find(suffix) {
          Some(len) => {
            let placeholder = &value[..len];
            let placeholder_len = len + suffix.len() + 1;

            log::debug!("Placeholder '{placeholder}' at index {idx}, +{start} (index {}) from last placeholder", idx + start);

            out.push_str(&str[idx..idx + start]);

            let value = match PLACEHOLDERS.get(placeholder) {
              Some(f) => f(data),
              None => {
                log::warn!("Placeholder '{placeholder}' does not exit");
                String::with_capacity(placeholder_len * 2)
                  .add(prefix)
                  .add(placeholder)
                  .add(suffix)
              }
            };

            out.push_str(value.as_str());

            idx += start + placeholder_len;
          }
          None => {
            let last_suffix = remaining.find(suffix).unwrap_or(0);
            let remaining = &remaining[last_suffix..];
            let next_prefix = remaining.find(prefix).unwrap_or(remaining.len());

            log::warn!("Missing suffix for placeholder at index '{}'", idx + start);

            out.push_str(&remaining[last_suffix..next_prefix]);

            idx += start;
          }
        }
      }
      None => {
        out.push_str(remaining);
        idx = str.len()
      }
    }

    idx += 1;
  }

  out
}

fn log_format(fmt: &mut LogFormatter, record: &log::Record) -> std::io::Result<()> {
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

fn main() {
  colored::control::set_override(true);
  pretty_env_logger::env_logger::builder()
    .format(log_format)
    .filter_level(LevelFilter::Trace)
    .init();
  // let info = FixedSystem::new_all();
  let data = Data {};
  let str = expand_placeholders(&data, &"${p_a} ${p_b} ${p_c");

  println!("{}", str);
}
