use crate::core::generators::images::proxy::ImageGeneratorProxy;
use crate::domain::outbound::list::ListResponseDto;
use crate::shared::commons::ResponseStatus;
use salvo::prelude::Json;
use salvo::{handler, Request, Response};
#[handler]
pub fn list_img_types(_req: &mut Request, res: &mut Response) {
    let img_types: Vec<String> = ImageGeneratorProxy::list_as_string();
    let response: ListResponseDto = ListResponseDto::new(ResponseStatus::OK, img_types);
    res.render(Json(response))
}
