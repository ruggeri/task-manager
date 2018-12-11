use util::ui::ColorPair;

#[derive(Clone, PartialEq, Eq)]
pub struct TerminalLine {
  pub text: String,
  pub color: ColorPair,
}

impl TerminalLine {
  pub fn len(&self) -> usize {
    self.text.len()
  }
}

impl Default for TerminalLine {
  fn default() -> TerminalLine {
    TerminalLine {
      text: String::new(),
      color: ColorPair::Default,
    }
  }
}
