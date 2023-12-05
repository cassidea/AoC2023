use std::fs;

pub fn get_lines_from_file(file_name : &str) -> Vec<String> {
    let content = fs::read_to_string(file_name).expect("File not found!");
    let mut result = vec![];
    content.lines().filter(|l| !l.is_empty()).for_each(|line| result.push(String::from(line)));
    result
}

pub fn get_lines_as_string(file_name : &str) -> String {
    fs::read_to_string(file_name).expect("File not found!")
}
