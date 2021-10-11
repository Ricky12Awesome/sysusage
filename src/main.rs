use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Add;
use colored::Colorize;

use crate::bytes::{ByteFormat, ByteFormatConvert};
use crate::fixed_system::FixedSystem;
use crate::util::TrimTrailingZerosToString;

mod bytes;
mod fixed_system;
mod util;

fn replace_all(str: &mut String, values: HashMap<impl ToString, impl ToString>) {
  for (key, val) in values {
    *str = str.replace(key.to_string().as_str(), val.to_string().as_str());
  }
}

static mut PLACEHOLDER_PREFIX: &str = "${";
static mut PLACEHOLDER_SUFFIX: &str = "}";

fn placeholder_prefix<'a>() -> &'a str {
  unsafe { PLACEHOLDER_PREFIX }
}

fn placeholder_suffix<'a>() -> &'a str {
  unsafe { PLACEHOLDER_SUFFIX }
}

macro_rules! placeholder {
  ($($arg:tt)*) => {
    String::new()
      .add(placeholder_prefix())
      .add(format!($($arg)*).as_str())
      .add(placeholder_suffix())
  };
}

fn insert_memory_info(sys: &FixedSystem, values: &mut HashMap<String, String>) {
  macro_rules! insert {
    ($name:literal, $val:expr) => {
      values.insert(placeholder!("{}_in_GB", $name), $val.convert_to_display(ByteFormat::KiB, ByteFormat::GB).to_string_no_suffix());
      values.insert(placeholder!("{}_in_GiB", $name), $val.convert_to_display(ByteFormat::KiB, ByteFormat::GiB).to_string_no_suffix());
      values.insert(placeholder!("{}_in_MB", $name), $val.convert_to_display(ByteFormat::KiB, ByteFormat::MB).to_string_no_suffix());
      values.insert(placeholder!("{}_in_MiB", $name), $val.convert_to_display(ByteFormat::KiB, ByteFormat::MiB).to_string_no_suffix());
      values.insert(placeholder!("{}_in_KB", $name), $val.convert_to_display(ByteFormat::KiB, ByteFormat::KB).to_string_no_suffix());
      values.insert(placeholder!("{}_in_KiB", $name), $val.convert_to_display(ByteFormat::KiB, ByteFormat::KiB).to_string_no_suffix());
    };
  }

  insert!("mem_total", sys.total_memory());
  insert!("mem_used", sys.used_memory());
  insert!("mem_free", sys.free_memory());
  insert!("mem_available", sys.available_memory());

  let mem_percent = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100f64;
  values.insert(placeholder!("mem_percent"), mem_percent.trim_trailing_zeros_with_precision(2));

  insert!("swap_total", sys.total_swap());
  insert!("swap_used", sys.used_swap());
  insert!("swap_free", sys.free_swap());

  let mem_percent = (sys.used_swap() as f64 / sys.total_swap() as f64) * 100f64;
  values.insert(placeholder!("swap_percent"), mem_percent.trim_trailing_zeros_with_precision(2));
}

fn main() {
  colored::control::set_override(true);
  let info = FixedSystem::new_all();
  let mut str = String::from("${mem_percent}% ${swap_percent}% ${mem_used_in_GiB} / ${mem_total_in_GiB}");
  let mut values = HashMap::new();

  insert_memory_info(&info, &mut values);
  replace_all(&mut str, values);

  println!("{}", str);
}
