extern crate array_init;
extern crate termion;

use termion::color;
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, stdin};

const MAP_EDGE_SIZE: usize = 2;

#[derive(Debug, Clone)]
struct Animal {
  name:     String,
  you_say:  String,
  they_say: String,
}

#[derive(Debug, Clone)]
struct Acre {
  animal: Option<Animal>,
}

impl Default for Acre {
  fn default () -> Acre {
    Acre { animal: None }
  }
}

fn main() {
  let stdout = stdout();
  let mut stdout = stdout.lock().into_raw_mode().unwrap();
  let stdin = stdin();
  let stdin = stdin.lock();

  // let mut map = [[Acre::default(); MAP_EDGE_SIZE+1]; MAP_EDGE_SIZE+1];
  let mut map : [ [Acre; MAP_EDGE_SIZE+1]; MAP_EDGE_SIZE+1 ] = array_init::array_init(
    |_i| array_init::array_init(|_j| Acre::default())
  );

  map[2][0].animal = Some(Animal {
    name:     "Bob".to_string(),
    you_say:  "Hi, Bob!".to_string(),
    they_say: "Mew.".to_string(),
  });

  map[0][1].animal = Some(Animal {
    name:     "Lobo".to_string(),
    you_say:  "Heya, Lobo!".to_string(),
    they_say: "Woof!.".to_string(),
  });

  let mut x = 1;
  let mut y = 1;

  let mut bytes = stdin.bytes();

  let mut you_said  = String::new();
  let mut they_said = String::new();

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

    if let Some(ref animal) = map[x][y].animal {
      write!(
        stdout,
        "{}{}{}{}.{}",
        termion::cursor::Goto(2, 4),
        animal.name,
        " is here.  Press 't' to talk to ",
        animal.name,
        termion::style::Reset,
      ).unwrap();
    }

    if ! you_said.is_empty() && ! they_said.is_empty() {
      write!(
        stdout,
        "{}{}You said:  {}{}{}",
        termion::cursor::Goto(2, 6),
        termion::style::Bold,
        color::Fg(color::LightCyan),
        &you_said,
        termion::style::Reset,
      ).unwrap();

      write!(
        stdout,
        "{}{}They said: {}{}{}",
        termion::cursor::Goto(2, 7),
        termion::style::Bold,
        color::Fg(color::LightGreen),
        &they_said,
        termion::style::Reset,
      ).unwrap();
    }

    you_said  = String::new();
    they_said = String::new();

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

      b'w' => { if y < MAP_EDGE_SIZE { y = y + 1; } }
      b's' => { if y > 0             { y = y - 1; } }

      b'a' => { if x > 0             { x = x - 1; } }
      b'd' => { if x < MAP_EDGE_SIZE { x = x + 1; } }

      b't' => {
        if let Some(ref animal) = map[x][y].animal {
          let a = &animal.you_say;
          let b = &animal.they_say;
          you_said  = a.clone();
          they_said = b.clone();
        }
      }

      _ => {}
    };

    stdout.flush().unwrap();
  }

}
