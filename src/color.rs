#![allow(unused)]

use std::borrow::Cow;
use std::str::FromStr;

static mut _SUPPORTS_COLOR: bool = true;
pub static SUPPORTS_COLOR: &bool = unsafe { &_SUPPORTS_COLOR };

pub enum ColorMode {
  Auto,
  Always,
  None,
}

pub fn set_color_mode(mode: ColorMode) {
  unsafe {
    _SUPPORTS_COLOR = match mode {
      ColorMode::Auto => true, // TODO
      ColorMode::Always => true,
      ColorMode::None => false,
    }
  }
}


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
  TrueColor(u8, u8, u8),
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
      Color::Reset => "39;49".into(),
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
      Color::Reset => "39;49".into(),
      Color::Color256(n) => format!("48;5;{}", n).into(),
      Color::TrueColor(r, g, b) => format!("48;2;{};{};{}", r, g, b).into(),
    }
  }

  pub fn fg(&self) -> String {
    if *SUPPORTS_COLOR {
      format!("\x1B[{}m", self.to_fg_str())
    } else {
      format!("")
    }
  }

  pub fn bg(&self) -> String {
    if *SUPPORTS_COLOR {
      format!("\x1B[{}m", self.to_bg_str())
    } else {
      format!("")
    }
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
      "magenta" | "purple" => Ok(Color::Magenta),
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

pub trait Colorize: Sized {
  fn with_fg(self, color: Color) -> String;
  fn with_bg(self, color: Color) -> String;

  fn black(self) -> String { self.with_fg(Color::Black) }
  fn red(self) -> String { self.with_fg(Color::Red) }
  fn green(self) -> String { self.with_fg(Color::Green) }
  fn yellow(self) -> String { self.with_fg(Color::Yellow) }
  fn blue(self) -> String { self.with_fg(Color::Blue) }
  fn magenta(self) -> String { self.with_fg(Color::Magenta) }
  fn cyan(self) -> String { self.with_fg(Color::Cyan) }
  fn white(self) -> String { self.with_fg(Color::White) }
  fn grey(self) -> String { self.white() }
  fn gray(self) -> String { self.white() }
  fn bright_black(self) -> String { self.with_fg(Color::BrightBlack) }
  fn bright_red(self) -> String { self.with_fg(Color::BrightRed) }
  fn bright_green(self) -> String { self.with_fg(Color::BrightGreen) }
  fn bright_yellow(self) -> String { self.with_fg(Color::BrightYellow) }
  fn bright_blue(self) -> String { self.with_fg(Color::BrightBlue) }
  fn bright_magenta(self) -> String { self.with_fg(Color::BrightMagenta) }
  fn bright_cyan(self) -> String { self.with_fg(Color::BrightCyan) }
  fn bright_white(self) -> String { self.with_fg(Color::BrightWhite) }

  fn on_black(self) -> String { self.with_bg(Color::Black) }
  fn on_red(self) -> String { self.with_bg(Color::Red) }
  fn on_green(self) -> String { self.with_bg(Color::Green) }
  fn on_yellow(self) -> String { self.with_bg(Color::Yellow) }
  fn on_blue(self) -> String { self.with_bg(Color::Blue) }
  fn on_magenta(self) -> String { self.with_bg(Color::Magenta) }
  fn on_cyan(self) -> String { self.with_bg(Color::Cyan) }
  fn on_white(self) -> String { self.with_bg(Color::White) }
  fn on_grey(self) -> String { self.on_white() }
  fn on_gray(self) -> String { self.on_white() }
  fn on_bright_black(self) -> String { self.with_bg(Color::BrightBlack) }
  fn on_bright_red(self) -> String { self.with_bg(Color::BrightRed) }
  fn on_bright_green(self) -> String { self.with_bg(Color::BrightGreen) }
  fn on_bright_yellow(self) -> String { self.with_bg(Color::BrightYellow) }
  fn on_bright_blue(self) -> String { self.with_bg(Color::BrightBlue) }
  fn on_bright_magenta(self) -> String { self.with_bg(Color::BrightMagenta) }
  fn on_bright_cyan(self) -> String { self.with_bg(Color::BrightCyan) }
  fn on_bright_white(self) -> String { self.with_bg(Color::BrightWhite) }
}

impl Colorize for &str {
  fn with_fg(self, color: Color) -> String {
    format!("{}{}{}", color.fg(), self, Color::Reset.fg())
  }

  fn with_bg(self, color: Color) -> String {
    format!("{}{}{}", color.bg(), self, Color::Reset.bg())
  }
}

impl Colorize for String {
  fn with_fg(self, color: Color) -> String {
    format!("{}{}", color.fg(), self)
  }

  fn with_bg(self, color: Color) -> String {
    format!("{}{}", color.bg(), self)
  }
}