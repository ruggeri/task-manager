use super::commands::{CommandResult, Commands};
use super::data_source::DataSource;
use super::scroller::Scroller;
use super::task_results_window::TaskResultsWindow;
use super::ui::Window;
use diesel::pg::PgConnection;
use connection;
use std::rc::Rc;

pub struct Reviewer {
  pub connection: Rc<PgConnection>,
  pub window: Rc<Window>,
  pub task_results_window: Rc<TaskResultsWindow>,
  pub scroller: Rc<Scroller>,
  pub data_source: DataSource,
}

impl Reviewer {
  pub fn new(max_tasks: usize) -> Reviewer {
    let connection = Rc::new(connection::get());
    let mut data_source = DataSource::new(&connection);
    let window = Rc::new(Window::new());
    let scroller = Rc::new(Scroller::new(vec![], max_tasks));
    let task_results_window = Rc::new(TaskResultsWindow::new(&window, &scroller));

    data_source.refresh();
    // TODO: Need to install callback to pump down to scroller et al.

    Reviewer {
      connection,
      data_source,
      window,
      scroller,
      task_results_window
    }
  }

  pub fn run(&mut self) {
    self.task_results_window.redraw();

    loop {
      let ch = match self.window.getch() {
        None => continue,
        Some(ch) => ch
      };

      use self::CommandResult::*;
      match Commands::handle_key(self, ch) {
        DidNothing => {},
        DidUpdateScroller => {
          self.task_results_window.redraw();
        },
        DidUpdateTaskData => {
          self.data_source.refresh();
          self.task_results_window.redraw();
        },
        RequestedShutDown => {
          break;
        }
      }
    }
  }
}
