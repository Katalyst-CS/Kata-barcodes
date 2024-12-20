use crate::shared::commons::{create_str, ResponseStatus};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListResponseDto {
    pub status: Option<ResponseStatus>,
    pub list: Vec<String>, // Cambié a Vec<String> en lugar de &'static mut Vec<String>
}

impl ListResponseDto {
    pub fn new(status: ResponseStatus, list: Vec<String>) -> Self {
        ListResponseDto {
            status: Some(status),
            list: list,
        }
    }

    fn map_vector(list: &[&str]) -> Vec<String> {
        list.iter().map(|&el| el.to_string()).collect()
    }

    pub fn add_element(&mut self, element: &str) {
        self.list.push(element.to_string());
    }

    pub fn add_vect(&mut self, vector: &Vec<&str>) {
        self.list = ListResponseDto::map_vector(vector);
    }

    pub fn add_list(&mut self, list: &[&str]) {
        self.list = ListResponseDto::map_vector(list);
    }
}
