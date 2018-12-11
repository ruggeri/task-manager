use super::TerminalLine;

pub enum LineChange {
  LineDidChange { old_line: TerminalLine },
  LineDidNotChange,
}

#[derive(Clone)]
pub struct TerminalLineState {
  line: TerminalLine,
  is_dirty: bool,
}

impl TerminalLineState {
  pub fn is_dirty(&self) -> bool {
    self.is_dirty
  }

  pub fn line(&self) -> &TerminalLine {
    &self.line
  }

  pub fn line_len(&self) -> usize {
    self.line.len()
  }

  pub fn mark_clean(&mut self) {
    self.is_dirty = false;
  }

  pub fn mark_dirty(&mut self) {
    self.is_dirty = true;
  }

  pub fn replace_line(&mut self, new_line: TerminalLine) -> LineChange {
    if self.line == new_line {
      return LineChange::LineDidNotChange;
    }

    let old_line = ::std::mem::replace(&mut self.line, new_line);
    self.mark_dirty();

    LineChange::LineDidChange { old_line }
  }
}

impl Default for TerminalLineState {
  fn default() -> TerminalLineState {
    TerminalLineState {
      line: TerminalLine::default(),
      is_dirty: false,
    }
  }
}
