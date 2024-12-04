use std::fs;

pub fn parse_input_to_string(file_path: &str) -> Result<String, std::io::Error> { 
    let file = fs::read_to_string(file_path).expect("Unable to read file"); 
    Ok(file)
}