extern crate termion;

use std::error::Error;
use std::default::Default;

use termion::color;
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, stdin};

fn main() {
  let stdout = stdout();
  let mut stdout = stdout.lock().into_raw_mode().unwrap();
  let stdin = stdin();
  let stdin = stdin.lock();


  let min_coord = 0;
  let max_coord = 2;

  let mut x = 1;
  let mut y = 1;

  let mut bytes = stdin.bytes();

  let mut you_said  = "";
  let mut they_said = "";

  loop {
    let str = format!("You are in Animal Town, in acre {}-{}.", x, y);

    write!(
      stdout,
      "{}{}{}{}{}",
      termion::clear::All,
      termion::cursor::Goto(2, 2),
      termion::style::Bold,
      &str,
      termion::style::Reset,
    ).unwrap();

    if x == 2 && y == 0 {
      write!(
        stdout,
        "{}{}{}",
        termion::cursor::Goto(2, 4),
        "Bob the Cat is here.  Press 't' to talk to Bob.",
        termion::style::Reset,
      ).unwrap();
    }

    if ! you_said.is_empty() && ! they_said.is_empty() {
      write!(
        stdout,
        "{}{}You said:  {}{}{}",
        termion::cursor::Goto(2, 6),
        termion::style::Bold,
        color::Fg(color::LightBlue),
        &you_said,
        termion::style::Reset,
      ).unwrap();

      write!(
        stdout,
        "{}{}They said: {}{}{}",
        termion::cursor::Goto(2, 7),
        termion::style::Bold,
        color::Fg(color::LightMagenta),
        &you_said,
        termion::style::Reset,
      ).unwrap();
    }

    you_said  = "";
    they_said = "";

    write!(
      stdout,
      "{}{}{}{}",
      termion::cursor::Goto(2, 21),
      termion::style::Bold,
      "Use 'wasd' to move.  Press 'q' to quit.",
      termion::style::Reset,
    ).unwrap();

    stdout.flush().unwrap();

    let b = bytes.next().unwrap().unwrap();

    match b {
      // Quit
      b'q' => { break; },

      b'w' => { if y < max_coord { y = y + 1; } }
      b's' => { if y > min_coord { y = y - 1; } }

      b'a' => { if x > min_coord { x = x - 1; } }
      b'd' => { if x < max_coord { x = x + 1; } }

      b't' => {
        if x == 2 && y == 0 {
          you_said  = "Hi, Bob!";
          they_said = "Mew.";
        }
      }

      _ => {}
    };

    stdout.flush().unwrap();
  }

}

