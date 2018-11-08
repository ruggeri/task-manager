use actions::ReversableAction;
use std::cell::{Cell, Ref, RefCell};

type Callback<ActionType, State> = Fn(&State, &ActionType) -> State;

pub struct CallbackPair<ActionType, State> {
  pub redo_callback: Box<Callback<ActionType, State>>,
  pub undo_callback: Box<Callback<ActionType, State>>,
}

pub struct UndoItem<ActionType, State> {
  action: Box<ActionType>,
  state_after_action: State,
}

pub struct UndoBuffer<ActionType, State> {
  initial_state: RefCell<State>,
  items: RefCell<Vec<UndoItem<ActionType, State>>>,
  idx: Cell<Option<usize>>,
  callback_pair: RefCell<Option<CallbackPair<ActionType, State>>>,
}

// TODO: Just need to pass in a method to perform the state update...
impl<ActionType, State> UndoBuffer<ActionType, State>
where
  ActionType: ReversableAction,
{
  pub fn new(initial_state: State) -> UndoBuffer<ActionType, State> {
    UndoBuffer {
      initial_state: RefCell::new(initial_state),
      items: RefCell::new(vec![]),
      idx: Cell::new(None),
      callback_pair: RefCell::new(None),
    }
  }

  pub fn set_callback_pair(&self, callback_pair: CallbackPair<ActionType, State>) {
    *self.callback_pair.borrow_mut() = Some(callback_pair)
  }

  fn undo_callback(&self) -> Ref<Callback<ActionType, State>> {
    Ref::map(self.callback_pair.borrow(), |cbp_option| match cbp_option {
      None => panic!("UndoBuffer callback not set?"),
      Some(ref cbp) => cbp.undo_callback.as_ref(),
    })
  }

  fn redo_callback(&self) -> Ref<Callback<ActionType, State>> {
    Ref::map(self.callback_pair.borrow(), |cbp_option| match cbp_option {
      None => panic!("UndoBuffer callback not set?"),
      Some(ref cbp) => cbp.redo_callback.as_ref(),
    })
  }

  fn action_at_idx(&self, idx: usize) -> Ref<Box<ActionType>> {
    Ref::map(self.items.borrow(), |items| &items[idx].action)
  }

  fn state_at_idx(&self, idx: Option<usize>) -> Ref<State> {
    match idx {
      None => self.initial_state.borrow(),
      Some(idx) => Ref::map(self.items.borrow(), |items| &items[idx].state_after_action),
    }
  }

  pub fn redo(&self) {
    let redo_idx = match self.idx.get() {
      None => 0,
      Some(idx) => idx + 1,
    };

    {
      let mut items = self.items.borrow_mut();
      if redo_idx >= items.len() {
        return;
      }

      let action = &mut items[redo_idx].action;
      action.execute();
    }

    self.idx.set(Some(redo_idx));

    let state_to_save = self.redo_callback()(
      &self.state_at_idx(Some(redo_idx)),
      self.action_at_idx(redo_idx).as_ref(),
    );
    self.set_current_state(state_to_save);
  }

  pub fn undo(&self) {
    let undo_idx = match self.idx.get() {
      None => return,
      Some(idx) => idx,
    };

    {
      let mut items = self.items.borrow_mut();
      let action = &mut items[undo_idx].action;
      action.unexecute();
    }

    self.idx.set(if undo_idx > 0 {
      Some(undo_idx - 1)
    } else {
      None
    });

    let state_to_save = self.undo_callback()(
      &self.state_at_idx(self.idx.get()),
      self.action_at_idx(undo_idx).as_ref(),
    );
    self.set_current_state(state_to_save);
  }

  pub fn set_current_state(&self, state: State) {
    match self.idx.get() {
      None => {
        self.initial_state.replace(state);
      }
      Some(idx) => {
        let mut items = self.items.borrow_mut();
        items[idx].state_after_action = state;
      }
    };
  }

  pub fn append_item(&self, state_after_action: State, action: Box<ActionType>) {
    let item = UndoItem {
      action,
      state_after_action,
    };

    let mut items = self.items.borrow_mut();
    if let Some(idx) = self.idx.get() {
      // Notice I want to keep the first `idx + 1` actions because I
      // want `idx` to remain a valid index.
      items.truncate(idx + 1);
    } else {
      items.truncate(0);
    }

    items.push(item);
    self.idx.set(Some(items.len() - 1));
  }
}
