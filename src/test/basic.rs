#[cfg(test)]
mod test {
    use std::{collections::{HashMap, HashSet}, vec};

    use crate::{
        engine::neo_solver::{neo_prove, KnowledgeCacheManager},
        knowledge_engine_from_file,
    };

    #[test]
    fn tests_file_one() {
        let mut engine = knowledge_engine_from_file("tests/test_one.txt");
        let mut knowledge_cache_manager: KnowledgeCacheManager = KnowledgeCacheManager {
            previous_line: None,
            resolved_data: HashMap::new(),
            resolve_stack: HashSet::new(),
            result_knowledge_stack: HashSet::new(),
            rhs_symbol_map: vec![]
        };
        assert_eq!(
            neo_prove(
                engine.search.first().unwrap().to_string(),
                &mut engine,
                &mut knowledge_cache_manager
            ),
            Some(true)
        );
    }
    #[test]
    fn tests_file_two() {
        let mut engine = knowledge_engine_from_file("tests/test_two.txt");
        let mut knowledge_cache_manager: KnowledgeCacheManager = KnowledgeCacheManager {
            previous_line: None,
            resolved_data: HashMap::new(),
            resolve_stack: HashSet::new(),
            result_knowledge_stack: HashSet::new(),
            rhs_symbol_map: vec![]
        };
        assert_eq!(
            neo_prove(
                engine.search.first().unwrap().to_string(),
                &mut engine,
                &mut knowledge_cache_manager
            ),
            Some(true)
        );
    }
}
