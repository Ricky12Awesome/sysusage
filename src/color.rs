use std::borrow::Cow;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
  Black,
  Red,
  Green,
  Yellow,
  Blue,
  Magenta,
  Cyan,
  White,
  BrightBlack,
  BrightRed,
  BrightGreen,
  BrightYellow,
  BrightBlue,
  BrightMagenta,
  BrightCyan,
  BrightWhite,
  Reset,
  Color256(u8),
  TrueColor(u8, u8, u8)
}

impl Color {
  fn to_fg_str(&self) -> Cow<'static, str> {
    match *self {
      Color::Black => "30".into(),
      Color::Red => "31".into(),
      Color::Green => "32".into(),
      Color::Yellow => "33".into(),
      Color::Blue => "34".into(),
      Color::Magenta => "35".into(),
      Color::Cyan => "36".into(),
      Color::White => "37".into(),
      Color::BrightBlack => "90".into(),
      Color::BrightRed => "91".into(),
      Color::BrightGreen => "92".into(),
      Color::BrightYellow => "93".into(),
      Color::BrightBlue => "94".into(),
      Color::BrightMagenta => "95".into(),
      Color::BrightCyan => "96".into(),
      Color::BrightWhite => "97".into(),
      Color::Reset => "39;49m".into(),
      Color::Color256(n) => format!("38;5;{}m", n).into(),
      Color::TrueColor(r, g, b) => format!("38;2;{};{};{}", r, g, b).into(),
    }
  }

  fn to_bg_str(&self) -> Cow<'static, str> {
    match *self {
      Color::Black => "40".into(),
      Color::Red => "41".into(),
      Color::Green => "42".into(),
      Color::Yellow => "43".into(),
      Color::Blue => "44".into(),
      Color::Magenta => "45".into(),
      Color::Cyan => "46".into(),
      Color::White => "47".into(),
      Color::BrightBlack => "100".into(),
      Color::BrightRed => "101".into(),
      Color::BrightGreen => "102".into(),
      Color::BrightYellow => "103".into(),
      Color::BrightBlue => "104".into(),
      Color::BrightMagenta => "105".into(),
      Color::BrightCyan => "106".into(),
      Color::BrightWhite => "107".into(),
      Color::Reset => "39;49m".into(),
      Color::Color256(n) => format!("48;5;{}", n).into(),
      Color::TrueColor(r, g, b) => format!("48;2;{};{};{}", r, g, b).into(),
    }
  }

  pub fn fg(&self) -> String {
    format!("\x1B[{}m", self.to_fg_str())
  }

  pub fn bg(&self) -> String {
    format!("\x1B[{}m", self.to_bg_str())
  }
}

impl FromStr for Color {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let src = s.to_lowercase();
    let mut result = Err(format!("Invalid Color '{}', see docs for more details", s));

    result = match src.as_ref() {
      "reset" => Ok(Color::Reset),
      "black" => Ok(Color::Black),
      "red" => Ok(Color::Red),
      "green" => Ok(Color::Green),
      "yellow" => Ok(Color::Yellow),
      "blue" => Ok(Color::Blue),
      "magenta" => Ok(Color::Magenta),
      "purple" => Ok(Color::Magenta),
      "cyan" => Ok(Color::Cyan),
      "gray" | "grey" | "white" => Ok(Color::White),
      "bright_black" | "bright black" => Ok(Color::BrightBlack),
      "bright_red" | "bright red" => Ok(Color::BrightRed),
      "bright_green" | "bright green" => Ok(Color::BrightGreen),
      "bright_yellow" | "bright yellow" => Ok(Color::BrightYellow),
      "bright_blue" | "bright blue" => Ok(Color::BrightBlue),
      "bright_magenta" | "bright magenta" => Ok(Color::BrightMagenta),
      "bright_cyan" | "bright cyan" => Ok(Color::BrightCyan),
      "bright_white" | "bright white" => Ok(Color::BrightWhite),
      _ => result
    };

    if let Ok(_) = result {
      return result;
    }

    result = match u8::from_str(&src) {
      Ok(n) => Ok(Color::Color256(n)),
      Err(_) => result
    };

    if let Ok(_) = result {
      return result;
    }

    result = src
      .replace(" ", "")
      .split(",")
      .map(u8::from_str)
      .collect::<Result<Vec<_>, _>>()
      .map_err(|_| result.clone().unwrap_err())
      .map(|it| {
        let mut result = result;

        if it.len() == 3 {
          result = Ok(Color::TrueColor(it[0], it[1], it[2]))
        }

        result
      })
      .flatten();

    result
  }
}