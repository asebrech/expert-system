use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Write};
use std::{thread, time};

use colored::Colorize;

use crate::data_types::fact::Condition;
use crate::{data_types, Knowledge};

pub struct KnowledgeEngine {
    pub data: std::collections::HashMap<std::string::String, std::vec::Vec<data_types::fact::Knowledge>>, //Need to put vector as we can have several rule for one knowledge
    pub current_symbol: Option<String>,
    pub search: Vec<char>,
}

pub struct KnowledgeCacheManager {
    pub resolved_data: HashMap<String, Option<bool>>, //keeps track of resolved formulas
    pub previous_line: Option<String>,
    pub resolve_stack: HashSet<String>,
}

pub fn process_user_input(symbol: &str) -> Option<bool> {
    let mut s = String::new();
    println!(
        "Undetermined knowledge found, asking user to clarify symbol {}, enter true or false.",
        symbol
    );
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    Some(s == "true")
}

/*
Char: '├' => $'\342\224\234'
Char: '─' => $'\342\224\200'
Char: '│' => $'\342\224\202'
Char: ' ' => $'\302\240'
Char: '└' => $'\342\224\224'
*/

fn print_line(depth: usize, data: String) {
    let mut chars: String = String::new();
    let mut i = 0;
    while i < depth {
        if i == 0 {
            chars.push('│');
        } else if i % 2 == 0 {
            chars.push('─');
        } else if i % 2 != 0 {
            chars.push('├');
        }else if i % 2 == 0 && i - 2 > 0 {
            chars.push('└');
        } else {
            chars.push(' ');
        }
        i += 1;
    }
    println!("{}{}", chars, data);
}

fn compare_boolean(lhs: bool, rhs: bool, condition: Condition) -> bool {
    match condition {
        Condition::AND => lhs && rhs,
        Condition::OR => lhs || rhs,
        Condition::XOR => lhs != rhs,
        _ => rhs,
    }
}

pub fn neo_prove(symbol: String,
    engine: &mut KnowledgeEngine,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
) -> Option<bool> {
    knowledge_cache_manager.resolve_stack.clear();
    knowledge_cache_manager.resolved_data.clear();
    return Some(get_symbol_value(&symbol, engine, knowledge_cache_manager, 0));
}

fn resolve_lhs_knowledge(
    knowledge: &Knowledge,
    engine: &KnowledgeEngine,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    mut depth: usize,
) -> bool {
    let mut condition: Option<Condition> = None;
    let mut previous_value: Option<bool> = None;
    let mut current_value: Option<bool> = None;

    for item in knowledge.requirements.iter() {
        if previous_value.is_some() {
            current_value = Some(get_symbol_value(&item.symbol, engine, knowledge_cache_manager, depth));
        } else {
            previous_value = Some(get_symbol_value(&item.symbol, engine, knowledge_cache_manager, depth));
        }
        if previous_value.is_some() && current_value.is_some() && condition.is_some() {
            previous_value = Some(compare_boolean(previous_value.unwrap(), current_value.unwrap(), condition.unwrap()));
        }
        condition = Some(item.condition)
    }
    return if previous_value.is_some() {previous_value.unwrap()} else {false};
}

fn resolve_rhs_knowledge(
    knowledge: &Knowledge,
    engine: &KnowledgeEngine,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    mut depth: usize,
) -> bool {
    false
}

pub fn get_symbol_value(
    symbol: &str,
    engine: &KnowledgeEngine,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    mut depth: usize,
) -> bool {
    //print_line(depth, format!("Resolving {}", symbol));
    let initial_vec = engine.data.get(symbol);
    depth += 1;
    if !cfg!(test) {
        let ten_millis = time::Duration::from_millis(200);
        thread::sleep(ten_millis);
    }
    if initial_vec.is_none() {
        print_line(depth, format!("{} is false", symbol));
        return false;
    }
    let initial_vec = initial_vec.unwrap();
    for knowledge in initial_vec.iter() {
        if knowledge.fact {
            print_line(
                depth,
                format!(
                    "{}{} is a fact that is {}",
                    if knowledge.not { "!" } else { "" },
                    symbol,
                    knowledge.fact && !knowledge.not
                ),
            );
            return knowledge.fact && !knowledge.not;
        }
    }
    //probably add the has same symbol in knowledge around here

    let mut answers: Vec<bool> = vec![];
    for knowledge in initial_vec.iter() {
        print_line(depth, format!("processing {:?}", knowledge.line));
        let lhs_value = resolve_lhs_knowledge(
            knowledge,
            engine,
            knowledge_cache_manager,
            depth,
        );
        if lhs_value {
            if knowledge.result_requirement.is_some() {
                //resolve the result requirement here, and find 
                //if the resolution is correct and symbol is correct
            }
        }
        answers.push(lhs_value);
    }
    let final_result = answers.contains(&true);
    if final_result && answers.contains(&false) {
        println!("{}", "Contradiction found".red());
        println!(
            "{}",
            "If one of the answers are true, the symbol will be true".yellow()
        );
    }
    //il manque le !
    print_line(depth, format!("{} is {}",
    symbol.green(),
    final_result));
    knowledge_cache_manager
        .resolve_stack
        .remove(&symbol.to_string());
    println!("returning {} {}", symbol, final_result);
    return final_result;
}
