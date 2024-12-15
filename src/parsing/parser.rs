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

use crate::{Condition, Requirement};

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

pub fn pasre(lines: Vec<String>) {
    let mut vec: Vec<String> = Vec::new();
    for line in lines {
        parse_line(&line, &mut vec);
    }
    for a in vec {
        println!("{}", a);
        check_line(&a);
    }
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

pub fn get_condition(operator: char) -> Condition {
    match operator {
        '|' => Condition::OR,
        '^' => Condition::XOR,
        '+' => Condition::AND,
        _ => Condition::END,
    }
}

pub fn check_line(line: &str) {
    let chars: Vec<char> = line.chars().collect();

    let mut requirements: Vec<Requirement> = Vec::new();
    let operators: Vec<char> = vec!['|', '^', '+'];
    let mut check_operator = false;
    let len = chars.len();
    let mut index = 0;
    while index < len {
        let mut not = false;
        let mut symbol = String::new();
        let mut operator = ' ';
        if chars[index] != '!'
            && !chars[index].is_alphabetic()
            && !operators.contains(&chars[index])
        {
            let end: String = chars[index..len].iter().collect();
            println!("{} is the end rule", end);
            break;
        }
        if chars[index] == '!' {
            not = true;
            index += 1;
        }
        if index < len && !check_operator && chars[index].is_alphabetic() {
            symbol = chars[index].to_string();
            print!("symbole :{} ", chars[index]);
            index += 1;
            check_operator = !check_operator;
        }
        if index < len && check_operator && operators.contains(&chars[index]) {
            operator = chars[index];
            print!("operator :{} ", chars[index]);
            check_operator = !check_operator;
            index += 1;
        }
        if symbol.is_empty() {
            println!("Error");
            break;
        }
        let requirement = Requirement::new(symbol, get_condition(operator), not);
        requirements.push(requirement);
    }
}

pub fn parse_file(file_path: &str) {
    // let mut data: HashMap<String, Vec<Knowledge>> = HashMap::new();
    match read_file(file_path) {
        Ok(lines) => {
            pasre(lines);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}
