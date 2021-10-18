#![feature(format_args_capture)]

use std::str::FromStr;

use crate::bytes::{ByteFormat, ByteFormatConvert};
use crate::fixed_system::FixedSystem;
use crate::log::LogMode;
use crate::placeholders::PlaceholderExpander;
use crate::util::TrimTrailingZerosToString;

mod bytes;
mod fixed_system;
mod log;
mod placeholders;
mod util;

macro_rules! placeholder {
  ($($vis:vis $name:ident ($slf:ident, $args:ident) $b:block)+) => {
    $(
      $vis fn $name(&$slf, $args: &[&str]) -> String $b
    )+

    #[allow(unused)]
    fn _get_placeholder(&self, name: &str) -> Option<GetPlaceholder> {
      match name {
        $(stringify!($name) => Some(Self::$name),)+
        _ => None
      }
    }
  };
}

struct Data {
  sys: FixedSystem,
}

type GetPlaceholder = placeholders::GetPlaceholder<Data>;

impl Data {
  placeholder! {
    mem_usage(self, args) {
      let mut percision = 2;
      let percent = (self.sys.used_memory() as f64 / self.sys.total_memory() as f64) * 100f64;

      for arg in args {
        if arg.starts_with('.') {
          match usize::from_str(&arg[1..]) {
            Ok(value) => percision = value,
            Err(err) => log::warn!("{:?}", err)
          }
        }
      }

      percent.trim_trailing_zeros_with_precision(percision)
    }
    mem_used(self, args) {
      self.mem_placeholder(FixedSystem::used_memory, args)
    }
    mem_free(self, args) {
      self.mem_placeholder(FixedSystem::free_memory, args)
    }
    mem_available(self, args) {
      self.mem_placeholder(FixedSystem::available_memory, args)
    }
    mem_total(self, args) {
      self.mem_placeholder(FixedSystem::total_memory, args)
    }
    swap_used(self, args) {
      self.mem_placeholder(FixedSystem::used_swap, args)
    }
    swap_total(self, args) {
      self.mem_placeholder(FixedSystem::total_swap, args)
    }
    swap_free(self, args) {
      self.mem_placeholder(FixedSystem::free_swap, args)
    }
  }

  fn mem_placeholder(&self, val: fn(&FixedSystem) -> u64, args: &[&str]) -> String {
    let mut with_suffix = false;
    let mut format = ByteFormat::GiB;

    for arg in args {
      match *arg {
        "with_suffix" => with_suffix = true,
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

    let val = val(&self.sys).convert_to_display(ByteFormat::KiB, format);

    if with_suffix {
      val.to_string()
    } else {
      val.to_string_no_suffix()
    }
  }
}

impl PlaceholderExpander for Data {
  fn get_placeholder(&self, name: &str) -> Option<GetPlaceholder> {
    self._get_placeholder(name)
  }
}

fn main() {
  log::init(LogMode::Debug);
  colored::control::set_override(true);

  let sys = FixedSystem::new_all();
  let data = Data { sys };
  let pre_str = "${mem_usage|.2}% ${mem_usage|.6}%";
  let str = data.expand_placeholders(pre_str);

  log::debug!("Input \"{}\"", pre_str);
  log::debug!("Output \"{}\"", str);

  print!("{}", str);
}
