pub mod solver {

    use std::collections::{HashMap, HashSet};

    use crate::{data_types, Condition, Requirement};

    pub struct KnowledgeEngine {
        pub data:  std::collections::HashMap<std::string::String, std::vec::Vec<data_types::fact::Knowledge>>, //Need to put vector as we can have several rule for one knowledge
        pub current_symbol: Option<String>,
    }

    pub struct KnowledgeCacheManager {
        pub resolved_data: HashMap<String, Option<bool>> //keeps track of resolved formulas
    }

    pub fn prove(symbol: String, engine: &mut KnowledgeEngine, knowledge_cache_manager: &mut KnowledgeCacheManager) -> Option<bool> {
        let mut symbol_met: HashSet<String> = HashSet::new();
        return get_knowledge_state(&symbol, engine, &mut symbol_met, knowledge_cache_manager, 0);
    }

    //check if given knowledge is true, false or none (undetermined)
    //Some(true) => true
    //Some(false) => false
    //None => undetermined
    fn get_knowledge_state(
        symbol: &str,
        engine: &KnowledgeEngine,
        symbol_met: &mut HashSet<String>,
        knowledge_cache_manager: &mut KnowledgeCacheManager,
        mut depth: usize,
    ) -> Option<bool> {
        let knowledge_vec = engine.data.get(symbol);
        //println!("{}Evaluating : {}", "\t".repeat(depth), symbol);
        if symbol_met.contains(&symbol.to_string()) {
            println!("{}Recursion found for symbol: {}", "\t".repeat(depth), symbol);
            return None;
        }
        depth += 1;
        if knowledge_vec.is_none() {
			println!("Sarutax {}", symbol);
            return Some(false);
        }

        let knowledge_vec = knowledge_vec.unwrap();
        if knowledge_vec.len() == 0 {
            println!("{}No requirement for {}",  "\t".repeat(depth), symbol);
            return Some(false);
        }

        //if ke_vec is a fact, it is stored up front
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
        symbol_met.insert(symbol.to_string());

        println!("{}Processing all knowledge of {}, total: {}", "\t".repeat(depth), symbol, knowledge_vec.len());
        for knowledge in knowledge_vec.iter() {
            println!("{}Checking formula: {} for {}", "\t".repeat(depth), knowledge.calcul, symbol);
            let are_req_met: Option<bool>;
            if let Some(temp) = knowledge_cache_manager.resolved_data.get(&knowledge.calcul) {
                are_req_met = *temp;
                println!("Cached data found for {} => {:?}", knowledge.calcul, temp.map_or("undetermined".to_string(), |v| v.to_string()));
            } else {
                //are_req_met = are_requirements_met(&knowledge.requirements, engine, symbol_met, knowledge_cache_manager, depth);
				are_req_met = process_formula(&knowledge.requirements, engine, symbol_met, knowledge_cache_manager, depth);
			}
            //cas not C is true
            if let Some(are_req_met) = are_req_met {
                //si le req est false, et que la knowledge veux que sa n existe pas
                if are_req_met == false && knowledge.not {
                    println!("True 1");
                    knowledge_cache_manager.resolved_data.insert(knowledge.calcul.clone(), Some(true));
                    symbol_met.remove(symbol);
                    return Some(true);
                }

                if are_req_met && knowledge.not {
                    println!("{}!{} is true", "\t".repeat(depth), knowledge.symbol);
                } else {
                    println!("{}{} is {}","\t".repeat(depth),  knowledge.symbol, are_req_met);
                    if are_req_met == false {
                        println!("XXFalse one");
                        knowledge_cache_manager.resolved_data.insert(knowledge.calcul.clone(), Some(false));
                        symbol_met.remove(symbol);
                        return Some(false);
                    }
                }
            } else {
                //println!("{}Default none", "\t".repeat(depth));
                knowledge_cache_manager.resolved_data.insert(knowledge.calcul.clone(), None);
                return None;
            }
            println!("{}------------------------------------------------", "\t".repeat(depth));
        }
        println!("Symbol met {:?}", symbol_met);
        Some(true)
    }

	fn process_formula(
        requirements: &Vec<Requirement>,
        brain: &KnowledgeEngine,
        symbol_met: &mut HashSet<String>,
        knowledge_cache_manager: &mut KnowledgeCacheManager,
        depth: usize,
    ) -> Option<bool> {
		let first_req = requirements.get(0).unwrap();
		if requirements.len() <= 1 {
			return get_knowledge_state(&first_req.symbol, brain, symbol_met, knowledge_cache_manager, depth);
		}
		let second_req = requirements.get(1).unwrap();
		let mut previous = second_req;
		let lhs = get_knowledge_state(&first_req.symbol, brain, symbol_met, knowledge_cache_manager, depth);
		let mut rhs = get_knowledge_state(&second_req.symbol, brain, symbol_met, knowledge_cache_manager, depth);
		if lhs.is_none() || rhs.is_none() {
			return None;
		}
		let mut lhs = compare_boolean(lhs.unwrap(), rhs.unwrap(), first_req.condition);
		if requirements.len() == 2 {
			return Some(lhs);
		}
		let to_iter = requirements.iter().skip(2);
		for item in to_iter {
			
			rhs = get_knowledge_state(&item.symbol, brain, symbol_met, knowledge_cache_manager, depth);
			if rhs.is_none() {
				return None;
			}
			lhs = compare_boolean(lhs, rhs.unwrap(), previous.condition);
			previous = item;
		}

        Some(lhs)
    }

	fn compare_boolean(lhs: bool, rhs: bool, condition: Condition) -> bool {
        return match condition {
            Condition::AND => lhs && rhs,
            Condition::OR => lhs || rhs,
            Condition::XOR => lhs != rhs,
            _ => rhs
        }
	}
}
