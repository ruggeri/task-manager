use std::rc::Rc;
use util::ui::Window;
use views::ActiveTasksView;

pub struct Application {
  // TODO: Will someday become multiple views.
  pub view: Rc<ActiveTasksView>,
  pub window: Rc<Window>,
}

impl Application {
  pub fn new() -> Application {
    let window = Rc::new(Window::initscr());
    Application {
      view: ActiveTasksView::new(&window),
      window,
    }
  }

  pub fn run(&mut self) {
    loop {
      let ch = match self.window.getch() {
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
