use salvo::Router;
use super::super::controllers::raw;

pub fn main_router() -> Router
{
  Router::new()
  .path("/raw/<code>.<ext>").get(raw::index)
}