use std::process::{Command, Stdio};

use anyhow::{anyhow, Result};
use primitypes::{
    contest::Language,
    problem::{ProblemId, SubmissionId, TestCaseInfo, TestCaseResult, ValidationType},
    status::{CmpExitCodes, Status, TestLibExitCodes},
};
use tokio::fs::metadata;
use tracing::info;

use crate::{code_executor::CodeExecutorError, consts::LANGUAGE, types::TestCaseError};
#[derive(Clone)]
pub struct Validator<'a> {
    validation_type: &'a ValidationType,
    problem_id: &'a ProblemId,
    submission_id: &'a SubmissionId,
    resources: &'a str,
    playground: &'a str,
}

impl<'a> Validator<'a> {
    pub fn new(
        validation_type: &'a ValidationType,
        problem_id: &'a ProblemId,
        submission_id: &'a SubmissionId,
        resources: &'a str,
        playground: &'a str,
    ) -> Self {
        Validator {
            validation_type,
            problem_id,
            submission_id,
            resources,
            playground,
        }
    }

    pub fn check_input(&self, test_case: &TestCaseInfo) -> Result<TestCaseResult, TestCaseError> {
        match self.validation_type {
            ValidationType::TestlibChecker => self.testlib_check_input_2(test_case),
            ValidationType::LiteralChecker => self.literal_checker(test_case),
            ValidationType::Interactive => todo!(),
        }
    }

    pub async fn prepare_validator(&mut self) -> Result<(), CodeExecutorError> {
        info!("PREPARING VALIDATOR");
        if let ValidationType::TestlibChecker = self.validation_type {
            if let Ok(metadata) = metadata(format!(
                "{}/{}/checker",
                self.resources,
                self.problem_id.as_u32()
            ))
            .await
            {
                if metadata.is_file() {
                    return Ok(());
                }
            };
            let dir = format!("{}/{}", self.resources, self.problem_id.as_u32());

            let cpp = LANGUAGE.get(&Language::Cpp17).unwrap();
            info!("COMPILING CHECKER {:?} with CPP {}", dir, cpp.path);
            let o = Command::new(&cpp.path)
                .current_dir(dir)
                .args(vec!["checker.cpp", "-o", "checker"])
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .output()?;
            if !o.status.success() {
                return Err(CodeExecutorError::ExternalError(anyhow!(
                    String::from_utf8_lossy(&o.stderr).to_string()
                )));
            }
            info!("CHECKER COMPILED {:?}", o);
            info!("PREPARED");
        }
        Ok(())
    }

    fn testlib_check_input_2(
        &self,
        test_case: &TestCaseInfo,
    ) -> Result<TestCaseResult, TestCaseError> {
        info!("VALIDATING");

        let judge_input_file_name = &test_case.stdin_path;
        let judge_output_file_name = &test_case.stdout_path;

        let user_output_file_name = format!(
            "{}/{}/{}.out",
            self.playground,
            self.submission_id.as_u128(),
            test_case.id
        );

        let checker = format!("{}/{}/checker", self.resources, self.problem_id.as_u32(),);

        let mut c = Command::new(checker);
        c.args(vec![
            judge_input_file_name,
            &user_output_file_name,
            judge_output_file_name.as_str(),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
        info!("{:?}", c);
        let output = c.output()?;

        info!("FINISHED VALIDATION");
        info!("{output:?}");
        info!("CODE {:?}", output.status.code());
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

    fn literal_checker(&self, test_case: &TestCaseInfo) -> Result<TestCaseResult, TestCaseError> {
        let cmp = LANGUAGE.get(&Language::Cmp).expect("/cmp path not set");
        let mut c = Command::new(cmp.path.as_str());

        let user_output_file_name = format!(
            "{}/{}/{}.out",
            self.playground,
            self.submission_id.as_u128(),
            test_case.id
        );
        let judge_output_file_name = test_case.stdout_path.clone();
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
                id: test_case.id.clone(),
                output: Some(output),
            })),
            s @ Status::Accepted => Ok(TestCaseResult {
                status: s,
                id: test_case.id.clone(),
                output: Some(output),
            }),
            Status::UnknownError(e) => Err(TestCaseError::ExternalError(anyhow!(e))),
            _ => {
                unreachable!()
            },
        }
    }
}
