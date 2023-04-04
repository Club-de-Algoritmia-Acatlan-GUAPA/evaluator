use evaluator::types::{
    ProblemExecutorResult,
    Status::{Accepted, RuntimeError, TimeLimitExceeded},
    TestCaseResult,
};

fn sort_by_id(mut arr: Vec<TestCaseResult>) -> Vec<TestCaseResult> {
    arr.sort_by(|a, b| a.id.cmp(&b.id));
    arr
}
pub fn get_expected_runtime_error() -> ProblemExecutorResult {
    ProblemExecutorResult {
        overall_result: RuntimeError,
        test_cases_results: sort_by_id(vec![
            TestCaseResult {
                status: RuntimeError,
                id: 12,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 3,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 18,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 6,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 1,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 0,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 9,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 2,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 19,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 13,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 5,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 4,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 10,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 15,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 7,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 11,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 14,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 20,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 21,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 22,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 17,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 8,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 16,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 23,
                output: None,
            },
        ]),
    }
}

pub fn get_expected_partial_runtime_error() -> ProblemExecutorResult {
    ProblemExecutorResult {
        overall_result: RuntimeError,
        test_cases_results: sort_by_id(vec![
            TestCaseResult {
                status: RuntimeError,
                id: 3,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 4,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 12,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 6,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 13,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 9,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 14,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 2,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 15,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 1,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 0,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 18,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 5,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 16,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 7,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 19,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 17,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 10,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 11,
                output: None,
            },
            TestCaseResult {
                status: RuntimeError,
                id: 8,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 21,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 22,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 23,
                output: None,
            },
            TestCaseResult {
                status: TimeLimitExceeded,
                id: 20,
                output: None,
            },
        ]),
    }
}

pub fn get_expected_time_limit() -> ProblemExecutorResult {
    ProblemExecutorResult {
        overall_result: TimeLimitExceeded,
        test_cases_results: sort_by_id(vec![
            TestCaseResult {
                status: Accepted,
                id: 2,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 12,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 3,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 1,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 0,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 7,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 6,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 13,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 9,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 18,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 4,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 5,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 14,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 19,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 8,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 16,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 17,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 15,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 21,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 10,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 11,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 22,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 23,
                output: None,
            },
            TestCaseResult {
                status: TimeLimitExceeded,
                id: 20,
                output: None,
            },
        ]),
    }
}

pub fn get_expected_accepted() -> ProblemExecutorResult {
    ProblemExecutorResult {
        overall_result: Accepted,
        test_cases_results: sort_by_id(vec![
            TestCaseResult {
                status: Accepted,
                id: 18,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 1,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 3,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 0,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 13,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 12,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 6,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 19,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 4,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 2,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 15,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 14,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 17,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 16,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 5,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 7,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 8,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 21,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 22,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 23,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 10,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 20,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 9,
                output: None,
            },
            TestCaseResult {
                status: Accepted,
                id: 11,
                output: None,
            },
        ]),
    }
}
