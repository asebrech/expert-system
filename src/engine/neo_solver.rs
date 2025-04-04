use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Write};
use std::{thread, time};

use colored::Colorize;

use crate::data_types::fact::{Condition, Requirement};
use crate::{data_types, Knowledge};

pub struct KnowledgeEngine {
    pub data: std::collections::HashMap<std::string::String, std::vec::Vec<data_types::fact::Knowledge>>, //Need to put vector as we can have several rule for one knowledge
    pub current_symbol: Option<String>,
    pub search: Vec<char>,
}

pub struct KnowledgeCacheManager {
    pub resolved_data: HashMap<String, bool>, //keeps track of resolved formulas
    pub previous_line: Option<String>,
    pub result_knowledge_stack: HashSet<String>,
    pub rhs_symbol_map: Vec<(HashMap<String, bool>, bool)>,
    pub resolve_stack: HashSet<String>,
}

pub fn process_user_input(symbol: &str) -> bool {
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
    s == "true"
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
    knowledge_cache_manager.previous_line = None;
    knowledge_cache_manager.result_knowledge_stack.clear();
    knowledge_cache_manager.rhs_symbol_map.clear();
    return get_symbol_value(&symbol, engine, knowledge_cache_manager, false, 0);
}

fn resolve_lhs_knowledge(
    knowledge: &Knowledge,
    engine: &KnowledgeEngine,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    depth: usize,
) -> bool {
    let mut condition: Option<Condition> = None;
    let mut previous_value: Option<bool> = None;
    let mut current_value: Option<bool> = None;

    for item in knowledge.requirements.iter() {
        if previous_value.is_some() {
			let sym = get_symbol_value(&item.symbol, engine, knowledge_cache_manager, true, depth);
            if sym.is_some() {
                current_value = if item.not { Some(!sym.unwrap() && item.not) } else { sym };
            } else {
                current_value = sym;
            }
            
        } else {
			let sym = get_symbol_value(&item.symbol, engine, knowledge_cache_manager, true, depth);
            if sym.is_some() {
                previous_value = if item.not { Some(!sym.unwrap() && item.not) } else { sym };
            } else {
                previous_value = sym;
            }
        }
        if previous_value.is_some() && current_value.is_some() && condition.is_some() {
            previous_value = Some(compare_boolean(previous_value.unwrap(), current_value.unwrap(), condition.unwrap()));
        }
        condition = Some(item.condition)
    }
    return if previous_value.is_some() {previous_value.unwrap()} else {false};
}

fn resolve_rhs_knowledge(
    requirements: &Vec<Requirement>,
    engine: &KnowledgeEngine,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    depth: usize,
) -> (HashMap<String, bool>, bool) {
    let mut condition: Condition = Condition::END;
    let mut previous_value: bool = false;
    let mut current_value: bool = false;
	let mut resolved_map: HashMap<String, bool> = HashMap::new();

    for (i, item) in requirements.iter().enumerate() {
        if i > 0 {
			//the loop has already iterated once and previous value is properly set
            current_value = get_rhs_requirement_value(Some(previous_value), &item,  &condition, &mut resolved_map, engine, knowledge_cache_manager, depth);
			resolved_map.insert(item.symbol.clone(), current_value);
        } else {
			//if the loop is only starting to iterate
			condition = item.condition;
            previous_value = get_rhs_requirement_value(None, &item,  &condition, &mut resolved_map, engine, knowledge_cache_manager, depth);
			resolved_map.insert(item.symbol.clone(), previous_value);
        }
        if i > 0 && condition != Condition::END {
            previous_value = compare_boolean(previous_value, current_value, condition);
        }

    }
    return (resolved_map, previous_value);
}

pub fn knowledge_contain_symbol(symbol: &str, knowledge: &Knowledge, check_lhs: bool, check_requirement: bool) -> bool {
	let mut res = false;
	if check_lhs {
		knowledge.requirements.iter().for_each(|e| {
			if e.symbol == symbol {
				res = true;
			}
		});
	}
	if check_requirement && knowledge.result_requirement.is_some() {
		knowledge.result_requirement.clone().unwrap().iter().for_each(|know| {
			if know.symbol == symbol {
				res = true;
			}
		});
	}
	return res;
}

pub fn get_rhs_requirement_value(
    precedent_value: Option<bool>,
	requirement: &Requirement,
	precedent_condition: &Condition,
	resolved_map: &mut HashMap<String, bool>,
    engine: &KnowledgeEngine,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    depth: usize) -> bool {
		print_line(depth, format!("processing {:?}", requirement.symbol));
		if *precedent_condition == Condition::AND {
			return true;
		}

		if let Some(sym) = resolved_map.get(&requirement.symbol) {
			return *sym;
		}
		let sym = get_symbol_value(&requirement.symbol, engine, knowledge_cache_manager, true, depth);
        if sym.is_none() && *precedent_condition == Condition::XOR && precedent_value.is_some() {
            return !precedent_value.unwrap();
        }

        if sym.is_none() {
            let user_input = process_user_input(&requirement.symbol);
            return user_input;
        } else {
            return sym.unwrap();
        }
	}

