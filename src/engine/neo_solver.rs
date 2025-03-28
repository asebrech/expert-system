use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Write};
use std::iter::Map;
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
			let sym = get_symbol_value(&item.symbol, engine, knowledge_cache_manager, depth);
			println!("curr {} {}", item.symbol, if item.not { !sym && item.not } else { sym });
            current_value = Some(if item.not { !sym && item.not } else { sym });
        } else {
			let sym = get_symbol_value(&item.symbol, engine, knowledge_cache_manager, depth);
            previous_value = Some(if item.not { !sym && item.not } else { sym });
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
    mut depth: usize,
) -> (HashMap<String, bool>, bool) {
    let mut condition: Condition = Condition::END;
    let mut previous_value: bool = false;
    let mut current_value: bool = false;
	let mut resolved_map: HashMap<String, bool> = HashMap::new();

    for (i, item) in requirements.iter().enumerate() {
        if i > 0 {
			//the loop has already iterated once and previous value is properly set
            current_value = get_rhs_requirement_value(&item,  &condition, &resolved_map, engine, knowledge_cache_manager, depth);
			resolved_map.insert(item.symbol.clone(), current_value);
        } else {
			//if the loop is only starting to iterate
			condition = item.condition;
            previous_value = get_rhs_requirement_value(&item,  &condition, &resolved_map, engine, knowledge_cache_manager, depth);
			resolved_map.insert(item.symbol.clone(), previous_value);
        }
        if i > 0 && condition != Condition::END {
            previous_value = compare_boolean(previous_value, current_value, condition);
			resolved_map.insert(item.symbol.clone(), previous_value);
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
	requirement: &Requirement,
	precedent_condition: &Condition,
	resolved_map: &HashMap<String, bool>,
    engine: &KnowledgeEngine,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    mut depth: usize) -> bool {
		print_line(depth, format!("processing {:?}", requirement.symbol));
		if *precedent_condition == Condition::AND {
			return true;
		}

		if let Some(sym) = resolved_map.get(&requirement.symbol) {
			return *sym;
		}
		let sym = get_symbol_value(&requirement.symbol, engine, knowledge_cache_manager, depth);
		let exists = engine.data.get(&requirement.symbol);
		if sym == false  {
			let mut is_undefined = exists.is_none();

			if exists.is_some() {
				let e: &Vec<Knowledge> = exists.unwrap();
				//println!("init vec is empty {:?}", e);
				let final_vec: Vec<_> = e.iter().filter(|know| knowledge_contain_symbol(&requirement.symbol, know, true, true)).cloned().collect();
				//println!("Final vec is empty {}, {:?}", final_vec.len(), final_vec);
				if final_vec.len() == 0{
					is_undefined = true;
				}
			}

			if is_undefined {
			//undefined result symbol
				return process_user_input(&requirement.symbol);
			}

		}
		return false;
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
        print_line(depth, format!("AAA {} is false", symbol));
        return false;
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
            return knowledge.fact &&
			 !knowledge.not;
        }
    }
    if knowledge_cache_manager.resolve_stack.contains(symbol) {
        println!("Cyclic for {}", symbol);
        return false;
    }
    knowledge_cache_manager.resolve_stack.insert(symbol.to_string());
    //probably add the has same symbol in knowledge around here
    
    let mut answers: Vec<bool> = vec![];
    if symbol.len() > 1 && knowledge_cache_manager.previous_line.is_some() {
        let prev_line = knowledge_cache_manager.previous_line.clone().unwrap();
        initial_vec = initial_vec.iter() // Use `iter()` instead of `into_iter()` to avoid moving ownership
        .filter(|e| e.line == prev_line)
        .cloned() // Clone elements to get owned `fact::Knowledge`
        .collect(); // Collect into a new `Vec<fact::Knowledge>`
    }

    for (i, knowledge) in initial_vec.iter().enumerate() {
        knowledge_cache_manager.previous_line = Some(knowledge.line.clone());
        
        //println!("line : {}", knowledge.line.red());
        if knowledge.symbol != symbol && !knowledge_contain_symbol(symbol, knowledge, false, true) {
            println!("{}", "Does not contain !".red());
        } else {
          // println!("{}", format!("{} {:?}", symbol.blue(), knowledge));
        }

		if knowledge_cache_manager.resolved_data.contains_key(symbol) {
			println!("Cached");
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
		let mut rhs_value = true; //true by default, will be modified if a knowledge result exists
        if lhs_value || !lhs_value && knowledge.not {
            if knowledge.result_requirement.is_some() {
				knowledge_cache_manager.previous_line = Some(knowledge.line.clone());
				let res = resolve_rhs_knowledge(&knowledge.result_requirement.clone().unwrap(), engine, knowledge_cache_manager, depth);
				if res.1 == false {
					println!("The evaluated result requirement is false, the condition cannot be proven");
					//checking if the value was still given true
					rhs_value = false;
				} else {
					let sym_val = res.0.get(symbol);
					if sym_val.is_some() {
						rhs_value = *sym_val.unwrap();
					} else {
						println!("Error occured, no result symbol found in result requirement");
						rhs_value = false;
					}
				}
				answers.push(lhs_value && rhs_value);
                //resolve the result requirement here, and find 
                //if the resolution is correct and symbol is correct
            }
        }
        answers.push(lhs_value && rhs_value);
    }
    knowledge_cache_manager.resolve_stack.remove(symbol);
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
	knowledge_cache_manager.resolved_data.insert(symbol.to_string(), final_result);
    return final_result;
}
