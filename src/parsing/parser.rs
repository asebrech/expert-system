use log::debug;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::panic;
use std::path::Path;

use crate::{data_types, Condition, Knowledge, Requirement};

pub fn check_search(chars: &[char], len: usize, start_index: usize) -> Vec<char> {
    let mut result = Vec::new();
    let mut i = start_index;

    while i < len {
        let c = chars[i];
        if c == '(' {
            let mut open_parens = 1;
            let mut j = i + 1;
            while j < len && open_parens > 0 {
                if chars[j] == '(' {
                    open_parens += 1;
                } else if chars[j] == ')' {
                    open_parens -= 1;
                }
                j += 1;
            }
            if open_parens != 0 {
                panic!("Unmatched parentheses in search line: {:?}", chars);
            }
            result.extend(&chars[i..j]);
            if j < len {
                result.push('+');
            }
            i = j;
        } else {
            result.push(c);
            if i < len - 1 {
                result.push('+');
            }
        }
        i += 1;
    }
    result
}

pub fn chars_without_parentheses(chars: &[char], start_index: usize) -> Vec<char> {
    let mut new_chars = Vec::new();
    for &c in &chars[start_index..] {
        if c != '(' && c != ')' {
            new_chars.push(c);
        }
    }
    new_chars
}

pub fn get_symbol(char: char) -> String {
    if !char.is_alphabetic() {
        panic!("Invalid symbol: {}", char);
    }
    char.to_string().to_uppercase()
}

