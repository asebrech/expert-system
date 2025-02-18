pub mod solver {

    use core::time;
    use std::{
        collections::HashMap,
        thread::{self},
    };

    use crate::{data_types, Condition, Knowledge, Requirement};

    pub struct KnowledgeEngine {
        pub data: std::collections::HashMap<
            std::string::String,
            std::vec::Vec<data_types::fact::Knowledge>,
        >, //Need to put vector as we can have several rule for one knowledge
        pub current_symbol: Option<String>,
        pub search: Vec<char>,
    }

    pub struct KnowledgeCacheManager {
        pub resolved_data: HashMap<String, Option<bool>>, //keeps track of resolved formulas
    }

    pub fn prove(
        symbol: String,
        engine: &mut KnowledgeEngine,
        knowledge_cache_manager: &mut KnowledgeCacheManager,
    ) -> Option<bool> {
        return get_knowledge_state(
            &symbol,
            engine,
            &"test".to_string(),
            knowledge_cache_manager,
            0,
            false,
        );
    }

    //check if given knowledge is true, false or none (undetermined)
    fn get_knowledge_state(
        symbol: &str,
        engine: &KnowledgeEngine,
        current_calcul: &String,
        knowledge_cache_manager: &mut KnowledgeCacheManager,
        mut depth: usize,
        is_result_symbol: bool,
    ) -> Option<bool> {
        let ten_millis = time::Duration::from_millis(200);

        thread::sleep(ten_millis);
        let knowledge_vec = engine.data.get(symbol);
        //println!("{}Evaluating : {}", "\t".repeat(depth), symbol);
        depth += 1;
        if knowledge_vec.is_none() {
            println!("Sarutax {}", symbol);
            if is_result_symbol {
                return None;
            }
            return Some(false);
        }

        let knowledge_vec = knowledge_vec.unwrap();
        if knowledge_vec.len() == 0 {
            println!("{}No requirement for {}", "\t".repeat(depth), symbol);
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
        //symbol_met.insert(symbol.to_string());

        println!(
            "{}Processing all knowledge of {}, total: {}",
            "\t".repeat(depth),
            symbol,
            knowledge_vec.len()
        );

        for knowledge in knowledge_vec.iter() {
            /*if is_result_symbol && knowledge.result_requirement.is_some() {
                continue;
            }*/
            println!("comparing {} {}", current_calcul, &knowledge.calcul);
            if current_calcul == &knowledge.calcul && is_result_symbol {
                println!("Skipping check for {}", knowledge.symbol);
                if knowledge_vec.len() == 1 {
                    //and knowledge requirement isnt an equal sign, otherwise it is true
                    return get_value_from_result_knowledge(&knowledge, &knowledge.symbol);
                }
                continue;
            }
            println!(
                "{}Checking formula: {} for {}",
                "\t".repeat(depth),
                knowledge.calcul,
                symbol
            );
            let are_req_met: Option<bool>;
            if let Some(temp) = knowledge_cache_manager.resolved_data.get(&knowledge.calcul) {
                are_req_met = *temp;
                println!(
                    "Cached data found for {} => {:?}",
                    knowledge.calcul,
                    temp.map_or("undetermined".to_string(), |v| v.to_string())
                );
            } else {
                are_req_met = process_formula(
                    &knowledge.requirements,
                    engine,
                    current_calcul,
                    knowledge_cache_manager,
                    depth,
                    false,
                );
            }
            //cas not C is true
            if let Some(are_req_met) = are_req_met {
                //si le req est false, et que la knowledge veux que sa n existe pas
                if are_req_met == false && knowledge.not {
                    println!("True 1");
                    knowledge_cache_manager
                        .resolved_data
                        .insert(knowledge.calcul.clone(), Some(true));
                    return Some(true);
                }

                if are_req_met && knowledge.not {
                    println!("{}!{} is true", "\t".repeat(depth), knowledge.symbol);
                } else {
                    println!(
                        "{}{} is {}",
                        "\t".repeat(depth),
                        knowledge.symbol,
                        are_req_met
                    );
                    if are_req_met == false {
                        println!("XXFalse one");
                        knowledge_cache_manager
                            .resolved_data
                            .insert(knowledge.calcul.clone(), Some(false));
                        return Some(false);
                    }
                }
            } else {
                //println!("{}Default none", "\t".repeat(depth));
                knowledge_cache_manager
                    .resolved_data
                    .insert(knowledge.calcul.clone(), None);
                return None;
            }
            println!(
                "{}------------------------------------------------",
                "\t".repeat(depth)
            );
            if let Some(krr) = &knowledge.result_requirement {
                println!("calcul : {}", knowledge.calcul);
                for item in krr.iter() {
                    println!("Item : {}", item.symbol);
                    if let Some(temp) = knowledge_cache_manager.resolved_data.get(&knowledge.calcul)
                    {
                        if item.symbol == symbol {
                            //symbol_met.insert(symbol.to_string());
                            println!("Inserting  {}", symbol);
                            println!(
                                "Cached data found for {} => {:?}",
                                knowledge.calcul,
                                temp.map_or("undetermined".to_string(), |v| v.to_string())
                            );

                            //might possibly want to return here or perform some action
                            continue;
                        }
                    } else {
                        println!("checking {}", symbol);
                        let res2 = process_formula(
                            krr,
                            engine,
                            &knowledge.calcul,
                            knowledge_cache_manager,
                            depth,
                            true,
                        );
                        println!("process res : {:?}", res2);
                        if res2.is_none() {
                            //ask user to clarify
                            println!("Asking user to clarify, he did not, undefined");
                            return None;
                        } else if res2.unwrap() == false {
                            //resolution is false
                            println!("Resolution is false for {}", knowledge.calcul);
                            return Some(false);
                        }
                        knowledge_cache_manager
                            .resolved_data
                            .insert(knowledge.calcul.clone(), Some(true));
                    }
                }
            }
            knowledge_cache_manager
                .resolved_data
                .insert(knowledge.calcul.clone(), Some(true));
        }

        //println!("Symbol met {:?}", symbol_met);
        println!("{}{} is true", "\t".repeat(depth), symbol);
        //solve calcul if it exists, otherwise return true

        Some(true)
    }

    fn get_value_from_result_knowledge(
        knowledge: &Knowledge,
        symbol_to_find: &str,
    ) -> Option<bool> {
        if knowledge.result_requirement.is_none() {
            return None;
        }
        let mut prev: Option<Condition> = None;
        let mut found = false;
        for ele in knowledge.result_requirement.clone().unwrap().iter() {
            if found == true && prev.is_some() {
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
        return None;
    }

    fn process_knowledge_state(
        requirement: &Requirement,
        engine: &KnowledgeEngine,
        current_calcul: &String,
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
        println!("Checking in process knowledge state {}", requirement.symbol);
        // parse it as an answer response and return it
        if res.is_none() {
            if requirement.condition == Condition::AND {
                println!("Req AND {}", !requirement.not);
                return Some(!requirement.not); //true if not is false and false if not is true, M A G I C
            }
            if requirement.condition == Condition::OR {
                return None;
            }
        }
        println!("Req res normal {:?}", res);
        return res;
    }

    fn process_formula(
        requirements: &Vec<Requirement>,
        brain: &KnowledgeEngine,
        current_calcul: &String,
        knowledge_cache_manager: &mut KnowledgeCacheManager,
        depth: usize,
        is_result_symbol: bool,
    ) -> Option<bool> {
        let first_req = requirements.get(0).unwrap();
        if requirements.len() <= 1 {
            return process_knowledge_state(
                &first_req,
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
            &first_req,
            brain,
            current_calcul,
            knowledge_cache_manager,
            depth,
            is_result_symbol,
        );
        let mut rhs = process_knowledge_state(
            &second_req,
            brain,
            current_calcul,
            knowledge_cache_manager,
            depth,
            is_result_symbol,
        );
        if lhs.is_none() || rhs.is_none() {
            return None;
        }
        let mut lhs = compare_boolean(lhs.unwrap(), rhs.unwrap(), first_req.condition);
        if requirements.len() == 2 {
            return Some(lhs);
        }
        let to_iter = requirements.iter().skip(2);
        for item in to_iter {
            rhs = process_knowledge_state(
                &item,
                brain,
                current_calcul,
                knowledge_cache_manager,
                depth,
                is_result_symbol,
            );
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
            _ => rhs,
        };
    }
}
