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

use crate::parsing::parser::read_file;

pub fn knowledge_engine_from_file(file_path: &str) -> KnowledgeEngine {
    let lines = read_file(file_path).unwrap_or_else(|e| {
        println!("Error reading file: {}", e);
        println!("Expert System usage : file");
        std::process::exit(1);
    });

    let mut data = HashMap::new();
    let mut search = Vec::new();

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

fn main() {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let args: Vec<String> = env::args().collect();
    ensure_program_args(&args);
	let mut file_path = &args[1];
    println!("{:?}", args);
    let mut ke = knowledge_engine_from_file(&file_path);

    let mut knowledge_cache_manager: KnowledgeCacheManager = KnowledgeCacheManager {
        resolved_data: HashMap::new(),
        previous_line: None,
        resolve_stack: HashSet::new()
    };
    for element in &ke.search.clone() {
        ke.current_symbol = Some(element.to_string());
        println!("Resolving symbol {}\n", element);
        println!(
            "{:?} is {}\n",
            element,
            prove(element.to_string(), &mut ke, &mut knowledge_cache_manager)
                .map_or("undetermined".to_string(), |v| v.to_string())
                .magenta()
        );
    }
}
