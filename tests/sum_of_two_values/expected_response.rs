use primitypes::{
    problem::{ProblemExecutorResult, TestCaseResult},
    status::Status,
};
use uuid::Uuid;

fn sort_by_id(mut arr: Vec<TestCaseResult>) -> Vec<TestCaseResult> {
    arr.sort_by(|a, b| a.id.cmp(&b.id));
    arr
}
pub fn get_expected_runtime_error() -> ProblemExecutorResult {
    ProblemExecutorResult {
        overall_result: Status::RuntimeError,
        test_cases_results: vec![
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
        ],
        prepare_output: None,
    }
}

pub fn get_expected_partial_runtime_error() -> ProblemExecutorResult {
    ProblemExecutorResult {
        overall_result: Status::RuntimeError,
        test_cases_results: sort_by_id(vec![
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::RuntimeError,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::TimeLimitExceeded,
                id: Uuid::new_v4(),
                output: None,
            },
        ]),
        prepare_output: None,
    }
}

pub fn get_expected_time_limit() -> ProblemExecutorResult {
    ProblemExecutorResult {
        overall_result: Status::TimeLimitExceeded,
        test_cases_results: sort_by_id(vec![
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::TimeLimitExceeded,
                id: Uuid::new_v4(),
                output: None,
            },
        ]),
        prepare_output: None,
    }
}

pub fn get_expected_accepted() -> ProblemExecutorResult {
    ProblemExecutorResult {
        overall_result: Status::Accepted,
        test_cases_results: sort_by_id(vec![
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
            TestCaseResult {
                status: Status::Accepted,
                id: Uuid::new_v4(),
                output: None,
            },
        ]),
        prepare_output: None,
    }
}
