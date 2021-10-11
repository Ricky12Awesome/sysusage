#![allow(unused)]

use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::str::FromStr;

use sysinfo::{RefreshKind, System, SystemExt};

/// Removes [sysinfo::System] memory conversion, since it makes it inaccurate
/// `/proc/meminfo` is already `KiB`, even though it says `kB`
#[derive(Default, Debug)]
pub struct FixedSystem {
  sys: System,
}

impl Deref for FixedSystem {
  type Target = System;

  fn deref(&self) -> &Self::Target {
    &self.sys
  }
}


fn convert(from: u64) -> u64 {
  (from as f64 / 1.024).ceil() as u64
}

impl FixedSystem {
  pub fn new() -> Self {
    Self {
      sys: System::new(),
      ..Default::default()
    }
  }

  pub fn new_all() -> Self {
    Self {
      sys: System::new_all(),
      ..Default::default()
    }
  }

  pub fn new_with_specifics(refreshes: RefreshKind) -> Self {
    Self {
      sys: System::new_with_specifics(refreshes),
      ..Default::default()
    }
  }

  /// Total Memory in `KiB`
  pub fn total_memory(&self) -> u64 {
    convert(self.sys.total_memory())
  }

  /// Free Memory in `KiB`
  pub fn free_memory(&self) -> u64 {
    convert(self.sys.free_memory())
  }

  /// Available Memory in `KiB`
  pub fn available_memory(&self) -> u64 {
    convert(self.sys.available_memory())
  }

  /// Used Memory in `KiB`
  pub fn used_memory(&self) -> u64 {
    convert(self.sys.total_memory() - self.sys.available_memory())
  }

  /// Total Swap in `KiB`
  pub fn total_swap(&self) -> u64 {
    convert(self.sys.total_swap())
  }

  /// Free Swap in `KiB`
  pub fn free_swap(&self) -> u64 {
    convert(self.sys.free_swap())
  }

  /// Used Swap in `KiB`
  pub fn used_swap(&self) -> u64 {
    convert(self.sys.used_swap())
  }
}
