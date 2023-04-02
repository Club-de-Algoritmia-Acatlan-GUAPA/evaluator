use anyhow::Result;
use std::fs;
use std::io::{Read, Write};

use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

use crate::types::{CodeExecutor, CodeExecutorResult, Status, TestCase};

pub struct Python3 {
    pub file_ending: String,
    pub file_for_execution: String, // TODO : convert to OsStr
}
impl Python3 {
    pub fn new(content: String) -> Result<Self> {
        let mut file = fs::File::create("./playground/foo.py")?;
        file.write_all(content.as_bytes())?;
        let file_name = String::from("foo.py");
        Ok(Self {
            file_ending: "py".to_string(),
            file_for_execution: file_name,
        })
    }
}
impl CodeExecutor for Python3 {
    fn execute(&self, testcase: &TestCase) -> Result<CodeExecutorResult> {
        let mut child = Command::new("python3")
            .current_dir("./playground")
            .arg(&self.file_for_execution)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Se murio");

        let child_stdin = child.stdin.as_mut().expect("F");
        if child_stdin
            .write_all(testcase.input_case.as_bytes())
            .is_err()
        {
            return Ok(CodeExecutorResult {
                err: Some(Status::RuntimeError),
                output: None,
            });
        }

        let one_sec = Duration::from_secs(1);
        let now = Instant::now();
        loop {
            let result = child.try_wait();

            match result {
                Ok(Some(_)) => {
                    break;
                }
                Ok(None) => {
                    if now.elapsed() > one_sec {
                        child.kill().unwrap();
                        child.kill()?;
                        return Ok(CodeExecutorResult {
                            err: Some(Status::TimeLimitExceeded),
                            output: None,
                        });
                    }
                }
                Err(e) => {
                    panic!("Error: {e}");
                }
            }
        }

        let status = child.wait().unwrap();
        let stdout = child.stdout.unwrap();
        let mut stderr = vec![];

        if !status.success() {
            return Ok(CodeExecutorResult {
                err: Some(Status::RuntimeError),
                output: None,
            });
        }
        if let Some(v) = child.stderr {
            stderr = v.bytes().filter_map(|x| x.ok()).collect::<Vec<_>>();
        }

        Ok(CodeExecutorResult {
            err: None,
            output: Some(Output {
                status,
                stdout: stdout.bytes().filter_map(|x| x.ok()).collect::<Vec<_>>(),
                stderr,
            }),
        })
    }
}
