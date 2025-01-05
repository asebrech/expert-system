// use std::{collections::HashMap, process::exit};
mod data_types;
mod engine;
mod parsing;
mod test;

use std::collections::HashMap;

use data_types::fact::*;
use dotenv::dotenv;
use env_logger::Env;
use parsing::parser::parse_lines;

use crate::parsing::parser::read_file;

// use engine::solver::solver::init;
// use parsing::parser::*;
//
// struct KnowledgeEngine<'a> {
//     pub data: HashMap<&'a str, Vec<&'a Knowledge<'a>>>, //Need to put vector as we can have several rule for one knowledge
// }

//A ^ B ^ C
//last_result is the last result for the current condition

//testing
//knowledge = A + (E + F) = C
//A => B
//Y => B
//=AB
//?C

//?ABC
// while y a des symbol A, B, C
//get un symbol A
//dit si il existe
//

/*
    --verbose print chaques step
    =BE
    B + C => A
    E => C
    A => !C
    B => !C
    !C => W

    Format print :

    Solving A
    Rule : B + C => A
    ----Solving B
        B is a fact
        B is true
    AND
    ----Solving C, C has 4 rules, if one of them is false
         or undetermined
        , it is undetermined
        if all rules are false, it is false
        if all rules are true, it is true
        if there are mixed result, it is undetermined
        1. Rule : E => C
        ----Solving E
            E is a fact
            E is true
        C is true

        2. Rule : Y => C
        ----Solving Y
            Y is a fact
            Y is true
        C is true

        3. Rule : A => !C
        ----Solving A
            Error: A has a cross dependance with C
            A is undetermined
        C is undetermined

        4. Rule : Y => !C
        ----Solving Y
            Y is false
        C is false
    A is undetermined
*/

fn main() {
    dotenv().ok();
    // to remove debugging change to default_filter_or("info") or add RUST_LOG=info to .env
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let file_path = "resources/input.txt";

    let lines = read_file(file_path).unwrap_or_else(|e| {
        println!("Error reading file: {}", e);
        std::process::exit(1);
    });

    let mut data = HashMap::new();
    let mut search = Vec::new();

    parse_lines(lines, &mut data, &mut search).unwrap_or_else(|e| {
        println!("Error parsing file: {}", e);
        std::process::exit(1);
    });

    println!("Facts to resolve : {:?}", search);
    println!("{:?}", data);

    /*let mut ke: KnowledgeEngine = KnowledgeEngine {
            data: HashMap::new()
        };
        let symbol_a: &str = "A".into();
        let knowledge_a: Knowledge = Knowledge {
            symbol: symbol_a,
            fact: true,
            requirements: Vec::new()
        };

        let symbol_b: &str = "B".into();
        let knowledge_b: Knowledge = Knowledge {
            symbol: symbol_b,
            fact: true,
            requirements: Vec::new()
        };
        let vecB = Vec::new();
        vecB.push(&knowledge_b);
        ke.data.insert(symbol_b, vecB);

        let goal_one = Requirement {
            knowledge: &knowledge_a,
            condition: Condition::AND,
            should_exist: true,
        };

        let goal_two = Requirement {
            knowledge: &knowledge_b,
            condition: Condition::END,
            should_exist: true,
        };

        ke.data.insert(symbol_a, &knowledge_a);

        let symbol_c: &str = "C".into();
        let mut knowledge_c: Knowledge = Knowledge {
            symbol: symbol_c,
            fact: false,
            requirements: Vec::new()
        };

        knowledge_c.requirements.push(goal_one);
        knowledge_c.requirements.push(goal_two);

        ke.data.insert(symbol_c, &knowledge_c);

        prove("C".into(),&ke);
        test();

    */
}