pub fn get_rhs_symbol_value_from_resolved_map(symbol: &str, data: &(HashMap<String, bool>, bool)) -> Option<bool> {
    let rhs_value: Option<bool>;
    if data.1 == false {
        println!("The evaluated result requirement is false, the condition cannot be proven");
        //checking if the value was still given true
        rhs_value = Some(false);
    } else {
        let sym_val = data.0.get(symbol);
        if sym_val.is_some() {
            rhs_value = Some(*sym_val.unwrap());
        } else {
            println!("Error occured, no result symbol found in result requirement");
            rhs_value = None;
        }
    }
    return rhs_value;
}

pub fn get_symbol_value(
    symbol: &str,
    engine: &KnowledgeEngine,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    is_result_symbol: bool,
    mut depth: usize,
) -> Option<bool> {
    let initial_vec = engine.data.get(symbol);
    depth += 1;
    if !cfg!(test) {
        let ten_millis = time::Duration::from_millis(200);
        thread::sleep(ten_millis);
    }
    if initial_vec.is_none() {
        print_line(depth, format!("{} is false", symbol));
        return Some(false);
    }
    let mut initial_vec = initial_vec.unwrap().clone();
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
            return Some(knowledge.fact &&
			 !knowledge.not);
        }
    }
    if knowledge_cache_manager.resolve_stack.contains(symbol) {
        if !is_result_symbol {
            return Some(false);
        }
    }
    knowledge_cache_manager.resolve_stack.insert(symbol.to_string());
    
    let mut answers: Vec<bool> = vec![];
    if symbol.len() > 1 && knowledge_cache_manager.previous_line.is_some() {
        let prev_line = knowledge_cache_manager.previous_line.clone().unwrap();
        initial_vec = initial_vec.iter() 
        .filter(|e| e.line == prev_line)
        .cloned() 
        .collect(); 
    }

    for knowledge in initial_vec.iter() {
        knowledge_cache_manager.previous_line = Some(knowledge.line.clone());
        if knowledge_cache_manager.result_knowledge_stack.contains(&knowledge.line) {
            if knowledge_cache_manager.rhs_symbol_map.len() > 0 {
                for item in knowledge_cache_manager.rhs_symbol_map.iter() {
                    let some_rhs: Option<bool> = get_rhs_symbol_value_from_resolved_map(symbol, &item);
                    if some_rhs.is_some() {
                        answers.push(some_rhs.unwrap());
                    }
                }
            }
           continue;
        }
        if knowledge.symbol != symbol && !knowledge_contain_symbol(symbol, knowledge, false, true) {
            println!("{}", "Does not contain !".red());
        }

		if knowledge_cache_manager.resolved_data.contains_key(symbol) {
			answers.push(*knowledge_cache_manager.resolved_data.get(symbol).unwrap());
			continue;
		}
        print_line(depth, format!("processing {:?}", knowledge.line));
        let lhs_value = resolve_lhs_knowledge(
            knowledge,
            engine,
            knowledge_cache_manager,
            depth,
        );
		let mut rhs_value = true;
        if lhs_value || !lhs_value && knowledge.not {
            if knowledge.result_requirement.is_some() {
				knowledge_cache_manager.previous_line = Some(knowledge.line.clone());
                knowledge_cache_manager.result_knowledge_stack.insert(knowledge.line.clone());
                let tuple = resolve_rhs_knowledge(&knowledge.result_requirement.clone().unwrap(), engine, knowledge_cache_manager, depth);
                knowledge_cache_manager.rhs_symbol_map.push(tuple.clone());

                let some_rhs: Option<bool> = get_rhs_symbol_value_from_resolved_map(symbol, &tuple);
                if some_rhs.is_some() {
                    rhs_value = some_rhs.unwrap();
                } else {
                    continue;
                }
            }
        }
        answers.push(lhs_value && rhs_value);
    }

    knowledge_cache_manager.resolve_stack.remove(symbol);
    let final_result = answers.contains(&true);
    if answers.is_empty() && is_result_symbol {
        return None;
    }
    if final_result && answers.contains(&false) {
        println!("{}", "Contradiction found".red());
        println!(
            "{}",
            "If one of the answers are true, the symbol will be true".yellow()
        );
    }
    print_line(depth, format!("{} is {}",
    symbol.green(),
    final_result));
    knowledge_cache_manager
        .resolve_stack
        .remove(&symbol.to_string());
	knowledge_cache_manager.resolved_data.insert(symbol.to_string(), final_result);
    return Some(final_result);
}
