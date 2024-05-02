use async_trait::async_trait;
use evaluator::{store::ProblemStore, types::TestCaseError};
use primitypes::problem::{Problem, ProblemID, TestCaseConfig, TestCaseInfo};

#[derive(Debug)]
pub struct FakeStore {
    problem: Problem,
    test_case_config: TestCaseConfig,
}

impl FakeStore {
    pub fn new(problem: Problem, test_case_config: TestCaseConfig) -> Self {
        Self {
            problem,
            test_case_config,
        }
    }
}

#[async_trait]
impl ProblemStore for FakeStore {
    type Error = TestCaseError;

    fn load_testcase(&self, test_case: &TestCaseInfo) -> Result<(), Self::Error> {
        Ok(())
    }

    fn get_problem_by_id(&self, _problem_id: &ProblemID) -> &Problem {
        &self.problem
    }

    async fn get_test_case_config(&self, _id: &ProblemID) -> Result<TestCaseConfig, Self::Error> {
        Ok(self.test_case_config.clone())
    }

    fn get_full_path_test_case(&self, test_case: &TestCaseInfo) -> TestCaseInfo {
        let problem_id = &test_case.problem_id;
        let stdout_path = test_case.stdout_path.as_ref().unwrap();
        TestCaseInfo {
            stdin_path: format!(
                "./tests/sum_of_two_values/stdio/{}/{}",
                problem_id, test_case.stdin_path
            ),
            stdout_path: Some(format!(
                "./tests/sum_of_two_values/stdio/{}/{}",
                problem_id,
                stdout_path.clone()
            )),
            id: test_case.id,
            problem_id: problem_id.clone()
        }
    }
}
