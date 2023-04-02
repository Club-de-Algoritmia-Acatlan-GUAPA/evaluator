use crate::back_to_enum;
use anyhow::Result;
use std::process::Output;

#[derive(Debug, Clone)]
pub enum Language {
    Python3,
    Java,
    Cpp,
}
#[derive(Debug, Clone)]
pub enum Status {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    PartialExecution,
    RuntimeError,
    UnknownError(String),
}

back_to_enum! {
    #[derive(Debug)]
    #[repr(i32)]
    pub enum TestLibExitCodes {
        Accepted = 0,
        WrongAnswer = 1,
        PartialExecution = 7,
    }
}
#[derive(Debug, Clone)]
pub enum PolicyExecution {
    Checker,
    Interactive,
    AnswerFile,
}
#[derive(Debug, Clone)]
pub struct SystemPolicy;

#[derive(Debug, Clone)]
pub struct TestCase {
    pub input_case: String,
    pub output_case: String,
    pub id: i32,
}

#[derive(Debug, Clone)]
pub struct Checker {
    pub checker: String,
}

#[derive(Debug, Clone)]
pub struct CodeExecutorResult {
    pub err: Option<Status>,
    pub output: Option<Output>,
}
pub trait CodeExecutor {
    fn execute(&self, test_case: &TestCase) -> Result<CodeExecutorResult>;
}

#[derive(Debug, Clone)]
pub struct Submission {
    pub language: Language,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub id: String,
    pub name: Option<String>,
    pub policy_execution: PolicyExecution,
    // todo default implementation for system policy
    pub system_policy: Option<SystemPolicy>,
    pub test_cases: Vec<TestCase>,
    pub checker: Option<Checker>,
}
