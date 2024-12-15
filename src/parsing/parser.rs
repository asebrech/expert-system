/*
    =BC
    Cree Knowledge B, C avec fact: true et insert dans la HashMap

    W => B
    Cree un Knowledge B avec en requirement W

    A | B => B
    Cree un Knowledge A avec en requiremement A OR B

    A | B => C | D
    Cree Knowledge C avec requirement A | B + C | D + C
    On check la condition de gauche, puis la condition de droite
    Si elles sont vrai, on check que la query initial soit vrai
    Ensuite, faire la meme avec D, A | B + C | D + D

    A <=> B
    Creer une nouvelle condition
    A => B
    B => A


    A | B <=> A

    A | B <=> A | B

    (A | B) + C => E
    Quand tu rencontres une parenthese (A | B) tu cree une knowledge (A | B)
    ensuite tu extraits A OR B et tu les mets en requirement de la knowledge (A | B)
    Cree la knowledge avec symbol (A | B) et en requirement tu mets A OR B
    Ensuite tu fais juste knowledge E requirement (A | B) AND C

    Y + C => (H + U)

    A | B => (Y + C)
    //
    A | B + (Y + C) + Y => Y
    A | B + (Y + C) + C => C
    ?C

*/

// use crate::data_types::fact::*;
// use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

pub fn parse_line(line: &str, vec: &mut Vec<String>) {
    let mut result = String::new();
    for c in line.chars() {
        if c == '#' {
            break;
        } else if !c.is_whitespace() {
            result.push(c);
        }
    }
    if !result.is_empty() {
        vec.push(result);
    }
}

pub fn check_line(line: &str) {
    let mut chars: Vec<char> = line.chars().collect();

    let mut index = 0;

    while index < chars.len() {
        index += 1;
    }
}

pub fn parse_file(file_path: &str) {
    // let mut data: HashMap<String, Vec<Knowledge>> = HashMap::new();
    match read_file(file_path) {
        Ok(lines) => {
            let mut vec: Vec<String> = Vec::new();
            for line in lines {
                parse_line(&line, &mut vec);
            }
            for a in vec {
                println!("{}", a);
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}
