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

use std::collections::HashMap;
// use crate::data_types::fact::*;
// use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::panic;
use std::path::Path;

use crate::{data_types, Condition, Knowledge, Requirement};

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

pub fn pasre(
    lines: Vec<String>,
) -> std::collections::HashMap<std::string::String, std::vec::Vec<data_types::fact::Knowledge>> {
    let mut vec: Vec<String> = Vec::new();
    for line in lines {
        parse_line(&line, &mut vec);
    }
    let mut data: HashMap<String, Vec<Knowledge>> = HashMap::new();
    for a in vec {
        println!("{}", a);
        check_line(&a, &mut data);
    }
    data
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

pub fn get_symbol(char: char) -> String {
    if !char.is_alphabetic() {
        panic!("Invalid symbol: {}", char);
    }
    char.to_string().to_uppercase()
}

pub fn get_condition(operator: char) -> Condition {
    match operator {
        '|' => Condition::OR,
        '^' => Condition::XOR,
        '+' => Condition::AND,
        '<' => Condition::END,
        '=' => Condition::END,
        _ => panic!("Invalid operator: {}", operator),
    }
}

pub fn get_requirement(
    chars: &[char],
    mut index: usize,
) -> (std::vec::Vec<data_types::fact::Requirement>, usize) {
    let mut requirements: Vec<Requirement> = Vec::new();
    let operators: Vec<char> = vec!['|', '^', '+'];
    let len = chars.len();
    while index < len
        && (chars[index] == '!'
            || chars[index].is_alphabetic()
            || operators.contains(&chars[index]))
    {
        let mut not = false;
        if chars[index] == '!' {
            not = true;
            index += 1;
        }
        let operator = if index + 1 < len {
            chars[index + 1]
        } else {
            '='
        };
        let requirement = Requirement::new(get_symbol(chars[index]), get_condition(operator), not);
        // println!("{:?}", requirement);
        requirements.push(requirement);
        index += 2;
    }
    (requirements, index)
}

pub fn add_to_data(
    symbole: String,
    knowledge: Knowledge,
    data: &mut HashMap<String, Vec<Knowledge>>,
) {
    data.entry(symbole)
        .and_modify(|v| v.push(knowledge.clone()))
        .or_insert_with(|| vec![knowledge]);
}

pub fn check_line(line: &str, data: &mut HashMap<String, Vec<Knowledge>>) {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut index = 0;
    if len > 1 && chars[0] == '=' && chars[1].is_alphabetic() {
        println!("Fact");
        return;
    }
    if len > 1 && chars[0] == '?' && chars[1].is_alphabetic() {
        println!("Search");
        return;
    }
    let (mut requirements, mut index) = get_requirement(&chars, index);
    if index < len + 1 && chars[index - 1] == '=' && chars[index] == '>' {
        let (results, _) = get_requirement(&chars, index + 1);
        if results.is_empty() {
            panic!("Invalid line: {}", line);
        }
        if results[0].condition == Condition::END {
            let knowledge = Knowledge::new(
                results[0].symbol.clone(),
                false,
                requirements,
                results[0].not,
            );
            println!("{:?}", knowledge);
            add_to_data(chars[index + 1].to_string(), knowledge, data);
        } else {
            let results_clone = results.clone();
            for result in results_clone {
                let mut all_requirements = requirements.clone();
                all_requirements.last_mut().unwrap().condition = Condition::AND;
                let mut results_clone = results.clone();
                results_clone.last_mut().unwrap().condition = Condition::AND;
                let requirement =
                    Requirement::new(result.symbol.clone(), Condition::END, result.not);
                results_clone.push(requirement);
                all_requirements.extend(results_clone);
                let knowledge = Knowledge::new(
                    result.symbol.clone(),
                    false,
                    all_requirements.clone(),
                    result.not,
                );
                println!("{:?}", knowledge);
                add_to_data(chars[index + 1].to_string(), knowledge, data);
            }
        }
        return;
    }
    if index + 1 < len && chars[index - 1] == '<' && chars[index] == '=' && chars[index + 1] == '>'
    {
        println!("Double");
        return;
    }
    panic!("Invalid line3: {}", line);
}

pub fn parse_file(file_path: &str) {
    match read_file(file_path) {
        Ok(lines) => {
            let result = panic::catch_unwind(|| {
                pasre(lines);
            });

            if let Err(err) = result {
                println!("Caught a panic: {:?}", err);
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}
