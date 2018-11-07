use actions::{ActiveTasksViewAction, ForwardAction};
use commands::ActiveTasksViewCommand;
use components::{
  DataSource,
  DataSourceState,
  Filterer,
  FiltererState,
  Scroller,
  ScrollerState,
  TaskResultsWindow,
  UndoBuffer
};
use diesel::pg::PgConnection;
use std::rc::Rc;
use util::{get_connection, ui::Window};

pub struct ActiveTasksView {
  pub connection: Rc<PgConnection>,
  pub root_window: Rc<Window>,
  pub task_results_window: Rc<TaskResultsWindow>,
  pub scroller: Rc<Scroller>,
  pub filterer: Rc<Filterer>,
  pub data_source: Rc<DataSource>,
  pub undo_buffer: Rc<UndoBuffer<ActiveTasksViewState>>,
}

#[derive(Clone, Debug)]
pub struct ActiveTasksViewState {
  pub data_source_state: DataSourceState,
  pub filterer_state: FiltererState,
  pub scroller_state: ScrollerState,
}

impl ActiveTasksView {
  pub fn new(root_window: &Rc<Window>) -> Rc<ActiveTasksView> {
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
    let mut filterer = Filterer::new();
    // Scroller pulls from Filterer.
    {
      let scroller = Rc::clone(&scroller);
      filterer.add_callback(Box::new(move |filtered_results, event| {
        use components::FiltererEvent::*;
        use components::ScrollerRefreshType::*;

        let refresh_type = match event {
          FiltererCriteriaUpdated => MajorRefresh,
          FiltererGotUpdatedResults => MinorRefresh,
        };

        scroller.refresh(filtered_results, refresh_type);
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
    let undo_buffer = Rc::new(UndoBuffer::new(ActiveTasksViewState {
      data_source_state: data_source.state(),
      filterer_state: filterer.state(),
      scroller_state: scroller.state(),
    }));

    let view = ActiveTasksView {
      connection,
      root_window,
      task_results_window,
      scroller,
      filterer,
      data_source,
      undo_buffer,
    };
    let view = Rc::new(view);

    {
      let weak_view = Rc::downgrade(&view);
      view.undo_buffer.set_callback(Box::new(move |state| {
        let view = weak_view
          .upgrade()
          .expect("How did undo buffer callback outlive view?");
        view.restore_state(state.clone());
      }));
    }

    view.data_source.pull(&view.connection);

    view
  }

  pub fn handle_key(&self, ch: char) {
    let action = ActiveTasksViewCommand::from_key(ch)
      .and_then(|cmd| cmd.to_action(self));

    if let Some(action) = action {
      self.handle_action(action)
    } else {
      // Redraw screen regardless.
      self.task_results_window.redraw(&self.scroller);
    }
  }

  pub fn handle_action(&self, mut action: ActiveTasksViewAction) {
    action.execute();

    use self::ActiveTasksViewAction::*;
    match action {
      Filterer { fa } => {
        self.data_source.pull(&self.connection);
        self.undo_buffer.append_item(self.state(), Box::new(fa));
      }
      Scroll { .. } => {
        self.scroller.push();
        self.undo_buffer.set_current_state(self.state());
      }
      Task { ta } => {
        self.data_source.pull(&self.connection);
        self.undo_buffer.append_item(self.state(), Box::new(ta));
      },
      UndoBuffer { .. } => {
        // On any undo buffer action, after restoring the states, tell
        // the data source to do a nice clean refresh.
        self.data_source.pull(&self.connection);
        self.undo_buffer.set_current_state(self.state());
      }
    }
  }

  pub fn state(&self) -> ActiveTasksViewState {
    ActiveTasksViewState {
      data_source_state: self.data_source.state().clone(),
      filterer_state: self.filterer.state(),
      scroller_state: self.scroller.state().clone(),
    }
  }

  pub fn restore_state(&self, state: ActiveTasksViewState) {
    self.data_source.restore_state(state.data_source_state);
    self.filterer.restore_state(state.filterer_state);
    self.scroller.restore_state(state.scroller_state);
  }
}
