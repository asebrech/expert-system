use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::{thread, time};

use colored::Colorize;

use crate::{data_types, Condition, Knowledge, Requirement};

pub struct KnowledgeEngine {
    pub data:
        std::collections::HashMap<std::string::String, std::vec::Vec<data_types::fact::Knowledge>>, //Need to put vector as we can have several rule for one knowledge
    pub current_symbol: Option<String>,
    pub search: Vec<char>,
}

pub struct KnowledgeCacheManager {
    pub resolved_data: HashMap<String, Option<bool>>, //keeps track of resolved formulas
}

pub fn process_user_input() -> Option<bool> {
    let mut s = String::new();
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

pub fn prove(
    symbol: String,
    engine: &mut KnowledgeEngine,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
) -> Option<bool> {
    get_knowledge_state(&symbol, engine, None, knowledge_cache_manager, 0, false)
}

//check if given knowledge is true, false or none (undetermined)
fn get_knowledge_state(
    symbol: &str,
    engine: &KnowledgeEngine,
    current_calcul: Option<&String>,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    mut depth: usize,
    is_result_symbol: bool,
) -> Option<bool> {
    let knowledge_vec = engine.data.get(symbol);
    depth += 1;
	let ten_millis = time::Duration::from_millis(1000);
	thread::sleep(ten_millis);
	
    if knowledge_vec.is_none() {
        println!(
            "Symbol {} has no knowledge defined, default to false",
            symbol
        );
        if is_result_symbol {
            return None;
        }
        return Some(false);
    }

    let knowledge_vec = knowledge_vec.unwrap();
    if knowledge_vec.is_empty() {
        println!(
            "{}No requirement for {}, default to false.",
            "\t".repeat(depth),
            symbol
        );
        return Some(false);
    } //if ke_vec is a fact, it is stored up front
    for ele in knowledge_vec {
        if ele.fact {
            println!(
                "{}{}{} is a fact that is {}",
                "\t".repeat(depth),
                if ele.not { "!" } else { "" },
                symbol,
                ele.fact && !ele.not
            );
            return Some(ele.fact && !ele.not);
        }
    }
    if !is_result_symbol {
        println!(
            "{}Processing all knowledge of {}, total: {}",
            "\t".repeat(depth),
            symbol.green(),
            knowledge_vec.len().to_string().red()
        );
    }

    let mut answers: Vec<bool> = vec![];
    for knowledge in knowledge_vec.iter() {
        if knowledge.calcul.is_some()
            && current_calcul.is_some()
            && current_calcul.unwrap() == &knowledge.calcul.clone().unwrap()
            && is_result_symbol
        {
            if knowledge_vec.len() == 1 {
                //and knowledge requirement isnt an equal sign, otherwise it is true
                return get_value_from_result_knowledge(knowledge, &knowledge.symbol);
            }
            continue;
        }
        println!(
            "{}Checking requirements for {} with formula {}",
            "\t".repeat(depth),
            symbol.green(),
            knowledge.line.bright_blue()
        );
        let are_req_met: Option<bool>;

        if knowledge.calcul.is_some()
            && knowledge_cache_manager
                .resolved_data
                .contains_key(&knowledge.calcul.clone().unwrap())
        {
            are_req_met = *knowledge_cache_manager
                .resolved_data
                .get(&knowledge.calcul.clone().unwrap())
                .unwrap();
            println!(
                "{}Cached data found for {} => {:?}",
                "\t".repeat(depth),
                knowledge.calcul.clone().unwrap().clone().green(),
                are_req_met.map_or("undetermined".to_string(), |v| v.to_string())
            );
        } else {
            are_req_met = process_formula(
                &knowledge.requirements,
                engine,
                None,
                knowledge_cache_manager,
                depth,
                false,
            );
        }
        if let Some(are_req_met) = are_req_met {
            if !are_req_met && knowledge.not {
                println!("True 1");
                if knowledge.calcul.is_some() {
                    knowledge_cache_manager
                        .resolved_data
                        .insert(knowledge.calcul.clone().unwrap().clone(), Some(true));
                }
                return Some(true);
            }

            if are_req_met && knowledge.not {
                println!(
                    "{}!{} is true one",
                    "\t".repeat(depth),
                    knowledge.symbol.green()
                );
            } else if !are_req_met {
                println!("XXFalse one");
                if knowledge.calcul.is_some() {
                    knowledge_cache_manager
                        .resolved_data
                        .insert(knowledge.calcul.clone().unwrap().clone(), Some(false));
                }
                answers.push(false);
                continue;
            }
        } else {
            println!("{} Default none", "\t".repeat(depth));
            if knowledge.calcul.is_some() {
                knowledge_cache_manager
                    .resolved_data
                    .insert(knowledge.calcul.clone().unwrap().clone(), None);
            }

            return None;
        }
        if let Some(krr) = &knowledge.result_requirement {
            for item in krr.iter() {
                if knowledge.calcul.is_some()
                    && knowledge_cache_manager
                        .resolved_data
                        .contains_key(&knowledge.calcul.clone().unwrap())
                {
                    let temp = knowledge_cache_manager
                        .resolved_data
                        .get(&knowledge.calcul.clone().unwrap())
                        .unwrap();
                    if item.symbol == symbol {
                        println!(
                            "Cached data found for {} => {:?}",
                            knowledge.calcul.clone().unwrap().green(),
                            temp.map_or("undetermined".to_string(), |v| v.to_string())
                        );

                        //might possibly want to return here or perform some action
                        continue;
                    }
                } else {
                    println!("checking {}", symbol.green());
                    let res2 = process_formula(
                        krr,
                        engine,
                        knowledge.calcul.as_ref(),
                        knowledge_cache_manager,
                        depth,
                        true,
                    );
                    if res2.is_none() {
                        //ask user to clarify

                        println!("Undetermined knowledge found, asking user to clarify symbol {}, enter true or false.", symbol);
                        return process_user_input();
                    } else if !res2.unwrap() {
                        //resolution is false
                        println!("Resolution is false for {:?}", knowledge.calcul);
                        //push in array
                        answers.push(false);
                        continue;
                    }
                    if knowledge.calcul.is_some() {
                        knowledge_cache_manager
                            .resolved_data
                            .insert(knowledge.calcul.clone().unwrap().clone(), Some(true));
                    }
                }
            }
        }
        if knowledge.calcul.is_some() {
            knowledge_cache_manager
                .resolved_data
                .insert(knowledge.calcul.clone().unwrap().clone(), Some(true));
        }

        //push true
        answers.push(true);
    }

    println!("{}{} is true two", "\t".repeat(depth), symbol.green());

    if answers.contains(&true) && answers.contains(&false) {
        println!("{}", "Contradiction found".red());
    }
    Some(answers.contains(&true))
}

fn get_value_from_result_knowledge(knowledge: &Knowledge, symbol_to_find: &str) -> Option<bool> {
    let result_requirement = knowledge.result_requirement.as_ref()?;
    let mut prev: Option<Condition> = None;
    let mut found = false;
    for ele in result_requirement.iter() {
        if found && prev.is_some() {
            break;
        }
        if ele.symbol == symbol_to_find {
            found = true;
            if prev.is_some() {
                break;
            }
        } else {
            prev = Some(ele.condition);
        }
    }
    if found && prev.is_some() {
        let cond = prev.unwrap();
        if cond == Condition::AND {
            return Some(true);
        }
    }
    None
}

fn process_knowledge_state(
    requirement: &Requirement,
    engine: &KnowledgeEngine,
    current_calcul: Option<&String>,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    depth: usize,
    is_result_symbol: bool,
) -> Option<bool> {
    let res = get_knowledge_state(
        &requirement.symbol,
        engine,
        current_calcul,
        knowledge_cache_manager,
        depth,
        is_result_symbol,
    );
    if !is_result_symbol {
        return res;
    }
    // parse it as an answer response and return it
    if res.is_none() {
        if requirement.condition == Condition::AND {
            return Some(!requirement.not); //true if not is false and false if not is true, M A G I C
        }
        if requirement.condition == Condition::OR {
            return None;
        }
    }
    res
}

fn process_formula(
    requirements: &[Requirement],
    brain: &KnowledgeEngine,
    current_calcul: Option<&String>,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    depth: usize,
    is_result_symbol: bool,
) -> Option<bool> {
    let first_req = requirements.first().unwrap();
    if requirements.len() <= 1 {
        return process_knowledge_state(
            first_req,
            brain,
            current_calcul,
            knowledge_cache_manager,
            depth,
            is_result_symbol,
        );
    }
    let second_req = requirements.get(1).unwrap();
    let mut previous = second_req;
    let lhs = process_knowledge_state(
        first_req,
        brain,
        current_calcul,
        knowledge_cache_manager,
        depth,
        is_result_symbol,
    );
    let mut rhs = process_knowledge_state(
        second_req,
        brain,
        current_calcul,
        knowledge_cache_manager,
        depth,
        is_result_symbol,
    );
    if lhs.is_none() || rhs.is_none() {
        return None;
    }
    let mut lhs = compare_boolean(
        (lhs.unwrap() && !first_req.not) || (!lhs.unwrap() && first_req.not),
        (rhs.unwrap() && !second_req.not) || (!rhs.unwrap() && second_req.not),
        first_req.condition,
    );
    if requirements.len() == 2 {
        return Some(lhs);
    }
    let to_iter = requirements.iter().skip(2);
    for item in to_iter {
        rhs = process_knowledge_state(
            item,
            brain,
            current_calcul,
            knowledge_cache_manager,
            depth,
            is_result_symbol,
        );
        if rhs? {
            return None;
        }
        lhs = compare_boolean(
            (lhs && !previous.not) || (!lhs && previous.not),
            (rhs.unwrap() && !item.not) || (!rhs.unwrap() && item.not),
            previous.condition,
        );
        previous = item;
    }

    Some(lhs)
}

fn compare_boolean(lhs: bool, rhs: bool, condition: Condition) -> bool {
    match condition {
        Condition::AND => lhs && rhs,
        Condition::OR => lhs || rhs,
        Condition::XOR => lhs != rhs,
        _ => rhs,
    }
}
