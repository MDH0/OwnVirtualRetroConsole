use std::fs;
use std::string::ToString;

pub struct FileData {
    pub name_length: u8,
    pub name: String,
    pub static_data_length: u8,
    pub static_data: Vec<u8>,
    pub dynamic_data: Vec<u8>,
}

impl FileData {
    pub fn open(path: String) -> Self {
        let content = fs::read(path).unwrap();
        let name_length = content.get(0).unwrap().clone();
        let name = content.get(1..=name_length as usize).unwrap();
        let name = ToString::to_string(name);
        let static_data_length = content.get(2+name_length as usize).unwrap().clone();
        let static_data = content.get(3+name_length as usize..=static_data_length).unwrap().to_vec();
        let dynamic_data = content.get(3+name_length as usize+static_data_length..).unwrap().to_vec();
        FileData {
            name_length,
            name,
            static_data_length,
            static_data,
            dynamic_data,
        }
    }
}