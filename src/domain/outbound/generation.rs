use serde::{Deserialize, Serialize};

use crate::shared::commons::create_str;


#[derive(Debug, Deserialize, Serialize)]
pub struct GenerationResponseDto {
  barcode: Option<String>,
  sign: Option<String>,
}


impl GenerationResponseDto{
  pub fn new(barcode_base64: &str, sign: &str) -> Self {
    GenerationResponseDto {
      barcode: Some(create_str(&barcode_base64)),
      sign: Some(create_str(&sign))
    }
  }
}