use serde::{Deserialize, Serialize};

use crate::shared::commons::{create_str, ResponseStatus};


#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponseDto {
  pub message: Option<String>,
  pub code: Option<u32>,
  pub status: Option<ResponseStatus>
}


impl ErrorResponseDto {
  pub fn new(message: &str, code: u32, status: ResponseStatus) -> Self
  {
    ErrorResponseDto {
      message: Some(create_str(&message)),
      code: Some(code),
      status: Some(status)
    }
  }
}