use super::ScrollerState;

#[derive(Clone)]
pub enum ScrollerEvent<ResultType: Clone> {
  ChangedScrollPosition {
    old_result_idx: i32,
    new_state: ScrollerState<ResultType>,
  },

  GotNewScrollResults {
    state: ScrollerState<ResultType>,
  },
}
