
use salvo::{cors::Cors, http::Method};

pub fn generate_cors()
{
  Cors::new()
  .allow_origin("*")
  .allow_methods(vec![Method::GET, Method::PATCH, Method::POST, Method::DELETE, Method::OPTIONS, Method::PUT])
  .into_handler();
}