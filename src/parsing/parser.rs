use log::debug;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::{Condition, Knowledge, Requirement};

pub fn chars_without_parentheses(chars: &[char], start_index: usize) -> Vec<char> {
    let mut new_chars = Vec::new();
    for &c in &chars[start_index..] {
        if c != '(' && c != ')' {
            new_chars.push(c);
        }
    }
    new_chars
}

pub fn get_symbol(char: char) -> Result<String, String> {
    if !char.is_alphabetic() {
        return Err(format!("Invalid symbol: {}", char));
    }
    Ok(char.to_string().to_uppercase())
}

pub fn get_operator(chars: &[char], index: usize) -> char {
    if index < chars.len() {
        chars[index]
    } else {
        '='
    }
}

pub fn get_condition(operator: char) -> Result<Condition, String> {
    match operator {
        '|' => Ok(Condition::OR),
        '^' => Ok(Condition::XOR),
        '+' => Ok(Condition::AND),
        '<' => Ok(Condition::END),
        '=' => Ok(Condition::END),
        _ => Err(format!("Invalid operator: {}", operator)),
    }
}

pub fn parentheses_content(
    chars: &[char],
    start_index: usize,
    is_bracket: bool,
) -> Result<(String, usize), String> {
    let mut result = String::new();
    let mut open_parens = 0;
    let mut index = start_index;
    let mut open = '(';
    let mut close = ')';
    if is_bracket {
        open = '[';
        close = ']';
    }

    while index < chars.len() {
        let c = chars[index];
        if c == open {
            open_parens += 1;
        } else if c == close {
            open_parens -= 1;
        }
        result.push(c);
        index += 1;
        if open_parens == 0 {
            break;
        }
    }

    if open_parens != 0 {
        return Err("Unmatched parentheses".to_string());
    }

    Ok((result, index))
}

pub fn priority_content(s: &str) -> String {
    let mut index = 0;
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    while index < chars.len() {
        if index + 1 < chars.len() && chars[index + 1] == '|' && chars[index].is_alphabetic() {
            if index > 0 && chars[index - 1] == '!' {
                result.pop();
                result.push('[');
                result.push('!');
            } else {
                result.push('[');
            }
            result.push(chars[index]);
            index += 1;
            while index < chars.len()
                && (chars[index].is_alphabetic()
                    || chars[index] == '|'
                    || chars[index] == '+'
                    || chars[index] == '!')
            {
                if index + 1 < chars.len()
                    && chars[index + 1] == '+'
                    && chars[index].is_alphabetic()
                {
                    if index > 0 && chars[index - 1] == '!' {
                        result.pop();
                        result.push('[');
                        result.push('!');
                    } else {
                        result.push('[');
                    }
                    result.push(chars[index]);
                    index += 1;
                    while index < chars.len()
                        && (chars[index].is_alphabetic()
                            || chars[index] == '+'
                            || chars[index] == '!')
                    {
                        result.push(chars[index]);
                        index += 1;
                    }
                    result.push(']');
                    continue;
                }

                result.push(chars[index]);
                index += 1;
            }
            result.push(']');
            continue;
        }

        if index + 1 < chars.len() && chars[index + 1] == '+' && chars[index].is_alphabetic() {
            if index > 0 && chars[index - 1] == '!' {
                result.pop();
                result.push('[');
                result.push('!');
            } else {
                result.push('[');
            }
            result.push(chars[index]);
            index += 1;
            while index < chars.len()
                && (chars[index].is_alphabetic() || chars[index] == '+' || chars[index] == '!')
            {
                result.push(chars[index]);
                index += 1;
            }
            result.push(']');
            continue;
        }
        result.push(chars[index]);
        index += 1;
    }
    result
}

pub fn create_knowledge(
    chars: &[char],
    index: usize,
    requirements: Vec<Requirement>,
    data: &mut HashMap<String, Vec<Knowledge>>,
    line: &str,
) -> Result<(), String> {
    let (results, _) = get_requirements(chars, index + 1, data)?;
    if results.is_empty() {
        return Err("Line missing result".to_string());
    }
    let chars_without = chars_without_parentheses(chars, index + 1);
    let (results_without, _) = get_requirements(&chars_without, 0, data)?;

    for result_without in results_without {
        let result_requirement = if results.len() > 1 {
            Some(results.clone())
        } else {
            None
        };
        let knowledge = Knowledge::new(
            result_without.symbol.clone(),
            false,
            Some(line.to_string()),
            requirements.clone(),
            result_requirement,
            result_without.not,
        );
		println!("Knowledge {} for cache line : {}", result_without.symbol.clone(), line);
        add_to_data(result_without.symbol.clone(), knowledge, data);
    }

    Ok(())
}

