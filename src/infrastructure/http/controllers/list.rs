use crate::{
    core::generators::barcodes::proxy::{barcode_list, BarcodeGeneratorProxy},
    domain::outbound::list::ListResponseDto,
    shared::commons::ResponseStatus,
};
use salvo::prelude::Json;
use salvo::{handler, Request, Response};

#[handler]
pub async fn list_code_types(_req: &mut Request, res: &mut Response) {
    let bartypes: Vec<String> = barcode_list();
    let response = ListResponseDto::new(ResponseStatus::OK, bartypes);
    println!("Response: {:?}", response);
    res.render(Json(response))
}
