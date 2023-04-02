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
use crate::types::{CodeExecutor, CodeExecutorResult, Language, Problem, Status, Submission};

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
    pub fn execute(&self, submission: Submission, problem: Problem) -> Result<Status> {
        let code_executor = match submission.language {
            Language::Python3 => {
                use crate::languages::python_3;
                python_3::Python3::new(submission.code)?
            }
            _ => todo!(),
        };

        // let mut test_status : Vec<Result<Status>> = vec![];
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
            let out = comp.wait_with_output()?;
            println!("Compilar {out:?}");
        }

        problem.test_cases.par_iter().for_each(|test_case| {
            let start_time = Instant::now();

            match code_executor.execute(test_case) {
                Ok(CodeExecutorResult { err, output }) => {
                    if let Some(err)  = err {
                        // dbg!(&err);
                        mutexed_tests.lock().unwrap().push(err);
                        return;
                    }
                    let status = check_input(test_case, output.unwrap()).expect("F");
                    let end_time = Instant::now();

                    let _elapsed_time = end_time - start_time;
                    // println!("Elapsed time {idx}: {:?}", elapsed_time.as_millis());

                    mutexed_tests.lock().unwrap().push(status);
                }
                Err(v) => {
                    dbg!(&v);
                }
            }
        });

        let bind = mutexed_tests.lock().unwrap();

        let res = bind.iter().collect::<Vec<_>>();
        for ele in res.iter() {
            println!("{ele:?}");
        }
        Ok(Status::Accepted)
    }
}
