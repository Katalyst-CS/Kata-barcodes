use crate::core::generators::{barcodes::proxy::barcode_list, images::proxy::ImageGeneratorProxy};

/// Check if the barcode type is available
/// 
/// __Description:__
/// * `barcode`: Barcode type for check
pub fn check_barcode(barcode: String) -> bool {
    let btypes_vec = barcode_list();
    btypes_vec.contains(&barcode.to_lowercase())
}


/// Check if the is in image valid format
/// 
/// __Description:__
/// * `format`: Format for check
pub fn check_img(format: String ) -> bool{
    let itypes_vec = ImageGeneratorProxy::list_as_string();
    itypes_vec.contains(&format.to_lowercase())
}
