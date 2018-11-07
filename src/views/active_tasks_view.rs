use actions::ForwardAction;
use commands::ActiveTasksViewCommand;
use components::{AttributeFilter, DataSource, Scroller, TaskResultsWindow, UndoBuffer};
use diesel::pg::PgConnection;
use std::rc::Rc;
use util::{get_connection, ui::Window};

pub struct ActiveTasksView {
  pub connection: Rc<PgConnection>,
  pub root_window: Rc<Window>,
  pub task_results_window: Rc<TaskResultsWindow>,
  pub scroller: Rc<Scroller>,
  pub filterer: Rc<AttributeFilter>,
  pub data_source: Rc<DataSource>,
  pub undo_buffer: Rc<UndoBuffer>,
}

impl ActiveTasksView {
  pub fn new(root_window: &Rc<Window>) -> ActiveTasksView {
    // We need our own copy of the root window.
    let root_window = Rc::clone(root_window);

    // Setup connection
    let connection = Rc::new(get_connection());

    // Setup TaskResultsWindow
    let task_results_window = Rc::new(TaskResultsWindow::new(&root_window));

    // Setup Scroller.
    let mut scroller = Scroller::new();
    // TaskResultsWindow listens to Scroller.
    {
      let task_results_window = Rc::clone(&task_results_window);
      scroller.add_callback(Box::new(move |scroller| {
        task_results_window.redraw(scroller);
      }));
    }
    let scroller = Rc::new(scroller);

    // Setup Filterer.
    let mut filterer = AttributeFilter::new();
    // Scroller pulls from Filterer.
    {
      let scroller = Rc::clone(&scroller);
      filterer.add_callback(Box::new(move |filtered_results| {
        scroller.refresh(filtered_results);
      }));
    }
    let filterer = Rc::new(filterer);

    // Setup DataSource
    let mut data_source = DataSource::new();
    // Filterer listens to DataSource.
    {
      let filterer = Rc::clone(&filterer);
      data_source.add_callback(Box::new(move |results| {
        filterer.refresh(results);
      }));
    }
    let data_source = Rc::new(data_source);

    // Setup UndoBuffer
    let undo_buffer = Rc::new(UndoBuffer::new());

    let view = ActiveTasksView {
      connection,
      root_window,
      task_results_window,
      scroller,
      filterer,
      data_source,
      undo_buffer,
    };

    view.data_source.refresh(&view.connection);

    view
  }

  pub fn handle_key(&self, ch: char) {
    let action = ActiveTasksViewCommand::from_key(ch)
      .and_then(|cmd| cmd.to_action(self));

    if let Some(mut action) = action {
      action.execute();

      // TODO: will have to put undoing back in soon.
      // if action.can_be_unexecuted() {
      //   self.undo_buffer.append_action(action);
      // }
    }
  }
}
