use std::process::exit;



fn find_subgoals(goal: &str, rules: &Vec<String>) -> Vec<String> {
    let mut subgoals: Vec<String> = vec![];

    let findthis = format!("{}:-", goal);
    for rule in rules {
        if let Some(pos) = rule.find(findthis.as_str()) {
            let subgoal = rule[(pos + findthis.len())..].trim();
            subgoals.push(subgoal.to_string());
        }
    }
    subgoals
}

fn conditional_save(fact: String, facts: &mut Vec<String>, truth_value: Option<bool>) {
    if truth_value == Some(true) {
        if !facts.contains(&fact) {
            facts.push(fact)
        }
    }
}

fn prove(goal: &str, mut depth: usize, rules: &Vec<String>, facts: &mut Vec<String>) -> bool {
    println!("{} PROVE {}", "\t".repeat(depth), goal);
    depth += 1;
    let mut once_true: bool = false;

    if facts.contains(&goal.to_string()) {
        once_true = true;
    } else {
        let subgoals = find_subgoals(goal, &rules);
        let mut connective = "AND".to_string();
        for g in subgoals {
            let mut truth_value: Option<bool> = None;
            let subgoal = g.split(' ');
            let mut cnt = 1;
            for s in subgoal {
                if cnt % 2 == 0 {
                    if !["AND", "OR"].contains(&s) {
                        eprintln!("ERROR: {} is not a connective", s);
                        exit(1)
                    } else {
                        connective = s.to_string();
                    }
                } else {
                    let trust_val = prove(s, depth, rules, facts);
                    conditional_save(s.to_string(), facts, truth_value);
                    if truth_value == Some(true) {
                        truth_value = Some(trust_val);
                    } else if connective == "AND" {
                        truth_value = truth_value.and(Some(trust_val));
                    } else if connective == "OR" {
                        truth_value = truth_value.or(Some(trust_val));
                    }
                }
                cnt += 1;
            }
            if truth_value == Some(true) {
                once_true = true;
            }
        }
    }
    println!("{}{} is {}", "\t".repeat(depth), goal, once_true);
    once_true
}

fn start() {
    let rules: Vec<String> = vec![
        "graduate:- has_theory AND has_database AND has_advprog AND has_english".to_string(),
        "has_theory:- has_datastruct OR has_automata".to_string(),
        "has_database:- cs450 OR cs451".to_string(),
        "has_datastruct:- cs460 AND cs461".to_string(),
        "has_automata:- cs470 AND cs471".to_string(),
        "has_advprog:- cs350 OR cs351 OR cs352".to_string(),
        "has_english:- eng101 OR eng102 OR eng103".to_string(),
    ];
    let mut facts: Vec<String> = vec![
        "eng101".to_string(),
        "cs450".to_string(),
        "cs491".to_string(),
        "cs352".to_string(),
        "cs460".to_string(),
    ];
	test();

    prove("graduate", 0, &rules, &mut facts);
}
