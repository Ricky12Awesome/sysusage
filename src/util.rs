use num::traits::clamp_max;

pub trait TrimTrailingZeros {
  fn trim_trailing_zeros(&self) -> &str;
  fn trim_trailing_zeros_with_precision(&self, precision: usize) -> &str;
}

pub trait TrimTrailingZerosToString {
  fn trim_trailing_zeros(&self) -> String;
  fn trim_trailing_zeros_with_precision(&self, precision: usize) -> String;
}

impl TrimTrailingZeros for str {
  fn trim_trailing_zeros(&self) -> &str {
    let mut length = self.len();

    if !self.contains('.') {
      return self;
    }

    for (index, char) in self.char_indices().rev() {
      if char != '0' {
        length = index + 1;
        break
      }
    }

    &self[..length].trim_end_matches('.')
  }

  fn trim_trailing_zeros_with_precision(&self, precision: usize) -> &str {
    match self.find('.') {
      None => self,
      Some(idx) => {
        let length = clamp_max(idx + precision + 1, self.len());

        (&self[..length]).trim_trailing_zeros()
      }
    }
  }
}

impl TrimTrailingZerosToString for f32 {
  fn trim_trailing_zeros(&self) -> String {
    self.to_string().trim_trailing_zeros().to_string()
  }

  fn trim_trailing_zeros_with_precision(&self, precision: usize) -> String {
    self.to_string().trim_trailing_zeros_with_precision(precision).to_string()
  }
}

impl TrimTrailingZerosToString for f64 {
  fn trim_trailing_zeros(&self) -> String {
    self.to_string().trim_trailing_zeros().to_string()
  }

  fn trim_trailing_zeros_with_precision(&self, precision: usize) -> String {
    self.to_string().trim_trailing_zeros_with_precision(precision).to_string()
  }
}
