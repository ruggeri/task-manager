use super::{LineBufferState, TerminalLine};
use std::cell::RefCell;
use std::rc::Rc;
use util::ui::UserInterface;

#[derive(Clone, Copy)]
struct Margins {
  left: usize,
  top: usize,
}

pub struct LineBuffer {
  state: RefCell<LineBufferState>,
  ui: Rc<UserInterface>,
}

impl LineBuffer {
  pub fn new(ui: &Rc<UserInterface>) -> LineBuffer {
    LineBuffer {
      state: RefCell::new(LineBufferState::default()),
      ui: Rc::clone(ui),
    }
  }

  pub fn clear_line(&self, idx: usize) {
    self.replace_line(idx, TerminalLine::default());
  }

  fn margins(&self) -> Margins {
    let state = self.state.borrow();
    let num_lines = state.num_lines();
    let max_line_len = state.max_line_len();

    let window = &self.ui.window;
    let window_max_x = window.get_max_x() as usize;
    let window_max_y = window.get_max_y() as usize;

    Margins {
      left: (window_max_x - max_line_len) / 2,
      top: (window_max_y - num_lines) / 2,
    }
  }

  fn num_lines(&self) -> usize {
    self.state.borrow().num_lines()
  }

  pub fn redraw(&self) {
    let margins = self.margins();
    let num_lines = self.num_lines();

    {
      let mut state = self.state.borrow_mut();
      let line_states = state.line_states_mut();
      for (idx, line_state) in line_states.enumerate() {
        if !line_state.is_dirty() {
          continue;
        }

        self.redraw_line(idx, margins, line_state.line());
        line_state.mark_clean();
      }
    }

    self
      .ui
      .window
      .mv((margins.top + num_lines) as i32, margins.left as i32);
  }

  fn redraw_line(
    &self,
    idx: usize,
    margins: Margins,
    line: &TerminalLine,
  ) {
    let window = &self.ui.window;

    window.mv((margins.top + idx) as i32, 0);
    window.clrtoeol();

    window.mv((margins.top + idx) as i32, margins.left as i32);
    window.attron(line.color.to_attr());
    window.printw(&line.text);
    window.attroff(line.color.to_attr());
  }

  pub fn replace_line(&self, idx: usize, new_line: TerminalLine) {
    self.state.borrow_mut().replace_line(idx, new_line);
  }

  pub fn truncate(&self, new_len: usize) {
    self.state.borrow_mut().truncate(new_len);
  }
}
