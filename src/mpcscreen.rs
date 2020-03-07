
extern crate termion;

use termion::color;
// use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, stdin};
use termion::raw::*;

pub struct MpcScreen<'a> {
  stdout: RawTerminal<std::io::StdoutLock<'a>>,
}

impl<'a> MpcScreen<'a> {

  pub fn new(out: &'a std::io::Stdout) -> MpcScreen<'a> {
    let mut res = out.lock().into_raw_mode();
    if res.is_err() {
      panic!("Unable to open stdout.");
    };
    let mut stdout = res.unwrap();
    MpcScreen {stdout}
  }

  pub fn flush(&mut self) {
    write!(self.stdout, "{}{}{}", color::Fg(color::Reset), color::Bg(color::Reset), termion::cursor::Goto(1, 1) ).unwrap();
    self.stdout.flush().unwrap();
  }

  pub fn clean(&mut self) {
      write!(self.stdout, "{}", termion::clear::All ).unwrap();
  }

  pub fn colored<C: termion::color::Color>(&mut self, x: u16 , y: u16, col: C) {
      write!(self.stdout,
             "{}{}\u{2588}{}{}",
             termion::cursor::Goto(x, y),
             color::Fg(col),
             color::Fg(color::Reset),
             termion::cursor::Goto(x, y)
             ).unwrap();
  }

  pub fn line<C: termion::color::Color>(&mut self, x: u16 , y: u16, line: &str, col: C) {
      write!(self.stdout,
             "{}{}{}{}{}",
             termion::cursor::Goto(x, y),
             color::Fg(col),
             line,
             termion::cursor::Goto(x, y),
             color::Fg(color::Reset)
             ).unwrap();
  }

  pub fn bgline<C: termion::color::Color>(&mut self, x: u16 , y: u16, line: &str, col: C) {
      write!(self.stdout,
             "{}{}{}{}{}{}{}",
             termion::cursor::Goto(x, y),
             color::Bg(col),
             color::Fg(color::Rgb(255,255,255)),
             line,
             termion::cursor::Goto(x, y),
             color::Fg(color::Reset),
             color::Bg(color::Reset)
             ).unwrap();
  }

  pub fn uline<C: termion::color::Color>(&mut self, x: u16 , y: u16, line: &str, col: C) {
      write!(self.stdout,
             "{}{}{}{}{}{}{}{}{}",
             termion::cursor::Goto(x, y),
             color::Fg(col),
             termion::style::Bold,
             termion::style::Underline,
             line,
             termion::cursor::Goto(x, y),
             termion::style::NoBold,
             termion::style::NoUnderline,
             color::Fg(color::Reset)
             ).unwrap();
  }

}
