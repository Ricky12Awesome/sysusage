#![feature(format_args_capture, associated_type_defaults, result_flattening)]

use std::collections::HashMap;
use std::str::FromStr;

use crate::bytes::{ByteFormat, ByteFormatConvert};
use crate::color::{Color, ColorMode, set_color_mode};
use crate::config::Config;
use crate::fixed_system::FixedSystem;
use crate::log::LogMode;
use crate::placeholders::PlaceholderExpander;
use crate::util::TrimTrailingZerosToString;

mod bytes;
mod color;
mod config;
mod fixed_system;
mod log;
mod placeholders;
mod util;

macro_rules! placeholder {
  ($($vis:vis $name:ident ($slf:ident, $args:ident) $b:block)+) => {
    $(
      $vis fn $name(&$slf, $args: Args) -> String $b
    )+

    #[allow(unused)]
    fn _get_placeholder(&self, name: &str) -> Option<PlaceholderFn> {
      match name {
        $(stringify!($name) => Some(Box::new(Self::$name)),)+
        _ => None
      }
    }
  };
}

struct Data {
  sys: FixedSystem,
  custom: HashMap<String, String>,
}

impl Data {
  fn new(sys: FixedSystem, config: Config) -> Self {
    let mut _self = Self {
      sys,
      custom: HashMap::with_capacity(config.sections.len() * 2)
    };

    for (name, value) in config.sections {
      _self.custom.insert(name, _self.expand_placeholders(&value));
    }

    _self
  }
}

#[derive(Debug)]
struct Args {
  name: String,
  precision: usize,
  format: ByteFormat,
  with_suffix: bool,
  fg: String,
  bg: String,
}

impl Default for Args {
  fn default() -> Self {
    Self {
      name: String::new(),
      precision: 2,
      format: ByteFormat::GiB,
      with_suffix: false,
      fg: String::new(),
      bg: String::new(),
    }
  }
}

impl Args {
  fn from(name: &str, args: &[&str]) -> Self {
    let mut out = Self::default();

    out.name = name.to_string();

    for arg in args {
      if arg.starts_with('.') {
        match usize::from_str(&arg[1..]) {
          Ok(value) => out.precision = value,
          Err(err) => log::warn!("{:?}", err)
        }
      }

      if name == "fg" {
        match Color::from_str(arg) {
          Ok(color) => out.fg = color.fg(),
          Err(err) => log::warn!("{err:?}")
        };
      }

      if name == "bg" {
        match Color::from_str(arg) {
          Ok(color) => out.bg = color.bg(),
          Err(err) => log::warn!("{err:?}")
        };
      }

      match *arg {
        "with_suffix" => out.with_suffix = true,
        "kb" => out.format = ByteFormat::KB,
        "kib" => out.format = ByteFormat::KiB,
        "mb" => out.format = ByteFormat::MB,
        "mib" => out.format = ByteFormat::MiB,
        "gb" => out.format = ByteFormat::GB,
        "gib" => out.format = ByteFormat::GiB,
        "tb" => out.format = ByteFormat::GB,
        "tib" => out.format = ByteFormat::GiB,
        _ => {}
      }
    }

    out
  }
}

type PlaceholderFn = placeholders::PlaceholderFn<Data, Args>;

impl Data {
  placeholder! {
    reset(self, _args) { Color::Reset.fg() }

    fg(self, args) { args.fg }
    bg(self, args) { args.bg }

    mem_usage(self, args) {
      let percent = (self.sys.used_memory() as f64 / self.sys.total_memory() as f64) * 100f64;

      percent.trim_trailing_zeros_with_precision(args.precision)
    }

    mem_used(self, args) { self.mem_placeholder(FixedSystem::used_memory, args) }
    mem_free(self, args) { self.mem_placeholder(FixedSystem::free_memory, args) }
    mem_available(self, args) { self.mem_placeholder(FixedSystem::available_memory, args) }
    mem_total(self, args) { self.mem_placeholder(FixedSystem::total_memory, args) }
    swap_used(self, args) { self.mem_placeholder(FixedSystem::used_swap, args) }
    swap_total(self, args) { self.mem_placeholder(FixedSystem::total_swap, args) }
    swap_free(self, args) { self.mem_placeholder(FixedSystem::free_swap, args) }
  }

  fn mem_placeholder(&self, val: fn(&FixedSystem) -> u64, args: Args) -> String {
    let val = val(&self.sys).convert_to_display(ByteFormat::KiB, args.format);

    if args.with_suffix {
      val.to_string()
    } else {
      val.to_string_no_suffix()
    }
  }
}


impl PlaceholderExpander for Data {
  type Args = Args;

  fn get_placeholder(&self, name: &str) -> Option<PlaceholderFn> {
    fn custom(s: &String) -> PlaceholderFn {
      let s = s.clone();

      Box::new(move |_, _| { s.clone() })
    }

    match self._get_placeholder(name) {
      Some(val) => Some(val),
      None => self.custom
        .get(&name.to_string())
        .map(custom)
    }
  }

  fn parse_args(&self, name: &str, args: &[&str]) -> Self::Args {
    Args::from(name, args)
  }
}

fn main() {
  log::init(LogMode::Debug);
  set_color_mode(ColorMode::Always);

  let sys = FixedSystem::new_all();
  let config = Config::from_str(r#"
used=${bg|black}${fg|green}${mem_used|.2}
total=${fg|gray}/ ${fg|blue}${mem_total}
usage=${fg|gray}(${fg|yellow}${mem_usage|.2|with_suffix}%${fg|gray})
output=${used} ${total} ${usage}
"#).unwrap();
  let output = config.output.clone();
  let data = Data::new(sys, config);
  let str = data.expand_placeholders(&output);

  print!("{}", str);
}
