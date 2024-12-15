use salvo::Router;
use salvo::{Response, handler};
use super::super::controllers::{raw, list};

#[handler]
fn saludo(res: &mut Response){
    res.render("Hola");
}

pub fn main_router() -> Router
{
  Router::new()
    .push(
        Router::with_path("/raw/<code>.<ext>").get(raw::index))
    .push(
        Router::with_path("/hello").get(saludo))
    .push(
        Router::with_path("/barcodes/list").get(list::list_code_tipes))
}
