use actions::ForwardAction;
use commands::ActiveTasksViewCommand;
use components::{
  DataSource, Filterer, TaskResultsWindow, TasksScroller, UndoBuffer,
};
use diesel::pg::PgConnection;
use std::rc::Rc;
use util::{get_db_connection, UserInterface};

pub struct ActiveTasksView {
  pub connection: Rc<PgConnection>,
  pub ui: Rc<UserInterface>,
  pub task_results_window: Rc<TaskResultsWindow>,
  pub scroller: Rc<TasksScroller>,
  pub filterer: Rc<Filterer>,
  pub data_source: Rc<DataSource>,
  pub undo_buffer: Rc<UndoBuffer>,
}

impl ActiveTasksView {
  pub fn new(ui: &Rc<UserInterface>) -> Rc<ActiveTasksView> {
    // We need our own copy of the root window.
    let ui = Rc::clone(ui);

    // Setup connection
    let connection = Rc::new(get_db_connection());

    // Setup TaskResultsWindow
    let task_results_window = Rc::new(TaskResultsWindow::new(&ui));

    // Setup Scroller.
    let mut scroller = TasksScroller::new();
    // TaskResultsWindow listens to Scroller.
    {
      let task_results_window = Rc::clone(&task_results_window);
      scroller.add_callback(Box::new(move |event| {
        task_results_window.redraw(event);
      }));
    }
    let scroller = Rc::new(scroller);

    // Setup Filterer.
    let mut filterer = Filterer::new();
    // Scroller pulls from Filterer.
    {
      let scroller = Rc::clone(&scroller);
      filterer.add_callback(Box::new(
        move |filtered_results, _event| {
          scroller.refresh(filtered_results);
        },
      ));
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
      ui,
      task_results_window,
      scroller,
      filterer,
      data_source,
      undo_buffer,
    };
    let view = Rc::new(view);

    view.data_source.pull(&view.connection);

    view
  }

  pub fn handle_key(view: &Rc<Self>, ch: char) {
    let did_execute_action = ActiveTasksViewCommand::from_key(ch)
      .and_then(|cmd| cmd.to_action(view))
      .map(|mut action| {
        action.execute();
        action.maybe_add_to_undo_buffer(&view.undo_buffer);
      }).is_some();

    if !did_execute_action {
      // Redraw screen regardless.
      view.task_results_window.full_redraw();
    }
  }
}
