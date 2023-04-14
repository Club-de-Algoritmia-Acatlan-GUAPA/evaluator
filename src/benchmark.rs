use anyhow::Result;

use std::io::{Read, Write};
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus, Output, Stdio};
use std::time::{Duration, Instant};
use crate::code_executor::CodeExecutorResult;
use crate::types::{Status, TestCase};
//
//https://github.com/sharkdp/hyperfine/blob/2763b411afe0f035be1a6dcd304e4635d9b2ea47/src/timer/mod.rs

pub fn run_and_meassure(mut command: Command, test_case: &TestCase) -> Result<CodeExecutorResult> {
    let child = command
        .current_dir("./playground")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped());

    let one_sec = Duration::from_secs(1);
    let now = Instant::now();
    // spawn
    let mut child = match child.spawn() {
        Ok(child) => child,
        Err(v) => {
            dbg!(&v);
            return Ok(CodeExecutorResult {
                err: Some(Status::RuntimeError),
                output: Some(Output {
                    status: ExitStatus::from_raw(1),
                    stdout: v.to_string().as_bytes().to_vec(),
                    stderr: vec![],
                }),
            });
        }
    };
    let child_stdin = child.stdin.as_mut().expect("F");

    if child_stdin
        .write_all(test_case.input_case.as_bytes())
        .is_err()
    {
        return Ok(CodeExecutorResult {
            err: Some(Status::RuntimeError),
            output: Some(Output {
                status: ExitStatus::from_raw(1),
                stdout: child
                    .stdout
                    .unwrap()
                    .bytes()
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>(),
                stderr: child
                    .stderr
                    .unwrap()
                    .bytes()
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>(),
            }),
        });
    }

    loop {
        let result = child.try_wait();

        match result {
            Ok(Some(_)) => {
                break;
            }
            Ok(None) => {
                if now.elapsed() > one_sec {
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
            output: Some(Output {
                status,
                stdout: stdout.bytes().filter_map(|x| x.ok()).collect::<Vec<_>>(),
                stderr: vec![],
            }),
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
