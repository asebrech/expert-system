#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        engine::solver::solver::{prove, KnowledgeCacheManager},
        knowledge_engine_from_file,
    };

    fn test_file(file_path: &str, expected_responses: Vec<bool>) {
        let mut ke = knowledge_engine_from_file(file_path);

        let mut knowledge_cache_manager: KnowledgeCacheManager = KnowledgeCacheManager {
            resolved_data: HashMap::new(),
        };

        let mut actual_responses = Vec::new();

        for element in &ke.search.clone() {
            ke.current_symbol = Some(element.to_string());
            let response = prove(element.to_string(), &mut ke, &mut knowledge_cache_manager);
            actual_responses.push(response.unwrap_or(false));
        }

        assert_eq!(actual_responses, expected_responses);
    }

    #[test]
    fn and_condition_0() {
        let file_path = "tests/subject/andCondition0.txt";
        let expected_responses: [bool; 4] = [true, true, true, true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn and_condition_1() {
        let file_path = "tests/subject/andCondition1.txt";
        let expected_responses: [bool; 4] = [true, true, false, true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn or_condition_0() {
        let file_path = "tests/subject/orCondition0.txt";
        let expected_responses: [bool; 1] = [false];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn or_condition_1() {
        let file_path = "tests/subject/orCondition1.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn or_condition_2() {
        let file_path = "tests/subject/orCondition2.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn or_condition_3() {
        let file_path = "tests/subject/orCondition3.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn xor_condition_0() {
        let file_path = "tests/subject/xorCondition0.txt";
        let expected_responses: [bool; 1] = [false];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn xor_condition_1() {
        let file_path = "tests/subject/xorCondition1.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn xor_condition_2() {
        let file_path = "tests/subject/xorCondition2.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn xor_condition_3() {
        let file_path = "tests/subject/xorCondition3.txt";
        let expected_responses: [bool; 1] = [false];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn same_conclusion_0() {
        let file_path = "tests/subject/sameConclusion0.txt";
        let expected_responses: [bool; 1] = [false];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn same_conclusion_1() {
        let file_path = "tests/subject/sameConclusion1.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn same_conclusion_2() {
        let file_path = "tests/subject/sameConclusion2.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn same_conclusion_3() {
        let file_path = "tests/subject/sameConclusion3.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn parentheses_0() {
        let file_path = "tests/subject/parentheses0.txt";
        let expected_responses: [bool; 1] = [false];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn parentheses_1() {
        let file_path = "tests/subject/parentheses1.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn parentheses_2() {
        let file_path = "tests/subject/parentheses2.txt";
        let expected_responses: [bool; 1] = [false];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn parentheses_3() {
        let file_path = "tests/subject/parentheses3.txt";
        let expected_responses: [bool; 1] = [false];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn parentheses_4() {
        let file_path = "tests/subject/parentheses4.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn parentheses_5() {
        let file_path = "tests/subject/parentheses5.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn parentheses_6() {
        let file_path = "tests/subject/parentheses6.txt";
        let expected_responses: [bool; 1] = [false];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn parentheses_7() {
        let file_path = "tests/subject/parentheses7.txt";
        let expected_responses: [bool; 1] = [false];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn parentheses_8() {
        let file_path = "tests/subject/parentheses8.txt";
        let expected_responses: [bool; 1] = [false];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn parentheses_9() {
        let file_path = "tests/subject/parentheses9.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }

    #[test]
    fn parentheses_10() {
        let file_path = "tests/subject/parentheses10.txt";
        let expected_responses: [bool; 1] = [true];
        test_file(file_path, expected_responses.to_vec());
    }
}
