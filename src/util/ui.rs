use pancurses;
use rustyline::{error::ReadlineError, Editor};
use std::io::{stdout, Write};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColorPair {
  Default,
  Highlight,
  Bold,
}

impl ColorPair {
  pub fn to_attr(self) -> u32 {
    use self::ColorPair::*;

    match self {
      Default => pancurses::COLOR_PAIR(self as u32),
      Highlight => pancurses::COLOR_PAIR(self as u32),
      Bold => pancurses::A_BOLD,
    }
  }
}

pub struct UserInterface {
  pub(super) window: pancurses::Window,
}

impl UserInterface {
  pub fn initscr() -> UserInterface {
    // Important! You must initscr before you can do any of the start
    // color stuff. Otherwise you get a wonderful segfault...
    let window = pancurses::initscr();

    pancurses::start_color();
    pancurses::use_default_colors();
    // -1 means "default color".
    pancurses::init_pair(ColorPair::Default as i16, -1, -1);
    pancurses::init_pair(
      ColorPair::Highlight as i16,
      -1,
      pancurses::COLOR_BLUE,
    );
    pancurses::noecho();
    // Keypad mode handles escape sequences. I think how it works is
    // waits a small amount of time to get all characters that occur
    // after the escape character.
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
    let result = loop {
      let mut editor = Editor::<()>::new();
      match editor.readline(prompt) {
        Ok(line) => {
          break Some(line);
        }

        // Corresponds to Ctrl-C
        Err(ReadlineError::Interrupted) => {
          break None;
        }
        Err(_) => continue,
      }
    };
    pancurses::noecho();

    // Super hacky. readline moves cursor without pancurses knowing.
    {
      // y, x is the position to restore. Since pancurses doesn't know,
      // the "cur_yx" is actually where we were before readline.
      let (y, x) = self.window.get_cur_yx();
      let mut out = stdout();
      // Scroll back, then clear the line.
      out.write_all(b"\x1b[F\x1b[2K").unwrap();
      // Then reposition. Is this freaking one based???
      out.write_all(format!("\x1b[{};{}H", y + 1, x + 1).as_bytes()).unwrap();
      out.flush().unwrap();
    }

    result
  }
}

impl Drop for UserInterface {
  fn drop(&mut self) {
    pancurses::endwin();
  }
}
