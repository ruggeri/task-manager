use pancurses;

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

    Window { window }
  }

  pub fn getch(&self) -> Option<char> {
    match self.window.getch().unwrap() {
      pancurses::Input::Character(ch) => Some(ch),
      _ => None,
    }
  }

  pub fn read_line(&self, prompt: &str) -> String {
    self.window.printw(prompt);
    let mut line = String::new();

    loop {
      use pancurses::Input::*;

      match self.window.getch().unwrap() {
        Character('\n') => break,
        Character('\x7f') => {
          if line.is_empty() {
            continue;
          }

          line.pop();

          let (y, x) = self.window.get_cur_yx();
          self.window.mv(y, x - 1);
          self.window.delch();
        }
        Character(c) => {
          self.window.addch(c);
          line.push(c);
        }
        _ => continue,
      }
    }

    line
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    pancurses::endwin();
  }
}
