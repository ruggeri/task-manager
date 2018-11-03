use pancurses;
use rustyline::{Editor, error::ReadlineError};
use std::io::{stdout, Write};

#[repr(C)]
pub enum ColorPair {
  Default,
  Highlight,
}

pub struct Window {
  pub window: pancurses::Window,
}

impl Window {
  #![allow(new_without_default)]
  pub fn new() -> Window {
    // Important! You must initscr before you can do any of the start
    // color stuff. Otherwise you get a wonderful segfault...
    let window = pancurses::initscr();

    pancurses::start_color();
    pancurses::use_default_colors();
    pancurses::init_pair(ColorPair::Default as i16, -1, -1);
    pancurses::init_pair(ColorPair::Highlight as i16, -1, pancurses::COLOR_BLUE);
    pancurses::noecho();
    window.keypad(true);

    Window { window }
  }

  pub fn getch(&self) -> Option<char> {
    match self.window.getch().unwrap() {
      pancurses::Input::Character(ch) => Some(ch),
      _ => None,
    }
  }

  pub fn read_line(&self, prompt: &str) -> Option<String> {
    pancurses::echo();
    loop {
      let mut editor = Editor::<()>::new();
      match editor.readline(prompt) {
        Ok(line) => {
          pancurses::noecho();
          return Some(line);
        },
        // Corresponds to Ctrl-C
        Err(ReadlineError::Interrupted) => {
          pancurses::noecho();
          // Super hacky. Otherwise "Ctrl-C" moves the cursor straight
          // one line down. So here I move back a line and delete it.
          {
            let mut out = stdout();
            out.write(b"\x1b[F\x1b[K").unwrap();
            out.flush().unwrap();
          }

          return None
        },
        Err(_) => continue,
      }
    }
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    pancurses::endwin();
  }
}
