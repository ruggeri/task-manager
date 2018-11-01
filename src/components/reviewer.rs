use actions::ActionResult;
use commands::Command;
use super::DataSource;
use super::Scroller;
use super::TaskResultsWindow;
use util::{get_connection, ui::Window};
use diesel::pg::PgConnection;
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
    let connection = Rc::new(get_connection());
    let data_source = DataSource::new(&connection);
    let window = Rc::new(Window::new());
    let scroller = Rc::new(Scroller::new(vec![], max_tasks));
    let task_results_window = Rc::new(TaskResultsWindow::new(&window, &scroller));

    let mut reviewer = Reviewer {
      connection,
      data_source,
      window,
      scroller,
      task_results_window,
    };

    let scroller2 = Rc::clone(&reviewer.scroller);
    reviewer.data_source.add_callback(Box::new(move |results| {
      scroller2.refresh(results.clone());
    }));
    reviewer.data_source.refresh();

    reviewer
  }

  pub fn run(&mut self) {
    self.task_results_window.redraw();

    loop {
      let ch = match self.window.getch() {
        None => continue,
        Some(ch) => ch,
      };

      let action_result = Command::from_key(ch).and_then(|cmd| {
        cmd.to_action(self)
      }).map(|mut action| {
        action.execute(self)
      });

      use self::ActionResult::*;
      match action_result {
        None => {}
        Some(DidNothing) => {}
        Some(DidUpdateScroller) => {
          self.task_results_window.redraw();
        }
        Some(DidUpdateTaskData) => {
          self.data_source.refresh();
          self.task_results_window.redraw();
        }
        Some(RequestedShutDown) => {
          break;
        }
      }
    }
  }
}
