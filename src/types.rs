use anyhow::Result;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::process::Output;

use crate::back_to_enum;
use crate::validator::ValidatorType;

#[derive(Debug, Clone)]
pub enum Language {
    Python3,
    Java,
    Cpp,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Status {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    PartialExecution,
    RuntimeError,
    UnknownError(String),
}

lazy_static! {
    pub static ref STATUS_PRECEDENCE: HashMap<Status, i32> = HashMap::from([
        (Status::Accepted, 0),
        (Status::PartialExecution, 1),
        (Status::WrongAnswer, 2),
        (Status::TimeLimitExceeded, 3),
        (Status::RuntimeError, 4),
    ]);
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestCaseResult {
    pub status: Status,
    pub id: i32,
    pub output: Option<Output>,
}
#[derive(Debug, Clone)]
pub struct Checker {
    pub checker: String,
}

#[derive(Debug, Clone)]
pub struct Submission {
    pub language: Language,
    pub code: String,
    pub id: i32,
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
    pub validation_type: ValidatorType,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProblemExecutorResult {
    pub overall_result: Status,
    pub test_cases_results: Vec<TestCaseResult>,
}
