pub fn test() {
    println!("test")
}

//
// # are comments
// => result
// = Initial fact
// ? Queries
//




use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

pub fn parse_file(file_path: &str) {
    match read_file(file_path) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}
