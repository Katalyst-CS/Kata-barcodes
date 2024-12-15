use crate::{core::generators::barcodes::proxy::{BarcodeGeneratorProxy, barcode_list}, domain::outbound::list::ListResponseDto, shared::commons::ResponseStatus};
use salvo::{Request, handler, Response};
use salvo::prelude::Json;

#[handler]
pub async fn list_code_tipes(_req: &mut Request, res: &mut Response) {
    let bartypes: Vec<&str> = vec!["ean8"];
    let response = ListResponseDto::new(ResponseStatus::OK, &bartypes);
    println!("Response: {:?}", response);
    res.render(Json(response))
}
