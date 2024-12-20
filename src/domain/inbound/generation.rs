use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerationRequestDto {
    pub data: Option<String>,
    pub sign: Option<String>,
    pub barcode: Option<String>, // type of barcode
    pub image: Option<String>,   // type of image
}
