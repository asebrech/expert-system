mod data_types;
mod engine;
mod parsing;
mod test;
use colored::Colorize;
use data_types::fact::*;
use dotenv::dotenv;
use engine::solver::{prove, KnowledgeCacheManager, KnowledgeEngine};
use env_logger::Env;
use parsing::parser::parse_lines;
use std::collections::{HashMap, HashSet};
use std::env;

use std::io::{self, BufRead, Write};

use crate::parsing::parser::read_file;

pub fn knowledge_engine_from_lines(lines: Vec<String>) -> KnowledgeEngine {
    let mut data: HashMap<String, Vec<Knowledge>> = HashMap::new();
    let mut search: Vec<char> = Vec::new();
    parse_lines(lines, &mut data, &mut search).unwrap_or_else(|e| {
        println!("Error parsing file: {}", e);
        std::process::exit(1);
    });

    KnowledgeEngine {
        data,
        current_symbol: None,
        search,
    }
}

fn ensure_program_args(args: &Vec<String>) {
    if args.len() < 2 {
        println!("Usage ./expert pathToFile");
        std::process::exit(1);
    }
}

fn launch_resolve(mut ke: KnowledgeEngine) {
    let mut knowledge_cache_manager: KnowledgeCacheManager = KnowledgeCacheManager {
        resolved_data: HashMap::new(),
        previous_line: None,
        resolve_stack: HashSet::new(),
    };
    for element in &ke.search.clone() {
        ke.current_symbol = Some(element.to_string());
        println!("Resolving symbol {}", element);
        println!(
            "{:?} is {}\n",
            element,
            prove(element.to_string(), &mut ke, &mut knowledge_cache_manager)
                .map_or("undetermined".to_string(), |v| v.to_string())
                .magenta()
        );
    }
}

fn merge_knowledge_engines(ke1: KnowledgeEngine, ke2: KnowledgeEngine) -> KnowledgeEngine {
    let mut merged_data = ke1.data.clone();

    for (key, value) in ke2.data {
        merged_data.entry(key).or_default().extend(value);
    }

    let mut merged_search = ke1.search.clone();
    for symbol in ke2.search {
        if !merged_search.contains(&symbol) {
            merged_search.push(symbol);
        }
    }

    KnowledgeEngine {
        data: merged_data,
        current_symbol: None,
        search: merged_search,
    }
}

fn get_user_input() -> std::string::String {
    let mut input = String::new();

    loop {
        println!("Want to add some shit dude ? (y/n)");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.contains("y") || input.contains("n") {
            break;
        }
    }
    input
}

fn knowledge_engine_from_file(file_path: &str) -> KnowledgeEngine {
    let lines = read_file(file_path).unwrap_or_else(|e| {
        println!("Error reading file: {}", e);
        println!("Expert System usage : file");
        std::process::exit(1);
    });
    knowledge_engine_from_lines(lines.clone())
}

fn main() {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let args: Vec<String> = env::args().collect();
    ensure_program_args(&args);
    let file_path = &args[1];
    // println!("{:?}", args);
    let lines = read_file(file_path).unwrap_or_else(|e| {
        println!("Error reading file: {}", e);
        println!("Expert System usage : file");
        std::process::exit(1);
    });

    let ke = knowledge_engine_from_file(file_path);

    launch_resolve(ke);

    match env::var("EXPERT_MODE") {
        Ok(_v) => {}
        Err(_e) => return,
    }

    loop {
        let mut ke = knowledge_engine_from_lines(lines.clone());

        let stdin = io::stdin();
        let mut lines: Vec<String> = Vec::new();

        let input = get_user_input();

        if input.contains("y") {
            println!("Enter your new knowledge : []=>[]\\n ?[]\\n =[]\\n then ctrl+D");

            for line in stdin.lock().lines() {
                match line {
                    Ok(content) => lines.push(content),
                    Err(e) => {
                        eprintln!("Error reading line: {}", e);
                        break;
                    }
                }
            }

            let new_ke = knowledge_engine_from_lines(lines);
            ke = merge_knowledge_engines(ke, new_ke);
        } else {
            break;
        }

        launch_resolve(ke);
    }
}
