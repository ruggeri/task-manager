use ::util::ui::{ColorPair, UserInterface};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq)]
pub struct Line {
  pub text: String,
  pub color: ColorPair,
}

impl Line {
  pub fn default() -> Line {
    Line { text: String::new(), color: ColorPair::Default }
  }

  pub fn len(&self) -> usize {
    self.text.len()
  }
}

#[derive(Clone)]
pub enum LineUpdate {
  NoUpdate { old_line: Line },
  RedrawLine { new_line: Line },
}

impl LineUpdate {
  pub fn make_no_update(&mut self) {
    use self::LineUpdate::*;

    let mut no_update_line = Line::default();
    {
      let redraw_line = match self {
        NoUpdate { .. } => return,
        RedrawLine { new_line } => new_line
      };

      mem::swap(redraw_line, &mut no_update_line);
    }

    *self = NoUpdate { old_line: no_update_line };
  }

  pub fn make_redraw(&mut self) {
    use self::LineUpdate::*;

    let mut redraw_line = Line::default();
    {
      let no_update_line = match self {
        NoUpdate { old_line } => old_line,
        RedrawLine { .. } => return
      };

      mem::swap(no_update_line, &mut redraw_line);
    }

    *self = RedrawLine { new_line: redraw_line };
  }
}

struct LineBufferState {
  line_updates: Vec<LineUpdate>,
  max_line_len: usize,
}

impl LineBufferState {
  pub fn default() -> LineBufferState {
    LineBufferState { line_updates: Vec::new(), max_line_len: 0 }
  }
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

  fn mark_all_dirty(&self) {
    let mut state = self.state.borrow_mut();
    for line_update in state.line_updates.iter_mut() {
      line_update.make_redraw();
    }
  }

  fn recalculate_max_line_len(&self) {
    use self::LineUpdate::*;
    let mut state = self.state.borrow_mut();

    // Recalculate max line len.
    let old_max_line_len = state.max_line_len;
    let mut new_max_line_len = 0;
    for line_update in &state.line_updates {
      let line = match line_update {
        NoUpdate { old_line } => old_line,
        RedrawLine { new_line } => new_line,
      };

      if line.len() > state.max_line_len {
        new_max_line_len = line.len()
      }
    }

    // Set it.
    state.max_line_len = new_max_line_len;

    // No more modificatons here.
    drop(state);

    // Possibly mark everyone as dirty.
    if new_max_line_len != old_max_line_len {
      self.mark_all_dirty();
    }
  }

  pub fn set_line(&self, idx: usize, new_line: Line) {
    use self::LineUpdate::*;
    let mut state = self.state.borrow_mut();

    // Possibly resize if line is far down.
    if idx >= state.line_updates.len() {
      state.line_updates.resize(
        idx + 1,
        NoUpdate { old_line: Line::default() }
      );
    }

    let replaced_line_len = {
      let replaced_line_update = &state.line_updates[idx];
      let replaced_line = match replaced_line_update {
        NoUpdate { old_line } => old_line,
        RedrawLine { new_line } => new_line
      };

      if *replaced_line == new_line {
        return
      }

      replaced_line.len()
    };

    let new_line_len = new_line.len();
    state.line_updates[idx] = RedrawLine { new_line };

    if new_line_len > state.max_line_len {
      state.max_line_len = new_line_len;
      drop(state);
      // Everyone is dirty now that line len changed.
      self.mark_all_dirty();
    } else if (replaced_line_len == state.max_line_len)
      && (new_line_len < replaced_line_len) {
      // Maybe reduced max line len. May have to redraw all.
      drop(state);
      self.recalculate_max_line_len();
    }
  }

  pub fn clear_line(&self, idx: usize) {
    self.set_line(idx, Line::default());
  }

  pub fn redraw(&self) {
    use self::LineUpdate::*;
    let mut state = self.state.borrow_mut();
    let window = &self.ui.window;

    let margin_left = (window.get_max_x() as usize - state.max_line_len) / 2;

    for (idx, line_update) in state.line_updates.iter_mut().enumerate() {
      {
        let new_line = match line_update {
          NoUpdate { .. } => continue,
          RedrawLine { new_line } => new_line
        };

        self.redraw_line(idx, margin_left, new_line);
      }

      line_update.make_no_update();
    }

    window.mv(state.line_updates.len() as i32, margin_left as i32);
  }

  fn redraw_line(&self, idx: usize, margin_left: usize, line: &Line) {
    let window = &self.ui.window;

    window.mv(idx as i32, margin_left as i32);

    window.attron(line.color.to_attr());
    window.printw(&line.text);
    window.attroff(line.color.to_attr());
  }
}
