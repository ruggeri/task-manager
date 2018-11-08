use std::rc::Rc;
use util::ui::UserInterface;
use views::ActiveTasksView;

pub struct Application {
  // TODO: Will someday become multiple views.
  view: Rc<ActiveTasksView>,
  ui: Rc<UserInterface>,
}

impl Application {
  pub fn new() -> Application {
    let ui = Rc::new(UserInterface::initscr());
    Application {
      view: ActiveTasksView::new(&ui),
      ui,
    }
  }

  pub fn run(&mut self) {
    loop {
      let ch = match self.ui.getch() {
        None => continue,
        Some(ch) => ch,
      };

      if ch == 'q' {
        break;
      }

      ActiveTasksView::handle_key(&self.view, ch);
    }
  }
}

impl Default for Application {
  fn default() -> Application {
    Application::new()
  }
}
