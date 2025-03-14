use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Write};
use std::{env, thread, time};

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

pub fn prove(
    symbol: String,
    engine: &mut KnowledgeEngine,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
) -> Option<bool> {
    knowledge_cache_manager.resolve_stack.clear();
    knowledge_cache_manager.resolved_data.clear();
    get_knowledge_state(&symbol, engine, None, knowledge_cache_manager, 0, false)
}

fn has_symbol_in_knowledge(symbol: &str, vec: &Vec<Requirement>) -> bool {
    for ele in vec {
        if ele.symbol == symbol {
            println!("Cyclic dependency found for symbol {}", symbol);
            return true;
        }
    }
    return false;
}


fn print_line(depth: usize, data: String) {
    let mut chars: Vec<char> = Vec::new();

    let mut i = 0;
    while i < depth {
        if i == 0 {
            chars.push('│');
        } else if i % 2 == 0 {
            chars.push('─');
        } else if i % 2 != 0 {
            chars.push('├');
        } else {
            chars.push(' '); 
        }
        i += 1;
    }
    println!("{:?}{}", chars, data);

}

//check if given knowledge is true, false or none (undetermined)
fn get_knowledge_state(
    symbol: &str, //[A + B]
    engine: &KnowledgeEngine,
    current_calcul: Option<&String>,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    mut depth: usize,
    is_result_symbol: bool,
) -> Option<bool> {
    let initial_vec = engine.data.get(symbol);
    depth += 1;
    if !cfg!(test) {
        let ten_millis = time::Duration::from_millis(200);
        thread::sleep(ten_millis);
    }

    if initial_vec.is_none() {
        print_line(depth, format!("{}{} is false", "\t".repeat(depth),
            symbol));
        if is_result_symbol {
            return None;
        }
        return Some(false);
    }

    let initial_vec = initial_vec.unwrap();
    if initial_vec.is_empty() {
        println!(
            "{}No requirement for {}, default to false.",
            "\t".repeat(depth),
            symbol
        );
        return Some(false);
    } //if ke_vec is a fact, it is stored up front
    for ele in initial_vec {
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

    let mut answers: Vec<bool> = vec![];

    // println!("Before for {} {:?}", symbol, knowledge_cache_manager.previous_line);
    let mut vector_t = vec![];
    for knowledge in initial_vec.iter() {
        if knowledge_cache_manager.previous_line.is_some()
            && knowledge.line == knowledge_cache_manager.previous_line.clone().unwrap()
        {
            //println!("Found lines {}", knowledge.line);
            if !has_symbol_in_knowledge(symbol, &knowledge.requirements) {
                vector_t.push(knowledge.clone());
            } else {
                knowledge_cache_manager.resolve_stack.remove(symbol);
                println!(" RES SYM  1 {} {}", symbol, is_result_symbol);

                return process_user_input(symbol);
            }
        }
    }
    if vector_t.len() == 0 {
        for knowledge in initial_vec.iter() {
            if !has_symbol_in_knowledge(symbol, &knowledge.requirements) {
                vector_t.push(knowledge.clone());
            } else {
                knowledge_cache_manager.resolve_stack.remove(symbol);
                println!(" RES SYM 2 {} {}", symbol, is_result_symbol);

                return process_user_input(symbol);
            }
        }
    }
    //println!("{} {:?} {}", symbol, knowledge_cache_manager.previous_line, vector_t.len());
    for knowledge in vector_t.iter() {
        // println!("{} {} {:?}", knowledge.symbol, knowledge.line, knowledge_cache_manager.previous_line);
        //println!("{} {} {}", knowledge.symbol, engine.current_symbol.clone().unwrap(), knowledge.line);
         
        if knowledge_cache_manager
            .resolve_stack
            .contains(&knowledge.symbol.to_string())
            && !is_result_symbol
        {
            println!("Dependence cyclique for {}", symbol);
            knowledge_cache_manager
                .resolve_stack
                .remove(&knowledge.symbol);
            match env::var("EXPERT_MODE") {
                Ok(_v) => {
                    knowledge_cache_manager
                        .resolve_stack
                        .remove(&symbol.to_string());
                    println!(" RES SYM 3 {} {}", knowledge.symbol, is_result_symbol);

                    return process_user_input(&knowledge.symbol);
                }
                Err(_e) => {
                    knowledge_cache_manager
                        .resolve_stack
                        .remove(&symbol.to_string());
                    return None;
                }
            }
        }
        if !is_result_symbol {
            knowledge_cache_manager
                .resolve_stack
                .insert(symbol.to_string());
        }
        if knowledge.calcul.is_some()
            && current_calcul.is_some()
            && current_calcul.unwrap() == &knowledge.calcul.clone().unwrap()
            && is_result_symbol
        {
            if vector_t.len() == 1 {
                //and knowledge requirement isnt an equal sign, otherwise it is true
                knowledge_cache_manager
                    .resolve_stack
                    .remove(&symbol.to_string());
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
        if symbol.len() == 1 {
            knowledge_cache_manager.previous_line = Some(knowledge.line.to_string());
        }
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
                symbol,
                &knowledge.requirements,
                engine,
                None,
                knowledge_cache_manager,
                depth,
                false,
            );
            //println!("ENEVER  {:?}", are_req_met)
        }
        if let Some(are_req_met) = are_req_met {
            if !are_req_met && knowledge.not {
                println!("True 1");
                if knowledge.calcul.is_some() {
                    knowledge_cache_manager
                        .resolved_data
                        .insert(knowledge.calcul.clone().unwrap().clone(), Some(true));
                }
                knowledge_cache_manager
                    .resolve_stack
                    .remove(&symbol.to_string());
                return Some(true);
            }

            if are_req_met && knowledge.not || !are_req_met {
                if are_req_met && knowledge.not {
                    println!(
                        "{}!{} is true one",
                        "\t".repeat(depth),
                        knowledge.symbol.green()
                    );
                }
                println!("XXFalse one");
                if knowledge.calcul.is_some() {
                    knowledge_cache_manager
                        .resolved_data
                        .insert(knowledge.calcul.clone().unwrap().clone(), Some(false));
                }
                knowledge_cache_manager
                    .resolve_stack
                    .remove(&knowledge.symbol);
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
            knowledge_cache_manager
                .resolve_stack
                .remove(&symbol.to_string());
            return None;
        }
        if let Some(krr) = &knowledge.result_requirement {
            //loop sur tout les req, garder les valeurs dans un array
            //return la reponse en fonction de si le symbol que l on a rechercher est true ou false
            let mut res: HashMap<String, Option<bool>> = HashMap::new();
            for result_requirement in &knowledge.result_requirement {
                let mut prev: Option<Requirement> = None;
                for ele in result_requirement.iter() {
                    println!("checking {} {}", ele.symbol, prev.is_some());
                    if prev.is_some() {
                        let unwr = prev.unwrap();
                        println!("cond : {:?} {:?}", unwr.condition, res);
                        if unwr.condition == Condition::AND {
                            res.insert(ele.symbol.to_string(), Some(true));
                        } else if unwr.condition == Condition::XOR {
                            let cur = res.get(&unwr.symbol);
                            if cur.is_some() {
                                res.insert(ele.symbol.to_string(), Some(!cur.unwrap().unwrap()));
                            }
                        } else {
                            res.insert(ele.symbol.to_string(), process_user_input(&ele.symbol));
                        }
                    } else {
                        if ele.condition == Condition::AND {
                            res.insert(ele.symbol.to_string(), Some(true));
                        } else {
                            res.insert(ele.symbol.to_string(), process_user_input(&ele.symbol));
                        }
                    }
                    prev = Some(ele.clone());
                }
            }
            println!("returning {}", symbol);
            let r = res.get(symbol);
            if r.is_some() {
                return *r.unwrap();
            }
            return None;
        }
        if knowledge.calcul.is_some() {
            knowledge_cache_manager
                .resolved_data
                .insert(knowledge.calcul.clone().unwrap().clone(), Some(true));
        }

        //push true
        knowledge_cache_manager
            .resolve_stack
            .remove(&knowledge.symbol);
        answers.push(true);
    }

    let final_result = answers.contains(&true);
    if final_result && answers.contains(&false) {
        println!("{}", "Contradiction found".red());
        println!(
            "{}",
            "If one of the answers are true, the symbol will be true".yellow()
        );
    }
    println!(
        "{}{} is {} two",
        "\t".repeat(depth),
        symbol.green(),
        final_result
    );
    knowledge_cache_manager
        .resolve_stack
        .remove(&symbol.to_string());
    println!("returning {} {}", symbol, final_result);
    Some(final_result)
}

fn get_value_from_result_knowledge(knowledge: &Knowledge, symbol_to_find: &str) -> Option<bool> {
    let result_requirement = knowledge.result_requirement.as_ref()?;
    let mut prev: Option<Condition> = None;
    let mut found = false;
    for ele in result_requirement.iter() {
        println!("req {:?}", ele);
        if found && prev.is_some() {
            break;
        }
        if ele.symbol == symbol_to_find {
            found = true;
            if prev.is_some() {
                break;
            } else {
                prev = Some(ele.condition);
            }
        } else {
            prev = Some(ele.condition);
        }
    }
    if found && prev.is_some() {
        let cond = prev.unwrap();
        println!("yelleelle {:?}", prev);
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
    symbol: &str,
    requirements: &[Requirement],
    brain: &KnowledgeEngine,
    current_calcul: Option<&String>,
    knowledge_cache_manager: &mut KnowledgeCacheManager,
    depth: usize,
    is_result_symbol: bool,
) -> Option<bool> {
    let first_req = requirements.first().unwrap();
    if requirements.len() <= 1 {
        let e = process_knowledge_state(
            first_req,
            brain,
            current_calcul,
            knowledge_cache_manager,
            depth,
            is_result_symbol,
        );
        //println!("adieu {:?}", e);
        return e;
    }
    let second_req = requirements.get(1).unwrap();
    let mut previous = second_req;
    let mut lhs = process_knowledge_state(
        first_req,
        brain,
        current_calcul,
        knowledge_cache_manager,
        depth,
        is_result_symbol,
    );
    if lhs.is_none() {
        match env::var("EXPERT_MODE") {
            Ok(_v) => {
                if !is_result_symbol {
                    knowledge_cache_manager
                    .resolve_stack
                    .remove(&first_req.symbol);
                } else {
                    println!(" RES SYM 5 {} {}", first_req.symbol, is_result_symbol);

                    lhs = process_user_input(&first_req.symbol);
                    if symbol == first_req.symbol {
                        return lhs;
                    }
                }
            }
            Err(_e) => {
                return None;
            }
        }
    }
    let mut rhs = process_knowledge_state(
        second_req,
        brain,
        current_calcul,
        knowledge_cache_manager,
        depth,
        is_result_symbol,
    );
    if rhs.is_none() {
        match env::var("EXPERT_MODE") {
            Ok(_v) => {
                if !is_result_symbol {
                    knowledge_cache_manager
                    .resolve_stack
                    .remove(&second_req.symbol);
                } else {
                    println!(" RES SYM 6 {} {}", second_req.symbol, is_result_symbol);
                    if first_req.condition == Condition::OR {
                        rhs = process_user_input(&second_req.symbol);
                    } else {
                        rhs = Some(!(lhs.unwrap()));
                    }
                }
            }
            Err(_e) => {
                return None;
            }
        }
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
            knowledge_cache_manager.resolve_stack.remove(&item.symbol);
            match env::var("EXPERT_MODE") {
                Ok(_v) => {
                    println!(" RES SYM 7 {} {}", item.symbol, is_result_symbol);
                    if !is_result_symbol {
                        rhs = process_user_input(&item.symbol);
                    }
                }
                Err(_e) => {}
            }
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
