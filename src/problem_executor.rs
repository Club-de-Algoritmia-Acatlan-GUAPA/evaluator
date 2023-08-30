use anyhow::Result;
use rayon::prelude::*;

use std::sync::{Arc, Mutex};

use primitypes::contest::{Language, Submission};

use crate::{
    code_executor::{CodeExecutor, CodeExecutorImpl, CodeExecutorResult},
    types::{Problem, ProblemExecutorResult, Status, TestCaseResult, STATUS_PRECEDENCE},
    validator::Validator,
};

#[derive(Debug, Eq, PartialEq)]

pub struct ProblemExecutor;
impl Default for ProblemExecutor {
    fn default() -> Self {
        ProblemExecutor::new()
    }
}

impl ProblemExecutor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(
        &self,
        submission: &Submission,
        problem: Problem,
    ) -> Result<ProblemExecutorResult> {
        let mut executor: Box<dyn CodeExecutorImpl> = get_executor(&submission.language);

        executor.code(submission.code.to_string());
        executor.set_id(submission.id.as_u128());
        executor.prepare_code_env()?;

        let mut validator = Validator::new(problem.validation_type);

        if let Some(checker) = problem.checker.as_ref() {
            validator.set_checker(&checker.checker);
        }

        validator.prepare_validator()?;

        //let validator : Validator = validator.into();

        let mutexed_tests = Arc::new(Mutex::new(Vec::new()));

        problem.test_cases.par_iter().for_each(|test_case| {
            match executor.execute(test_case.input_case.clone(), vec![]) {
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

// "optimization" maybe I'm not sure, try to not use Box
fn get_executor(language: &Language) -> Box<dyn CodeExecutorImpl> {
    match language {
        Language::Python3 => {
            use crate::languages::python_3;
            Box::new(CodeExecutor::<python_3::Python3>::new())
        }
        Language::Cpp11 => {
            use crate::languages::cpp;
            Box::new(CodeExecutor::<cpp::Cpp11>::new())
        }
        Language::Cpp17 => {
            use crate::languages::cpp;
            Box::new(CodeExecutor::<cpp::Cpp17>::new())
        }
        _ => todo!(),
    }
}
