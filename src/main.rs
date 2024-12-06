use std::{collections::HashMap, process::exit};
mod data_types;
mod parsing;
use data_types::fact::*;

use parsing::parser::*;

struct KnowledgeEngine<'a> {
    pub data: HashMap<&'a str, Vec<&'a Knowledge<'a>>>, //Need to put vector as we can have several rule for one knowledge
}

fn prove(goal: &str, knowledge: &KnowledgeEngine) {
    //get the goal in the knowledge, if it exists and its fact is set to true, say it
    let existing_knowledge = knowledge.data.get(goal);
    println!("{:?}", existing_knowledge.is_some());
    if existing_knowledge.is_some() {
        //TODO
        //is_knowledge_true(&existing_knowledge.unwrap(), &knowledge);
    } else {
        println!("Unknown goal requested : {}", goal);
    }
}

fn is_knowledge_true(knowledge: &Knowledge, brain: &KnowledgeEngine) -> bool {
    //the knowledge is a fact, then it is true by default
    if knowledge.fact {
        return true;
    }

    //solve the knowledge
    let knowledge_req = &knowledge.requirements;

    //the knowledge has no requirements, hence it should be false ?
    if knowledge_req.len() == 0 {
        return false;
    }

    let first_req = knowledge_req.first().unwrap();

    //only one parameter
    if knowledge_req.len() == 1 {
        //check that the only requirement is true
        return is_knowledge_true(&first_req.knowledge, &brain);
    }
    are_requirements_met(&knowledge_req, &brain)
}

//anything passed to this function must be a vec of at least 2 elements, otherwise it will return false.
fn are_requirements_met(requirements: &Vec<Requirement>, brain: &KnowledgeEngine) -> bool {
    if requirements.len() < 2 {
        return false;
    }
    //init with the first requirement
    let first_req = requirements.first().unwrap();

    //skip first element, then iter on every others
    let requirement_iter = requirements.iter().skip(1);
    //will have problem here with the first one
    let mut last_condition = first_req.condition;
    let mut last_result = match_requirement(first_req, brain, first_req.condition, true, false);
    for requirement in requirement_iter {
        last_result = match_requirement(
            requirement,
            brain,
            requirement.condition,
            last_result,
            requirement.condition == last_condition,
        );
        if !last_result {
            return false;
        }
        last_condition = requirement.condition;
    }

    true
}

//A ^ B ^ C
//last_result is the last result for the current condition
fn match_requirement(
    current: &Requirement,
    brain: &KnowledgeEngine,
    condition: Condition,
    last_result: bool,
    same_condition: bool,
) -> bool {
    let current_knowledge_truthy = is_knowledge_true(&current.knowledge, &brain);

    match condition {
        Condition::AND => {
            if current_knowledge_truthy == false || last_result == false {
                return false;
            }
        }
        Condition::OR => {
            if last_result == true && current_knowledge_truthy == true && same_condition == true {
                return false;
            }
        }
        _ => {}
    }
    true
}

//testing
//knowledge = A + (E + F) = C
//=AB
//?C
fn main() {
    //prove("C".into(), &ke);
    test();
}
