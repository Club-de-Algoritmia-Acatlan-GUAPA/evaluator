use anyhow::Result;
use rayon::prelude::*;


use std::sync::{Arc, Mutex};

use crate::{
    code_executor::{CodeExecutor, CodeExecutorResult},
    languages::{cpp, python_3},
    types::{
        Language, Problem, ProblemExecutorResult, Status, Submission, TestCaseResult,
        STATUS_PRECEDENCE,
    },
    
    validator::Validator,
    match_lang
};

#[derive(Debug, Eq, PartialEq)]

pub struct ProblemExecutor;
impl Default for ProblemExecutor {
    fn default() -> Self {
        ProblemExecutor::new()
    }
}
pub enum TrickyEnum {
    Cpp(CodeExecutor<cpp::Cpp>),
    Python3(CodeExecutor<python_3::Python3>),
}

impl ProblemExecutor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn execute(
        &self,
        submission: Submission,
        problem: Problem,
    ) -> Result<ProblemExecutorResult> {
        match_lang! {
            Executor: let mut executor,
            Lang : submission.language,
            executor.code(submission.code);
            executor.set_id(submission.id);
            executor.prepare_code_env()?;

            let mut validator = Validator::new(problem.validation_type);

            if let Some(checker) = problem.checker.as_ref() {
                validator.set_checker(&checker.checker);
            }

            validator.prepare_validator()?;

            let mutexed_tests = Arc::new(Mutex::new(Vec::new()));

            problem
                .test_cases
                .par_iter()
                .for_each(|test_case| match executor.execute(test_case) {
                    Ok(CodeExecutorResult { err, output }) => {
                        if let Some(err) = err {
                            mutexed_tests.lock().unwrap().push(TestCaseResult {
                                status: err,
                                id: test_case.id,
                                output,
                            });
                            return;
                        }

                        let status = validator
                            .check_input(test_case, &output.unwrap())
                            .expect("F");

                        mutexed_tests.lock().unwrap().push(status);
                    }
                    Err(v) => {
                        dbg!(&v);
                    }
                });

            let bind = mutexed_tests.lock().unwrap();

            let overall_result_testcase = bind.iter().max_by_key(|testcase_result| {
                STATUS_PRECEDENCE
                    .get(&testcase_result.status)
                    .unwrap_or(&10)
            });

            let overall_result = if let Some(result) = overall_result_testcase {
                result.status.to_owned()
            } else {
                Status::UnknownError("Status can't be infered".to_string())
            };

            Ok(ProblemExecutorResult {
                overall_result,
                test_cases_results: bind.to_owned(),
            })
        }
    }
}
