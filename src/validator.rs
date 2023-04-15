use anyhow::Result;

use std::{
    fs,
    io::{Read, Write},
    os::unix::process::ExitStatusExt,
    process::{Command, ExitStatus, Output, Stdio},
};

use crate::types::{Status, TestCase, TestCaseResult, TestLibExitCodes};
#[derive(Clone, Debug)]
pub enum ValidatorType {
    TestLibChecker,
    LiteralChecker,
}
pub struct Validator {
    validation_type: ValidatorType,
    checker: Option<String>,
}
impl Validator {
    pub fn new(validation_type: ValidatorType) -> Self {
        Validator {
            validation_type,
            checker: None,
        }
    }
    pub fn set_checker(&mut self, checker: &str) {
        self.checker = Some((*checker).to_string());
    }

    pub fn check_input(&self, test_case: &TestCase, output: Output) -> Result<TestCaseResult> {
        match self.validation_type {
            ValidatorType::TestLibChecker => testlib_check_input(test_case, output),
            ValidatorType::LiteralChecker => literal_checker(test_case, output),
        }
    }

    pub fn prepare_validator(&self) -> Result<()> {
        if let Some(checker) = &self.checker {
            let mut file = fs::File::create("./playground/checker.cpp")?;
            file.write_all(checker.as_bytes())?;
            let comp = Command::new("g++-12")
                .current_dir("./playground")
                .args(vec!["checker.cpp", "-o", "checker"])
                .stderr(Stdio::null())
                .stdout(Stdio::null())
                .stdin(Stdio::null())
                .spawn()?;
            let _ = comp.wait_with_output()?; // output from compiling the checker
        }
        Ok(())
    }
}

fn literal_checker(test_case: &TestCase, output: Output) -> Result<TestCaseResult> {
    let user_output = String::from_utf8_lossy(&output.stdout);

    let status = if user_output == test_case.output_case {
        Status::Accepted
    } else {
        Status::WrongAnswer
    };

    Ok(TestCaseResult {
        status,
        id: test_case.id,
        output: Some(Output {
            status: ExitStatus::from_raw(0),
            stdout: output.stdout,
            stderr: output.stderr,
        }),
    })
}
fn testlib_check_input(test_case: &TestCase, output: Output) -> Result<TestCaseResult> {
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

    let mut child = Command::new("./checker")
        .current_dir("./playground")
        .args(vec![
            input_file_name,
            user_output_file_name,
            judge_output_file_name,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let status_code = child.wait()?.code();

    let status = match status_code {
        Some(res) => match res.try_into() {
            Ok(TestLibExitCodes::Accepted) => Status::Accepted,
            Ok(TestLibExitCodes::WrongAnswer) => Status::WrongAnswer,
            Ok(TestLibExitCodes::PartialExecution) => Status::PartialExecution,
            Err(v) => Status::UnknownError(format!("found {v:?}")),
        },
        None => Status::UnknownError("testlib execution fails".to_string()),
    };

    Ok(TestCaseResult {
        status,
        id: test_case.id,
        output: Some(Output {
            status: ExitStatus::from_raw(status_code.unwrap()),
            stdout: output.stdout,
            stderr: child
                .stderr
                .unwrap()
                .bytes()
                .filter_map(|x| x.ok())
                .collect(),
        }),
    })
}
