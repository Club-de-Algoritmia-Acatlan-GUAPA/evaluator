use std::{
    process::{Command, Output},
    time::Instant,
};

use anyhow::Result;
use primitypes::status::Status;
use tracing::debug;

use crate::{
    code_executor::{CodeExecutorError, CodeExecutorInternalError, CodeExecutorResult},
    command::JailedCommand,
};
//https://github.com/sharkdp/hyperfine/blob/2763b411afe0f035be1a6dcd304e4635d9b2ea47/src/timer/mod.rs
pub fn run_and_meassure(command: JailedCommand) -> Result<CodeExecutorResult, CodeExecutorError> {
    // spawn
    let start = Instant::now();
    let output = command.output()?;
    let duration = start.elapsed();

    debug!("Output {:?}", output);
    if output.status.success() {
        Ok(CodeExecutorResult {
            status: Some(output.status),
            output: Some(output),
            duration,
        })
    } else {
        Err(CodeExecutorError::InternalError(
            CodeExecutorInternalError {
                status: match_stderr(&output),
                output: Some(output),
                duration,
            },
        ))
    }
}

fn match_stderr(output: &Output) -> Status {
    let stderr = String::from_utf8_lossy(output.stderr.as_slice());
    if stderr.starts_with("[>>EVALUATOR<<][TIME_LIMIT]") {
        Status::TimeLimitExceeded
    } else {
        Status::RuntimeError
    }
}

pub fn run_and_meassure_2(command: &mut Command) -> Result<CodeExecutorResult, CodeExecutorError> {
    // spawn

    let start = Instant::now();
    let output = command.output()?;
    let duration = start.elapsed();

    debug!("Output {:?}", output);
    if output.status.success() {
        Ok(CodeExecutorResult {
            status: Some(output.status),
            output: Some(output),
            duration,
        })
    } else {
        Err(CodeExecutorError::InternalError(
            CodeExecutorInternalError {
                status: match_stderr(&output),
                output: Some(output),
                duration,
            },
        ))
    }
}
