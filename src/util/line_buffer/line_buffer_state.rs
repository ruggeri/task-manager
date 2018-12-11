use super::{LineChange, TerminalLine, TerminalLineState};

#[must_use]
enum DimensionsStatus {
  DimensionsChangeForcesFullRedraw,
  DimensionsDidNotChange,
}

impl DimensionsStatus {
  pub fn if_dims_changed<F: FnMut()>(self, mut f: F) {
    use self::DimensionsStatus::*;

    if let DimensionsChangeForcesFullRedraw = self {
      f()
    }
  }

  // Used simply to ignore the dimension status change.
  pub fn ignore_dims_change(self) {}
}

pub struct LineBufferState {
  line_states: Vec<TerminalLineState>,
  max_line_len: usize,
}

impl LineBufferState {
  pub fn default() -> LineBufferState {
    LineBufferState {
      line_states: Vec::new(),
      max_line_len: 0,
    }
  }

  // Resizes the buffer with more blank lines if needed.
  fn extend_num_lines(
    &mut self,
    new_num_lines: usize,
  ) -> DimensionsStatus {
    let old_num_lines = self.line_states.len();
    if new_num_lines <= old_num_lines {
      return DimensionsStatus::DimensionsDidNotChange;
    }

    self
      .line_states
      .resize(new_num_lines, TerminalLineState::default());

    DimensionsStatus::DimensionsChangeForcesFullRedraw
  }

  // Handles a possible change in width of a line. This must trigger a
  // redraw if the width dimensions change.
  fn handle_line_width_change(
    &mut self,
    old_line_len: usize,
    new_line_len: usize,
  ) -> DimensionsStatus {
    if new_line_len > self.max_line_len {
      self.max_line_len = new_line_len;
      DimensionsStatus::DimensionsChangeForcesFullRedraw
    } else if (new_line_len < old_line_len)
      && (old_line_len == self.max_line_len)
    {
      // Maybe *reduced* the max line len. Forces a full recalculation
      // of max_line_len.
      self.recalculate_max_line_len()
    } else {
      DimensionsStatus::DimensionsDidNotChange
    }
  }

  pub fn line_states_mut(
    &mut self,
  ) -> impl Iterator<Item = &mut TerminalLineState> {
    self.line_states.iter_mut()
  }

  // Marks all lines as dirty, thus triggering a full redraw.
  fn mark_all_dirty(&mut self) {
    for line_state in &mut self.line_states {
      line_state.mark_dirty();
    }
  }

  pub fn max_line_len(&self) -> usize {
    self.max_line_len
  }

  pub fn num_lines(&self) -> usize {
    self.line_states.len()
  }

  // Iterates the lines and calculates the maximum line length. Not
  // often needed.
  fn recalculate_max_line_len(&mut self) -> DimensionsStatus {
    // Recalculate max line len.
    let old_max_line_len = self.max_line_len;
    let mut new_max_line_len = 0;
    for line_state in &self.line_states {
      let line_len = line_state.line_len();
      if line_len > new_max_line_len {
        new_max_line_len = line_len
      }
    }

    if new_max_line_len != old_max_line_len {
      self.max_line_len = new_max_line_len;
      DimensionsStatus::DimensionsChangeForcesFullRedraw
    } else {
      DimensionsStatus::DimensionsDidNotChange
    }
  }

  // Replaces the line at the given index. May trigger dimensions
  // changes, in which case all lines may be marked dirty.
  pub fn replace_line(&mut self, idx: usize, new_line: TerminalLine) {
    let mut should_mark_all_dirty = false;

    // Possibly resize if line is far down.
    self.extend_num_lines(idx + 1).if_dims_changed(|| {
      should_mark_all_dirty = true;
    });

    // Get the old/new lengths of the line.
    let new_line_len = new_line.len();
    let line_change = self.line_states[idx].replace_line(new_line);
    let old_line_len = match line_change {
      LineChange::LineDidChange { old_line } => old_line.len(),
      // If line did not change we can return.
      LineChange::LineDidNotChange => return,
    };

    // TODO: if the line becomes empty, maybe I should also treat that
    // as a *shrinking* of the screen size??

    // Handle the change in line width (if any).
    self
      .handle_line_width_change(old_line_len, new_line_len)
      .if_dims_changed(|| {
        should_mark_all_dirty = true;
      });

    if should_mark_all_dirty {
      self.mark_all_dirty();
    }
  }

  pub fn truncate(&mut self, new_len: usize) {
    // TODO: I am explicitly avoiding truncation because that makes the
    // screen smaller and that is actually kind of challenging...
    for idx in new_len..(self.num_lines()) {
      self.replace_line(idx, TerminalLine::default());
    }
  }
}
