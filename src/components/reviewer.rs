use actions::ActionResult;
use commands::Command;
use components::{AttributeFilter, DataSource, Scroller, TaskResultsWindow, UndoBuffer};
use diesel::pg::PgConnection;
use std::rc::Rc;
use util::{get_connection, ui::Window};

pub struct Reviewer {
  pub connection: Rc<PgConnection>,
  pub window: Rc<Window>,
  pub task_results_window: TaskResultsWindow,
  pub scroller: Rc<Scroller>,
  pub data_source: DataSource,
  pub undo_buffer: UndoBuffer,
  pub filterer: Rc<AttributeFilter>
}

impl Reviewer {
  pub fn new(max_results_to_display: usize) -> Reviewer {
    let connection = Rc::new(get_connection());
    let mut data_source = DataSource::new(&connection);
    let window = Rc::new(Window::new());
    let scroller = Rc::new(Scroller::new(max_results_to_display));
    // TODO: I prolly don't need to make either window or scroller Rc if
    // I: (1) Pump scroller refreshes into window, rather than pull, (2)
    // I pass &window to redraw.
    let task_results_window = TaskResultsWindow::new(&window, &scroller);
    let undo_buffer = UndoBuffer::new();
    let mut filterer = AttributeFilter::new();

    // Scroller pulls from Filterer.
    {
      let scroller = Rc::clone(&scroller);
      filterer.add_callback(Box::new(move |filtered_results| {
        scroller.refresh(filtered_results);
      }));
    }
    let filterer = Rc::new(filterer);
    // Filterer listens to DataSource.
    {
      let filterer = Rc::clone(&filterer);
      data_source.add_callback(Box::new(move |results| {
        filterer.refresh(results);
      }));
    }

    Reviewer {
      connection,
      data_source,
      window,
      scroller,
      task_results_window,
      undo_buffer,
      filterer
    }
  }

  pub fn run(&mut self) {
    self.data_source.refresh();
    self.task_results_window.redraw();

    loop {
      let ch = match self.window.getch() {
        None => continue,
        Some(ch) => ch,
      };

      let action_result = Command::from_key(ch)
        .and_then(|cmd| cmd.to_action(self))
        .map(|mut action| {
          let result = action.execute(self);

          if action.can_be_unexecuted() {
            self.undo_buffer.append_action(action);
          }

          result
        });

      use self::ActionResult::*;
      match action_result {
        None => {}
        Some(DidNothing) => {}
        Some(DidUpdateFilterer) => {
          // TODO: This is lazy to do a pull from the data source. We
          // don't need to requery; we can filter results already pulled
          // down.
          self.data_source.refresh();
          self.task_results_window.redraw();
        }
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
