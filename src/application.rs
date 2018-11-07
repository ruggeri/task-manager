use std::cell::Cell;
use std::rc::Rc;
use util::ui::Window;
use views::ActiveTasksView;

pub struct Application {
  pub shutdown_requested: Cell<bool>,
  // TODO: Will someday become multiple views.
  pub view: ActiveTasksView,
  pub window: Rc<Window>,
}

impl Application {
  pub fn new() -> Application {
    let window = Rc::new(Window::new());
    Application {
      view: ActiveTasksView::new(&window),
      shutdown_requested: Cell::new(false),
      window,
    }
  }

  pub fn run(&mut self) {
    while !self.shutdown_requested.get() {
      let ch = match self.window.getch() {
        None => continue,
        Some(ch) => ch,
      };

      if ch == 'q' {
        break;
      }

      // TODO: eventually must handle multiple views.
      self.view.handle_key(ch);
    }
  }
}
