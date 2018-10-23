extern crate rustbox;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

fn main() {
  let rustbox = match RustBox::init(Default::default()) {
    Result::Ok(v)  => v,
    Result::Err(e) => panic!("{}", e),
  };

  let min_coord = 0;
  let max_coord = 2;

  let mut x = 1;
  let mut y = 1;

  loop {
    let str = format!("You are in Animal Town, in acre {}-{}.", x, y);
    rustbox.print(
      1, 1,
      rustbox::RB_BOLD, Color::White, Color::Black,
      &str
    );

    if x == 2 && y == 0 {
      rustbox.print(
        1, 3,
        rustbox::RB_NORMAL, Color::White, Color::Black,
        "Bob the Cat is here.",
      );
    }

    rustbox.print(1, 20, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Use 'wasd' to move.  Press 'q' to quit.");

    rustbox.present();
    match rustbox.poll_event(false) {
      Ok(rustbox::Event::KeyEvent(key)) => {
        match key {
          Key::Char('q') => { break; }

          Key::Char('w') => { if y < max_coord { y = y + 1; } }
          Key::Char('s') => { if y > min_coord { y = y - 1; } }

          Key::Char('a') => { if x > min_coord { x = x - 1; } }
          Key::Char('d') => { if x < max_coord { x = x + 1; } }

          _ => { }
        }
      },
      Err(e) => panic!("{}", e.description()),
        _ => { }
    }
  }
}

