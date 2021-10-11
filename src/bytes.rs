#![allow(unused)]

use std::fmt::{Display, Formatter};

use crate::util::TrimTrailingZerosToString;

#[repr(u64)]
#[derive(Debug, Copy, Clone)]
pub enum ByteFormat {
  Bytes = 0,
  KB = 1000,
  KiB = 1024,
  MB = 1000u64.pow(2),
  MiB = 1024u64.pow(2),
  GB = 1000u64.pow(3),
  GiB = 1024u64.pow(3),
  TB = 1000u64.pow(4),
  TiB = 1024u64.pow(4),
}

impl ByteFormat {
  fn as_u64(&self) -> u64 {
    unsafe {
      std::mem::transmute(*self)
    }
  }

  pub fn convert_to_bytes(&self, val: u64) -> u64 {
    if let Self::Bytes = self {
      return val;
    }

    val * self.as_u64()
  }

  pub fn convert_from_bytes(&self, val: u64) -> f64 {
    let val = val as f64;
    if let Self::Bytes = self {
      return val;
    }

    val / self.as_u64() as f64
  }
}

impl Display for ByteFormat {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ByteFormat::Bytes => write!(f, "B"),
      ByteFormat::KB => write!(f, "KB"),
      ByteFormat::KiB => write!(f, "KiB"),
      ByteFormat::MB => write!(f, "MB"),
      ByteFormat::MiB => write!(f, "MiB"),
      ByteFormat::GB => write!(f, "GB"),
      ByteFormat::GiB => write!(f, "GiB"),
      ByteFormat::TB => write!(f, "TB"),
      ByteFormat::TiB => write!(f, "TiB"),
    }
  }
}

pub struct ByteDisplay {
  value: u64,
  format: ByteFormat,
}

impl ByteDisplay {
  pub fn from(from: u64, from_format: ByteFormat, to_format: ByteFormat) -> Self {
    Self {
      value: from_format.convert_to_bytes(from),
      format: to_format,
    }
  }

  pub fn to_string_no_suffix(&self) -> String {
    self.format
      .convert_from_bytes(self.value)
      .trim_trailing_zeros_with_precision(2)
  }
}

impl Display for ByteDisplay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_string_no_suffix())?;
    write!(f, " {}", self.format)
  }
}

pub trait ByteFormatConvert {
  fn convert(&self, from_format: ByteFormat, to_format: ByteFormat) -> f64;
  fn convert_to_display(&self, from_format: ByteFormat, to_format: ByteFormat) -> ByteDisplay;
}

impl ByteFormatConvert for u64 {
  fn convert(&self, from_format: ByteFormat, to_format: ByteFormat) -> f64 {
    to_format.convert_from_bytes(from_format.convert_to_bytes(*self))
  }

  fn convert_to_display(&self, from_format: ByteFormat, to_format: ByteFormat) -> ByteDisplay {
    ByteDisplay::from(*self, from_format, to_format)
  }
}
