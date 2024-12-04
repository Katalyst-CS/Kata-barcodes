use serde::{Deserialize, Serialize};
use crate::shared::commons::{create_str, ResponseStatus};


#[derive(Deserialize, Serialize, Debug)]

pub struct ListResponseDto {
    pub status: Option<ResponseStatus>,
    pub list: Vec<String>
}


impl ListResponseDto {
    pub fn new(status: ResponseStatus, list: &[&str]) -> Self
    {
        let vector: Vec<String> = ListResponseDto::map_arr(list);
        ListResponseDto {
            status: Some(status),
            list: vector
        }
    }

    fn map_arr(list: &[&str]) -> Vec<String> 
    {
        let mut vector: Vec<String> = vec![];
        for element in list {
            vector.push( create_str(&element) );
        };
        vector
    }

    pub fn add_element(&mut self, element: &str) 
    {
        self.list.push(create_str(&element));
    }

    pub fn add_list(&mut self, list: &[&str]) {
        let vector = ListResponseDto::map_arr(list);
        self.list = vector;
    }
}