pub fn get_requirements(
    chars: &[char],
    mut index: usize,
    data: &mut HashMap<String, Vec<Knowledge>>,
) -> Result<(Vec<Requirement>, usize), String> {
    let mut requirements: Vec<Requirement> = Vec::new();
    let syntax: Vec<char> = vec!['!', '(', '['];
    let len = chars.len();
    let line_from_chars: String = chars.iter().collect();

    while index < len && (chars[index].is_alphabetic() || syntax.contains(&chars[index])) {
        let mut not = false;
        if chars[index] == '!' {
            not = true;
            index += 1;
        }
        if chars[index] == '(' || chars[index] == '[' {
            let is_bracket = chars[index] == '[';
            let (content, content_index) = parentheses_content(chars, index, is_bracket)?;
            let trim_result = content[1..content.len() - 1].to_string();
            let line: Vec<char> = trim_result.chars().collect();
            let (requirements_parentheses, _) = get_requirements(&line, 0, data)?;
            let knowledge = Knowledge::new(
                content.to_string(),
                false,
                None,
                requirements_parentheses,
                None,
                false,
            );
            add_to_data(content.to_string(), knowledge, data);
            let operator = get_operator(chars, content_index);
            let requirement = Requirement::new(content.to_string(), get_condition(operator)?, not);
            requirements.push(requirement);
            index = content_index + 1;
        } else {
            let operator = get_operator(chars, index + 1);
            let requirement =
                Requirement::new(get_symbol(chars[index])?, get_condition(operator)?, not);
            requirements.push(requirement);
            index += 2;
        }
    }
    Ok((requirements, index))
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
    search: &mut Vec<char>,
) -> Result<(), String> {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut index = 0;

    if chars[0] == '=' {
        index += 1;
        while index < len && chars[index].is_alphabetic() {
            let knowledge = Knowledge::new(
                chars[index].to_string(),
                true,
                None,
                Vec::new(),
                None,
                false,
            );
            add_to_data(chars[index].to_string(), knowledge, data);
            index += 1;
        }
        return Ok(());
    }

    if len > 1 && chars[0] == '?' && chars[1].is_alphabetic() {
        let query_chars: Vec<char> = chars[1..].to_vec();
        if query_chars.iter().all(|&c| c.is_alphabetic()) {
            debug!("{:?}", query_chars);
            search.extend(query_chars);
        } else {
            return Err(format!("Invalid query line: {}", line));
        }
        return Ok(());
    }

    let (requirements, index) = get_requirements(&chars, index, data)?;
    if len > index && index > 0 && chars[index - 1] == '=' && chars[index] == '>' {
        create_knowledge(&chars, index, requirements, data, line)?;
        return Ok(());
    }

    if len > index + 1
        && index > 0
        && chars[index - 1] == '<'
        && chars[index] == '='
        && chars[index + 1] == '>'
    {
        create_knowledge(&chars, index + 1, requirements, data, line)?;
        let before = &chars[..index - 1];
        let after = &chars[index + 2..];
        let mut new_string = String::new();
        new_string.push_str(&after.iter().collect::<String>());
        new_string.push_str("=>");
        new_string.push_str(&before.iter().collect::<String>());
        return check_line(new_string.as_str(), data, search);
    }

    Err(format!("Invalid line: {}", line))
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
	debug!("{}", result);
    if !result.is_empty() {
        vec.push(result);
    }
}

pub fn parse_lines(
    lines: Vec<String>,
    data: &mut HashMap<String, Vec<Knowledge>>,
    search: &mut Vec<char>,
) -> Result<(), String> {
    let mut vec: Vec<String> = Vec::new();
    for line in lines {
        clean_line(&line, &mut vec);
    }
    for a in vec {
        debug!("Line : {}", a);
        let priority_line = priority_content(&a);
        println!("{}", priority_line);
        check_line(&priority_line, data, search)?;
    }
    Ok(())
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
