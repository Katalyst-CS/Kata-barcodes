use serde::{Deserialize, Serialize};
use crate::shared::commons::{create_str, ResponseStatus};


#[derive(Deserialize, Serialize, Debug)]

pub struct ListResponseDto {
    pub status: Option<ResponseStatus>,
    pub list: Vec<String>
}


impl ListResponseDto {
    pub fn new(status: ResponseStatus, list: Vec<&str>) -> Self
    {
        let vector: Vec<String> = ListResponseDto::map_vector(list);
        ListResponseDto {
            status: Some(status),
            list: vector
        }
    }

    fn map_arr(list: &[&str]) -> Vec<String> 
    {
        let mut vector: Vec<String> = Vec::new();
        for element in list {
            vector.push( create_str(&element) );
        };
        vector
    }

    fn map_vector(list: Vec<&str>) -> Vec<String>
    {
        let mut vector: Vec<String> = Vec::new();
        list.iter().map(|el| {vector.push(el.to_string()); el});
        vector
    }

    pub fn add_element(&mut self, element: &str) 
    {
        self.list.push(create_str(&element));
    }

    pub fn add_vect(&mut self, vector: Vec<&str>)
    {
        self.list = ListResponseDto::map_vector(vector);
    }

    pub fn add_list(&mut self, list: &[&str]) {
        let vector = ListResponseDto::map_arr(list);
        self.list = vector;
    }
}

