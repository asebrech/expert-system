
pub mod solver {
    use std::{collections::HashMap, iter::Map};

    use crate::{Condition, Knowledge, Requirement};

    struct KnowledgeEngine<'a> {
        pub data: HashMap<&'a str, Vec<Knowledge<'a>>>, //Need to put vector as we can have several rule for one knowledge
    }

//check if given knowledge is true, false or none (undetermined)
//Some(true) => true
//Some(false) => false
//None => undetermined
fn get_knowledge_state(symbol: &str, engine: &KnowledgeEngine, mut depth: i32) -> Option<bool> {
    let knowledge_vec = engine.data.get(symbol);
    println!("Evaluating : {}", symbol);
    depth+=1;
    if knowledge_vec.is_none() {
        return None;
    }

    let ke_vec = knowledge_vec.unwrap();
    if ke_vec.len() == 0 {
        println!("No requirement for {}", symbol);
        return None;
    }

    //if ke_vec is a fact, it is stored up front
    let first = ke_vec.first().unwrap();
    if first.fact {
        println!("{}{} is a fact that is {}", if first.not {"!"} else {""}, symbol, first.fact && !first.not);
        return Some(first.fact && !first.not)
    }

    println!("Checking knowledge for {}", symbol);
    for knowledge in ke_vec.iter() {
        let are_req_met = are_requirements_met(&knowledge.requirements, &engine, depth);
        println!("Checking Knowledge {}", knowledge.symbol);
        //cas not C is true
        if let Some(are_req_met) = are_req_met {
            //si le req est false, et que la knowledge veux que sa n existe pas
            if are_req_met == false && knowledge.not {
                println!("True 1");
                return Some(true)
            }




            if are_req_met && knowledge.not {
                println!("!{} is true", knowledge.symbol);
            } else {
                println!("{} is {}", knowledge.symbol, are_req_met);
                if are_req_met == false {
                    println!("False one");
                    return Some(false);
                }
            }
        } else {
            println!("Default none");
            return None;
        }
    }
    Some(true)
}

fn are_requirements_met(requirements: &Vec<Requirement>, brain: &KnowledgeEngine, mut depth: i32) -> Option<bool> {
    //init with the first requirement
    let first_req = requirements.first().unwrap();

    //skip first element, then iter on every others
    let requirement_iter = requirements.iter().skip(1);
    //will have problem here with the first one
    let mut last_condition = first_req.condition;
    let mut last_result = match_requirement(first_req, brain, first_req.condition, true, false, depth);
    if last_result.is_none() || last_result.unwrap() == false {
        println!("First requirement arent met");
        return None;
    }
    for requirement in requirement_iter {
        last_result = match_requirement(
            requirement,
            brain,
            requirement.condition,
            last_result.unwrap(),
            requirement.condition == last_condition,
            depth + 1
        );
        if last_result.is_none() || last_result.unwrap() == false {
            println!("Requirement failed to be satisfied");
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
    mut depth: i32
) -> Option<bool> {
    let current_knowledge_truthy = get_knowledge_state(&current.symbol, &brain, depth);

    if current_knowledge_truthy.is_none() {
        println!("Undetermined symbol {} ", current.symbol);
    }
    match condition {
        Condition::AND => {
            if current_knowledge_truthy.unwrap() == false || last_result == false {
                return Some(false);
            }
        }
        Condition::OR => {
            if last_result == true && current_knowledge_truthy.unwrap() == true && same_condition == true {
                return Some(false);
            }
        }
        _ => {}
    }
    Some(true)
}

pub fn init() {
    let mut knowledge_engine = KnowledgeEngine {
        data: HashMap::new()
    };

    //not Y is a fact
    //=BD!Y
    //B + !C => A
    //D => !C
    //Y => C
    //A
    let mut ka = Knowledge {
        symbol: "A",
        fact: false,
        not: false,
        requirements: Vec::new()
    };

    let kb = Knowledge {
        symbol: "B",
        fact: true,
        not: false,
        requirements: Vec::new()
    };

    let mut kcn = Knowledge {
        symbol: "C",
        fact: false,
        not: true,
        requirements: Vec::new()
    };

    let kd= Knowledge {
        symbol: "D",
        fact: true,
        not: false,
        requirements: Vec::new()
    };

    let ky= Knowledge {
        symbol: "Y",
        fact: true,
        not: true,
        requirements: Vec::new()
    };
//a c deja fait manque b d y
    knowledge_engine.data.insert("B", vec!(kb));
    knowledge_engine.data.insert("D", vec!(kd));
    knowledge_engine.data.insert("Y", vec!(ky));


    //=BD
    //B + !C => A
    //D => !C
    //Y => C
    //A

    //B + !C => A
    ka.requirements.push(Requirement {
        condition: crate::Condition::AND,
        symbol: "B",
        not: false,
    });

    ka.requirements.push(Requirement {
        condition: crate::Condition::END,
        symbol: "C",
        not: true,
    });

    knowledge_engine.data.insert("A",  vec![ka]);

    //D => !C
    kcn.requirements.push(Requirement {
        condition: crate::Condition::END,
        symbol: "D",
        not: false
    });

    let mut kc = Knowledge {
        symbol: "C",
        fact: false,
        not: false,
        requirements: Vec::new()
    };

    //Y => C
    kc.requirements.push(Requirement {
        condition: crate::Condition::END,
        symbol: "Y",
        not: false
    });
    knowledge_engine.data.insert("C", vec![kcn, kc]);


    //=BD!Y
    //B + !C => A
    //D => !C
    //Y => C
    //A
    println!("\nA should be undetermined (true) : {}", get_knowledge_state("A", &knowledge_engine, 0).is_none());
    //println!("\nB should be true : {}", get_knowledge_state("B", &knowledge_engine, 0).unwrap());
    //println!("\nD should be true : {}", get_knowledge_state("D", &knowledge_engine, 0).unwrap());
    //println!("\nC should be true : {}", get_knowledge_state("C", &knowledge_engine, 0).unwrap());
}

}


