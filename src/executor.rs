use anyhow::Result;
use rayon::prelude::*;

use std::{
    fs,
    io::Write,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::checker::check_input;
use crate::types::{
    Language, Problem, ProblemExecutorResult, Status, Submission,
    TestCaseResult, STATUS_PRECEDENCE,
};
use crate::code_executor::{ CodeExecutor, CodeExecutorResult };


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
        submission: Submission,
        problem: Problem,
    ) -> Result<ProblemExecutorResult> {
        let mut code_executor = match submission.language {
            Language::Python3 => {
                use crate::languages::python_3;
                CodeExecutor::<python_3::Python3>::new(submission.id)
            }
            _ => todo!(),
        };

        code_executor.code(submission.code);

        code_executor.prepare_code_env()?;

        let mutexed_tests = Arc::new(Mutex::new(Vec::new()));

        if let Some(checker) = problem.checker {
            let mut file = fs::File::create("./playground/checker.cpp")?;
            file.write_all(checker.checker.as_bytes())?;
            let comp = Command::new("g++-12")
                .current_dir("./playground")
                .arg("checker.cpp")
                .arg("-o")
                .arg("checker")
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .stdin(Stdio::piped())
                .spawn()?;
            let _ = comp.wait_with_output()?; // output from compiling the checker
        }

        problem.test_cases.par_iter().for_each(|test_case| {
            let start_time = Instant::now();

            match code_executor.execute(test_case) {
                Ok(CodeExecutorResult { err, output }) => {
                    if let Some(err) = err {
                        mutexed_tests.lock().unwrap().push(TestCaseResult {
                            status: err,
                            id: test_case.id,
                            output,
                        });
                        return;
                    }
                    let status = check_input(test_case, output.unwrap()).expect("F");
                    let end_time = Instant::now();

                    let _elapsed_time = end_time - start_time;
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
