#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        engine::solver::solver::{prove, KnowledgeCacheManager},
        knowledge_engine_from_file,
    };

    #[test]
    fn tests_file_one() {
        let mut engine = knowledge_engine_from_file("tests/test_one.txt");
        let mut knowledge_cache_manager: KnowledgeCacheManager = KnowledgeCacheManager {
            resolved_data: HashMap::new(),
        };
        assert_eq!(
            prove(
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
            resolved_data: HashMap::new(),
        };
        assert_eq!(
            prove(
                engine.search.first().unwrap().to_string(),
                &mut engine,
                &mut knowledge_cache_manager
            ),
            Some(true)
        );
    }
}
