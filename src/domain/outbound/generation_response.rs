use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct GenerationResponseDto {
  barcode: Option<String>,
  sign: Option<String>,
}