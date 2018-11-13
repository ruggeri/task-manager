use super::base_scroller::ResultsVec;

#[derive(Clone, Debug)]
pub struct ScrollerState<ResultType: Clone> {
  pub current_result_idx: i32,
  pub results: ResultsVec<ResultType>,
}
