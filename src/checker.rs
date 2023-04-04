use anyhow::Result;

use std::{
    fs,
    io::{Write, Read},
    process::{Command, Output, Stdio},
};

use crate::types::{Status, TestCase, TestCaseResult, TestLibExitCodes};

pub fn check_input(test_case: &TestCase, output: Output) -> Result<TestCaseResult> {
    let user_output = String::from_utf8_lossy(&output.stdout);

    let input_file_name = format!("./playground/judge_input_{}.in", test_case.id);
    let mut input_file = fs::File::create(input_file_name)?;
    input_file.write_all(test_case.input_case.as_bytes())?;
    let input_file_name = format!("judge_input_{}.in", test_case.id);

    let user_output_file_name = format!("./playground/user_output_{}.out", test_case.id);
    let mut user_output_file = fs::File::create(user_output_file_name)?;
    user_output_file.write_all(user_output.as_bytes())?;
    let user_output_file_name = format!("user_output_{}.out", test_case.id);

    let judge_output_file_name = format!("./playground/judge_output_{}.out", test_case.id);
    let mut judge_output_file = fs::File::create(judge_output_file_name)?;
    judge_output_file.write_all(test_case.output_case.as_bytes())?;
    let judge_output_file_name = format!("judge_output_{}.out", test_case.id);

    let mut exec_testlib_checker = Command::new("./checker")
        .current_dir("./playground")
        .stdin(Stdio::piped())
        .arg(input_file_name)
        .arg(user_output_file_name)
        .arg(judge_output_file_name)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let status_code = exec_testlib_checker.wait()?.code();

    let status = match status_code {
        Some(res) => match res.try_into() {
            Ok(TestLibExitCodes::Accepted) => Status::Accepted,
            Ok(TestLibExitCodes::WrongAnswer) => Status::WrongAnswer,
            Ok(TestLibExitCodes::PartialExecution) => Status::PartialExecution,
            Err(v) => Status::UnknownError(format!("found {v:?}")),
        },
        None => Status::UnknownError("testlib execution fails".to_string()),
    };

    let ans = String::from_utf8_lossy(output.stdout.as_slice()).into_owned();
    Ok(TestCaseResult {
        status,
        id: test_case.id,
        output: Some(ans)
    })
}
