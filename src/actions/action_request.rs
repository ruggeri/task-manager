#[derive(Clone, Copy, Debug)]
pub enum ActionRequest {
  RequestFiltererUpdate,
  RequestScrollerUpdate,
  RequestDataSourceUpdate,
  RequestShutDown,
}
