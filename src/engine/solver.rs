pub mod solver {

    use std::collections::HashMap;

    use crate::{data_types, Condition, Requirement};

    pub struct KnowledgeEngine {
        pub data:  std::collections::HashMap<std::string::String, std::vec::Vec<data_types::fact::Knowledge>>, //Need to put vector as we can have several rule for one knowledge
        pub resolved_data: HashMap<String, Option<bool>>, //map formula to a boolean
        pub expert_mode: bool,
    }

    pub fn prove(symbol: String, engine: &mut KnowledgeEngine) -> Option<bool> {
        let mut symbol_met: Vec<String> = Vec::new();
        return get_knowledge_state(&symbol, engine, &mut symbol_met, 0);
    }

    //check if given knowledge is true, false or none (undetermined)
    //Some(true) => true
    //Some(false) => false
    //None => undetermined
    fn get_knowledge_state(
        symbol: &str,
        engine: &KnowledgeEngine,
        symbol_met: &mut Vec<String>,
        mut depth: usize,
    ) -> Option<bool> {
        let knowledge_vec = engine.data.get(symbol);
        //println!("{}Evaluating : {}", "\t".repeat(depth), symbol);
        if symbol_met.contains(&symbol.to_string()) {
            println!("{}Recursion found for symbol: {}", "\t".repeat(depth), symbol);
            return None;
        }
        symbol_met.push(symbol.to_string());
        depth += 1;
        if knowledge_vec.is_none() {
            return None;
        }

        let ke_vec = knowledge_vec.unwrap();
        if ke_vec.len() == 0 {
            println!("{}No requirement for {}",  "\t".repeat(depth), symbol);
            return None;
        }

        //if ke_vec is a fact, it is stored up front
        for ele in ke_vec {
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

        println!("{}Processing all knowledge of {}, total: {}", "\t".repeat(depth), symbol, ke_vec.len());
        for knowledge in ke_vec.iter() {
            println!("{}Checking formula: {}", "\t".repeat(depth), knowledge.calcul);

            let are_req_met = are_requirements_met(&knowledge.requirements, engine, symbol_met, depth);
            //cas not C is true
            if let Some(are_req_met) = are_req_met {
                //si le req est false, et que la knowledge veux que sa n existe pas
                if are_req_met == false && knowledge.not {
                    println!("True 1");
                    return Some(true);
                }

                if are_req_met && knowledge.not {
                    println!("{}!{} is true", "\t".repeat(depth), knowledge.symbol);
                } else {
                    println!("{}{} is {}","\t".repeat(depth),  knowledge.symbol, are_req_met);
                    if are_req_met == false {
                        println!("False one");
                        return Some(false);
                    }
                }
            } else {
                //println!("{}Default none", "\t".repeat(depth));
                return None;
            }
            println!("{}------------------------------------------------", "\t".repeat(depth));
        }
        Some(true)
    }

    fn are_requirements_met(
        requirements: &Vec<Requirement>,
        brain: &KnowledgeEngine,
        symbol_met: &mut Vec<String>,
        mut depth: usize,
    ) -> Option<bool> {
        //init with the first requirement
        let first_req = requirements.first().unwrap();

        //skip first element, then iter on every others
        let requirement_iter = requirements.iter().skip(1);
        //will have problem here with the first one
        let mut last_condition = first_req.condition;
        let initial_last_res = match first_req.condition {
            Condition::AND => true,
            Condition::END => true,
            Condition::OR => false,
            Condition::XOR => false,
        };
        let mut last_result =
            match_requirement(first_req, brain, first_req.condition, initial_last_res, false, symbol_met, depth);
        if last_result.is_none() || last_result.unwrap() == false {
           // println!("{}First requirement arent met","\t".repeat(depth));
            return None;
        }
        for requirement in requirement_iter {
            if brain.expert_mode == false {
                if requirement.condition == Condition::OR {
                    return None;
                }
            }

            last_result = match_requirement(
                requirement,
                brain,
                requirement.condition,
                last_result.unwrap(),
                requirement.condition == last_condition,
                symbol_met,
                depth + 1,
            );
            if last_result.is_none() || last_result.unwrap() == false {
                println!("{}Requirement failed to be satisfied","\t".repeat(depth));
                return None;
            }
            last_condition = requirement.condition;
        }

        Some(true)
    }

    fn match_requirement(
        current: &Requirement,
        brain: &KnowledgeEngine,
        condition: Condition,
        last_result: bool,
        same_condition: bool,
        symbol_met: &mut Vec<String>,
        depth: usize,
    ) -> Option<bool> {
        let current_knowledge_truthy = get_knowledge_state(&current.symbol, brain, symbol_met, depth);

    let mut current_knowledge = false;
        if brain.expert_mode == true {
            if current_knowledge_truthy.is_none() {
                println!("{}{} is undetermined","\t".repeat(depth),  current.symbol, );
                return None;
            }
            current_knowledge = current_knowledge_truthy.unwrap();
        } else if brain.expert_mode == false {
            current_knowledge = current_knowledge_truthy.unwrap_or(false)
        }
        /**/
        match condition {
            Condition::AND => {
                if current_knowledge == false || last_result == false {
                    return Some(false);
                }
            }
            Condition::OR => {
                if last_result == false
                    && current_knowledge == false
                    && same_condition == true
                {
                    return Some(false);
                }
            }
            Condition::XOR => {
                if current_knowledge == false && last_result == false || 
                last_result == true && current_knowledge == true {
                    return Some(false);
                }
            }
            _ => {
                return Some(last_result);
            }
        }
        Some(true)
    }












}
