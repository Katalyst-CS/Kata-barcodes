use crate::infrastructure::http::controllers::json::json_handler;

use super::super::controllers::{img, list, raw};
use salvo::Router;
use salvo::{handler, Response};

pub fn main_router() -> Router {
    Router::new()
        .push(Router::with_path("/raw/<code>.<ext>").get(raw::index))
        .push(Router::with_path("/json/<code>").post(json_handler))
        .push(Router::with_path("/barcodes/list").get(list::list_code_types))
        .push(Router::with_path("/images/formats/list").get(img::list_img_types))
}
