use pancurses;
use rustyline::{error::ReadlineError, Editor};
use std::io::{stdout, Write};

#[repr(C)]
pub enum ColorPair {
  Default,
  Highlight,
}

pub struct UserInterface {
  pub window: pancurses::Window,
}

impl UserInterface {
  pub fn initscr() -> UserInterface {
    // Important! You must initscr before you can do any of the start
    // color stuff. Otherwise you get a wonderful segfault...
    let window = pancurses::initscr();

    pancurses::start_color();
    pancurses::use_default_colors();
    pancurses::init_pair(ColorPair::Default as i16, -1, -1);
    pancurses::init_pair(ColorPair::Highlight as i16, -1, pancurses::COLOR_BLUE);
    pancurses::noecho();
    window.keypad(true);

    UserInterface { window }
  }

  pub fn getch(&self) -> Option<char> {
    let result = self.window.getch();

    match result {
      None => {
        // wgetch had some problem. I believe one such problem happens
        // when the window size changes.
        //
        // Could try to handle SIGWINCH. Should redraw the display. But
        // right now my program has fixed with output regardless of
        // display size.

        None
      }
      // A character
      Some(pancurses::Input::Character(ch)) => Some(ch),
      // Not a character
      Some(_) => None,
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
        }
        // Corresponds to Ctrl-C
        Err(ReadlineError::Interrupted) => {
          pancurses::noecho();
          // Super hacky. Otherwise "Ctrl-C" moves the cursor straight
          // one line down. So here I move back a line and delete it.
          {
            let mut out = stdout();
            out.write_all(b"\x1b[F\x1b[K").unwrap();
            out.flush().unwrap();
          }

          return None;
        }
        Err(_) => continue,
      }
    }
  }
}

impl Drop for UserInterface {
  fn drop(&mut self) {
    pancurses::endwin();
  }
}
