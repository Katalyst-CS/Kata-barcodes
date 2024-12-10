use crate::{core::generators::barcodes::proxy::BarcodeGeneratorProxy, domain::outbound::list::ListResponseDto, shared::commons::ResponseStatus};



pub async fn list_code_tipes() {
    let bartypes = BarcodeGeneratorProxy::list();
    let response: ListResponseDto = ListResponseDto::new(ResponseStatus::OK, bartypes);
}
