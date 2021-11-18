#![allow(unused)]

use std::fs::File;
use std::io::Read;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use sysinfo::{ComponentExt, RefreshKind, System, SystemExt};

/// Removes [sysinfo::System] memory conversion, since it makes it inaccurate
/// `/proc/meminfo` is already `KiB`, even though it says `kB`
#[derive(Default, Debug)]
pub struct FixedSystem {
  sys: System,
}

pub struct CPU<'a> {
  pub processor: &'a sysinfo::Processor,
  pub component: &'a sysinfo::Component
}

impl Deref for FixedSystem {
  type Target = System;

  fn deref(&self) -> &Self::Target {
    &self.sys
  }
}

impl DerefMut for FixedSystem {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.sys
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

  pub fn cpu(&self) -> Option<CPU<'_>> {
    Some(CPU {
      processor: self.sys.global_processor_info(),
      component: self.components()
        .iter()
        .find(|it| it.label() == "CPU")?
    })
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