pub fn get_operator(chars: &[char], index: usize) -> char {
    if index < chars.len() {
        chars[index]
    } else {
        '='
    }
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

pub fn parentheses_content(chars: &[char], start_index: usize) -> (std::string::String, usize) {
    let mut result = String::new();
    let mut open_parens = 0;
    let mut index = start_index;

    while index < chars.len() {
        let c = chars[index];
        if c == '(' {
            open_parens += 1;
        } else if c == ')' {
            open_parens -= 1;
        }
        result.push(c);
        index += 1;
        if open_parens == 0 {
            break;
        }
    }

    if open_parens != 0 {
        panic!("Unmatched parentheses");
    }

    (result, index)
}

pub fn create_knowledge(
    chars: &[char],
    index: usize,
    requirements: Vec<Requirement>,
    data: &mut HashMap<String, Vec<Knowledge>>,
) {
    // Get the requirements from the characters starting after the implication operator
    // This parses the right-hand side of the implication to determine the resulting conditions.
    let (results, _) = get_requirements(chars, index + 1, data);
    if results.is_empty() {
        panic!("Line missing result");
    }
    // Remove parentheses from the characters and get the requirements again to handle nested conditions.
    let chars_without = chars_without_parentheses(chars, index + 1);
    let (results_without, _) = get_requirements(&chars_without, 0, data);

    // If the first result has an END condition, create a new Knowledge instance and add it to the data
    if results_without[0].condition == Condition::END {
        let knowledge = Knowledge::new(
            results_without[0].symbol.clone(),
            false,
            requirements,
            results_without[0].not,
        );
        add_to_data(results[0].symbol.clone(), knowledge, data);
    } else {
        // Otherwise, iterate through the results and create Knowledge instances for each
        for result_without in results_without {
            // Clone the initial requirements and set the last condition to AND to ensure logical connection.
            let mut all_requirements = requirements.clone();
            all_requirements.last_mut().unwrap().condition = Condition::AND;
            // Clone the results and set the last condition to AND to maintain consistency.
            let mut results_clone = results.clone();
            results_clone.last_mut().unwrap().condition = Condition::AND;
            // Create a new requirement with the current result and END condition to mark it as final.
            let requirement = Requirement::new(
                result_without.symbol.clone(),
                Condition::END,
                result_without.not,
            );
            // Add the new requirement to the cloned results and extend the initial requirements with them.
            results_clone.push(requirement);
            all_requirements.extend(results_clone);
            // Create a new Knowledge instance with the combined requirements to represent the logical relationship.
            let knowledge = Knowledge::new(
                result_without.symbol.clone(),
                false,
                all_requirements.clone(),
                result_without.not,
            );
            add_to_data(result_without.symbol.clone(), knowledge, data);
        }
    }
}

pub fn get_requirements(
    chars: &[char],
    mut index: usize,
    data: &mut HashMap<String, Vec<Knowledge>>,
) -> (std::vec::Vec<data_types::fact::Requirement>, usize) {
    let mut requirements: Vec<Requirement> = Vec::new();
    let syntax: Vec<char> = vec!['!', '('];
    let len = chars.len();
    while index < len && (chars[index].is_alphabetic() || syntax.contains(&chars[index])) {
        let mut not = false;
        if chars[index] == '!' {
            not = true;
            index += 1;
        }
        if chars[index] == '(' {
            // Extract the content within the parentheses and the index after the closing parenthesis
            let (content, content_index) = parentheses_content(chars, index);
            // Trim the parentheses from the content
            let trim_result = content[1..content.len() - 1].to_string();
            let line: Vec<char> = trim_result.chars().collect();
            // Recursively get the requirements within the parentheses
            let (requirements_parentheses, _) = get_requirements(&line, 0, data);
            // Create a new Knowledge instance with the content and its requirements
            let knowledge =
                Knowledge::new(content.to_string(), false, requirements_parentheses, false);
            // Add the new Knowledge instance to the data
            add_to_data(content.to_string(), knowledge, data);
            let operator = get_operator(chars, content_index);
            let requirement = Requirement::new(content.to_string(), get_condition(operator), not);
            requirements.push(requirement);
            index = content_index + 1;
        } else {
            let operator = get_operator(chars, index + 1);
            let requirement =
                Requirement::new(get_symbol(chars[index]), get_condition(operator), not);
            requirements.push(requirement);
            index += 2;
        }
    }
    (requirements, index)
}

pub fn add_to_data(
    symbole: String,
    knowledge: Knowledge,
    data: &mut HashMap<String, Vec<Knowledge>>,
) {
    debug!("{:?}", knowledge);
    data.entry(symbole)
        .and_modify(|v| v.push(knowledge.clone()))
        .or_insert_with(|| vec![knowledge]);
}

pub fn check_line(
    line: &str,
    data: &mut HashMap<String, Vec<Knowledge>>,
    search: &mut Vec<Requirement>,
) {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut index = 0;

    // Check if the line defines initial facts (e.g., "=AB")
    if len > 1 && chars[0] == '=' && chars[1].is_alphabetic() {
        index += 1;
        while index < len && chars[1].is_alphabetic() {
            let knowledge = Knowledge::new(chars[index].to_string(), true, Vec::new(), false);
            add_to_data(chars[index].to_string(), knowledge, data);
            index += 1;
        }
        if index < len {
            panic!("Invalid fact line: {}", line);
        }
        return;
    }
    // Check if the line defines a query (e.g., "?GVX")
    if len > 1 && chars[0] == '?' && chars[1].is_alphabetic() {
        let res = check_search(&chars, len, index + 1);
        debug!("{:?}", res);
        let (results, _) = get_requirements(&res, 0, data);
        search.extend(results);
        return;
    }
    // Parse the requirements from the line
    let (requirements, index) = get_requirements(&chars, index, data);
    // Check if the line defines an implication (e.g., "A + B => C")
    if len > index && chars[index - 1] == '=' && chars[index] == '>' {
        create_knowledge(&chars, index, requirements, data);
        return;
    }
    // Check if the line defines a biconditional (e.g., "A + B <=> C")
    if len > index + 1 && chars[index - 1] == '<' && chars[index] == '=' && chars[index + 1] == '>'
    {
        // Create knowledge for the biconditional statement
        create_knowledge(&chars, index + 1, requirements, data);
        // Split the characters into two parts: before and after the biconditional operator
        let before = &chars[..index - 1];
        let after = &chars[index + 2..];
        // Construct a new string representing the reversed implication
        let mut new_string = String::new();
        new_string.push_str(&after.iter().collect::<String>());
        new_string.push_str("=>");
        new_string.push_str(&before.iter().collect::<String>());
        // Recursively check the new string to handle the reversed implication
        check_line(new_string.as_str(), data, search);
        return;
    }
    // If the line does not match any valid format, panic
    panic!("Invalid line: {}", line);
}

pub fn clean_line(line: &str, vec: &mut Vec<String>) {
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

pub fn parse_lines(
    lines: Vec<String>,
) -> (
    std::collections::HashMap<std::string::String, std::vec::Vec<data_types::fact::Knowledge>>,
    Vec<Requirement>,
) {
    let mut vec: Vec<String> = Vec::new();
    for line in lines {
        clean_line(&line, &mut vec);
    }
    let mut data: HashMap<String, Vec<Knowledge>> = HashMap::new();
    let mut search: Vec<Requirement> = Vec::new();
    for a in vec {
        debug!("Line : {}", a);
        check_line(&a, &mut data, &mut search);
    }
    (data, search)
}

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

type KnowledgeMap = std::collections::HashMap<String, Vec<data_types::fact::Knowledge>>;
type RequirementVec = Vec<Requirement>;

pub fn parse_file(file_path: &str) -> Option<(KnowledgeMap, RequirementVec)> {
    match read_file(file_path) {
        Ok(lines) => {
            let result = panic::catch_unwind(|| parse_lines(lines));

            if let Ok(parsed_data) = result {
                Some(parsed_data)
            } else {
                println!("Caught a panic: {:?}", result.err());
                None
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            None
        }
    }
}
