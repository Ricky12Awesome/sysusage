#![feature(format_args_capture)]

use phf::Map;

use crate::bytes::{ByteFormat, ByteFormatConvert};
use crate::fixed_system::FixedSystem;
use crate::log::LogMode;
use crate::placeholders::PlaceholderExpander;

mod bytes;
mod fixed_system;
mod log;
mod placeholders;
mod util;

struct Data {
  sys: FixedSystem,
}

impl Data {
  fn mem_used(&mut self, args: &[&str]) -> String {
    let mut no_suffix = false;
    let mut format = ByteFormat::GiB;

    for arg in args {
      match *arg {
        "no_suffix" => no_suffix = true,
        "KB" | "kb" => format = ByteFormat::KB,
        "KiB" | "kib" => format = ByteFormat::KiB,
        "MB" | "mb" => format = ByteFormat::MB,
        "MiB" | "mib" => format = ByteFormat::MiB,
        "GB" | "gb" => format = ByteFormat::GB,
        "GiB" | "gib" => format = ByteFormat::GiB,
        "TB" | "tb" => format = ByteFormat::GB,
        "TiB" | "tib" => format = ByteFormat::GiB,
        _ => {}
      }
    }

    let value = self.sys
      .used_memory()
      .convert_to_display(ByteFormat::KiB, format);

    if no_suffix {
      value.to_string_no_suffix()
    } else {
      value.to_string()
    }
  }
}

impl PlaceholderExpander for Data {
  fn placeholders(&self) -> &Map<&'static str, fn(&mut Self, &[&str]) -> String> {
    static PLACEHOLDERS: phf::Map<&'static str, fn(&mut Data, &[&str]) -> String> = phf::phf_map! {
      "mem_used" => Data::mem_used,
    };

    &PLACEHOLDERS
  }

  fn placeholder_prefix(&self) -> &str {
    "${"
  }

  fn placeholder_suffix(&self) -> &str {
    "}"
  }
}

fn main() {
  colored::control::set_override(true);
  log::init(LogMode::Debug);
  let sys = FixedSystem::new_all();
  let mut data = Data { sys };
  let pre_str = "${mem_used} ${mem_used|mib} ${mem_used|no_suffix} ${mem_used|KiB}";
  let str = data.expand_placeholders(pre_str);

  log::debug!("Input \"{}\"", pre_str);
  log::debug!("Output \"{}\"", str);

  print!("{}", str);
}
