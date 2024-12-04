use serde::{Deserialize, Serialize};

pub fn create_str(text: &str) -> String
{
  String::from(text)
}


#[derive(Deserialize, Serialize, Debug)]
pub enum ResponseStatus {
  OK,
  KO,
}