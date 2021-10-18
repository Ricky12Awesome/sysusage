#![feature(format_args_capture)]

use crate::bytes::{ByteFormat, ByteFormatConvert};
use crate::fixed_system::FixedSystem;
use crate::log::LogMode;
use crate::placeholders::PlaceholderExpander;

mod bytes;
mod fixed_system;
mod log;
mod placeholders;
mod util;

macro_rules! placeholder {
  ($($vis:vis $name:ident ($slf:ident, $args:ident) $b:block)+) => {
    $(
      #[allow(unused)]
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

    let val = val(&self.sys).convert_to_display(ByteFormat::KiB, format);

    if no_suffix {
      val.to_string_no_suffix()
    } else {
      val.to_string()
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
  let pre_str = "${mem_used} ${mem_available|mib} ${mem_free} ${mem_total|KiB} ${this_should_not_exist}";
  let str = data.expand_placeholders(pre_str);

  log::debug!("Input \"{}\"", pre_str);
  log::debug!("Output \"{}\"", str);

  print!("{}", str);
}
