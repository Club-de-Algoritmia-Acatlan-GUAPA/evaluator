use std::process::{Command, Stdio};

use anyhow::{anyhow, Result};
use primitypes::{
    problem::{Checker, ProblemID, SubmissionID, TestCase, TestCaseResult, ValidatorType},
    status::{CmpExitCodes, Status, TestLibExitCodes},
};
use tokio::fs::metadata;

use crate::{code_executor::CodeExecutorError, types::TestCaseError};
#[derive(Clone)]
pub struct Validator {
    validation_type: ValidatorType,
    checker: Option<Checker>,
    problem_id: ProblemID,
    submission_id: SubmissionID,
}

impl Validator {
    pub fn new(
        validation_type: &ValidatorType,
        problem_id: &ProblemID,
        submission_id: &SubmissionID,
    ) -> Self {
        Validator {
            validation_type: validation_type.clone(),
            checker: None,
            problem_id: problem_id.clone(),
            submission_id: submission_id.clone(),
        }
    }

    pub fn set_checker(&mut self, checker: Option<&Checker>) {
        self.checker = checker.cloned();
    }

    pub fn check_input(&self, test_case: &TestCase) -> Result<TestCaseResult, TestCaseError> {
        match self.validation_type {
            ValidatorType::TestLibChecker => self.testlib_check_input_2(test_case),
            ValidatorType::LiteralChecker => self.literal_checker(test_case),
        }
    }

    pub async fn prepare_validator(&mut self) -> Result<(), CodeExecutorError> {
        println!("PREPARING");
        if let ValidatorType::TestLibChecker = self.validation_type {
            if let Ok(metadata) =
                metadata(format!("./resources/{}/checker", self.problem_id.as_u32())).await
            {
                if metadata.is_file() {
                    return Ok(());
                }
            };
            let dir = format!("./resources/{}", self.problem_id.as_u32());

            let o = Command::new("/usr/bin/g++")
                .current_dir(dir)
                .args(vec!["checker.cpp", "-o", "checker"])
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .output()?;
            println!("CHECKER {:?}", o);
            println!("PREPARED");
        }
        Ok(())
    }

    fn testlib_check_input_2(&self, test_case: &TestCase) -> Result<TestCaseResult, TestCaseError> {
        println!("VALIDATING");
        let judge_input_file_name = format!(
            "./resources/{}/input_{}.in",
            self.problem_id.as_u32(),
            test_case.id
        );

        let judge_output_file_name = format!(
            "./resources/{}/output_{}.out",
            self.problem_id.as_u32(),
            test_case.id
        );

        let user_output_file_name = format!(
            "./playground/{0}/user_output_{1}.out",
            self.submission_id.as_u128(),
            test_case.id
        );

        let checker = format!("./resources/{}/checker", self.problem_id.as_u32(),);

        let mut c = Command::new(checker);
        //.current_dir(format!("./playground/{}", self.submission_id.as_u128()))
        c.args(vec![
            judge_input_file_name,
            user_output_file_name,
            judge_output_file_name,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
        println!("{:?}", c);
        let output = c.output()?;

        println!("FINISHED VALIDATION");
        println!("{output:?}");
        println!("CODE {:?}", output.status.code());
        let status_code = output.status.code();

        let status = match status_code {
            Some(res) => match res.try_into() {
                Ok(TestLibExitCodes::Accepted) => Status::Accepted,
                Ok(TestLibExitCodes::WrongAnswer) => Status::WrongAnswer,
                Ok(TestLibExitCodes::PartialExecution) => Status::PartialPoints,
                Ok(TestLibExitCodes::FormatError) => Status::WrongAnswer,
                Err(v) => Status::UnknownError(format!("found {v:?}")),
            },
            None => Status::UnknownError("Testlib Failed".to_string()),
        };

        match status {
            s @ Status::WrongAnswer | s @ Status::TimeLimitExceeded | s @ Status::PartialPoints => {
                Err(TestCaseError::InternalError(TestCaseResult {
                    status: s,
                    id: test_case.id,
                    output: Some(output),
                }))
            },
            s @ Status::Accepted => Ok(TestCaseResult {
                status: s,
                id: test_case.id,
                output: Some(output),
            }),
            Status::UnknownError(e) => Err(TestCaseError::ExternalError(anyhow!(e))),
            _ => {
                unreachable!()
            },
        }
    }

    fn literal_checker(&self, test_case: &TestCase) -> Result<TestCaseResult, TestCaseError> {
        let mut c = Command::new("/usr/bin/cmp");

        let user_output_file_name = format!(
            "./playground/{0}/user_output_{1}.out",
            self.submission_id.as_u128(),
            test_case.id
        );
        let judge_output_file_name = format!(
            "./resources/{}/output_{}.out",
            self.problem_id.as_u32(),
            test_case.id
        );

        let output = c
            .arg("--silent")
            .arg(user_output_file_name)
            .arg(judge_output_file_name)
            .output()?;

        let status = match output.status.code() {
            Some(code) => match code.try_into() {
                Ok(CmpExitCodes::Equal) => Status::Accepted,
                Ok(CmpExitCodes::Different) => Status::WrongAnswer,
                Ok(CmpExitCodes::Problem) => Status::UnknownError(
                    "Something went wrong with CMP finished with status 2".to_string(),
                ),
                Err(_) => Status::UnknownError(format!(
                    "This {} is inexistent in cmp documentation",
                    code
                )),
            },
            None => Status::UnknownError("Something went wrong with Cmp".to_string()),
        };
        match status {
            s @ Status::WrongAnswer => Err(TestCaseError::InternalError(TestCaseResult {
                status: s,
                id: test_case.id,
                output: Some(output),
            })),
            s @ Status::Accepted => Ok(TestCaseResult {
                status: s,
                id: test_case.id,
                output: Some(output),
            }),
            Status::UnknownError(e) => Err(TestCaseError::ExternalError(anyhow!(e))),
            _ => {
                unreachable!()
            },
        }
    }
}
