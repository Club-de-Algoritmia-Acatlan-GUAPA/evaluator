use anyhow::Result;
use itertools::{Either, Itertools};
use rayon::prelude::*;

use primitypes::contest::{Language, Submission};

use crate::{
    code_executor::{CodeExecutor, CodeExecutorError, CodeExecutorImpl},
    testcase::load_testcases,
    types::TestCaseError,
    utils::file_to_bytes,
    validator::Validator,
};
use primitypes::problem::Problem;
use primitypes::{
    problem::{ProblemExecutorResult, TestCaseResult},
    status::{Status, STATUS_PRECEDENCE},
};
use tracing::{info, instrument};

#[derive(Debug, Eq, PartialEq)]
pub struct ProblemExecutor;
impl Default for ProblemExecutor {
    fn default() -> Self {
        ProblemExecutor::new()
    }
}

#[derive(Debug)]
pub enum ProblemExecutorError {
    InternalError(ProblemExecutorResult),
    ExternalError(anyhow::Error),
}

#[derive(Debug)]
pub enum TestExecutionError {
    InternalError(TestCaseResult),
    ExternalError(anyhow::Error),
}

impl<E> From<E> for ProblemExecutorError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::ExternalError(err.into())
    }
}

impl From<CodeExecutorError> for ProblemExecutorError {
    fn from(err: CodeExecutorError) -> Self {
        match err {
            CodeExecutorError::InternalError(e) => Self::InternalError(ProblemExecutorResult {
                overall_result: e.status,
                test_cases_results: vec![],
                prepare_output: e.output,
            }),
            CodeExecutorError::ExternalError(e) => Self::ExternalError(e),
        }
    }
}

impl From<TestCaseError> for ProblemExecutorError {
    fn from(err: TestCaseError) -> Self {
        match err {
            TestCaseError::InternalError(e) => Self::InternalError(ProblemExecutorResult {
                overall_result: e.status,
                test_cases_results: vec![],
                prepare_output: e.output,
            }),
            TestCaseError::ExternalError(e) => Self::ExternalError(e),
        }
    }
}

impl ProblemExecutor {
    pub fn new() -> Self {
        Self {}
    }

    #[instrument]
    pub async fn execute(
        &self,
        submission: &Submission,
        problem: &Problem,
    ) -> Result<ProblemExecutorResult, ProblemExecutorError> {
        let mut executor: Box<dyn CodeExecutorImpl> = get_executor(&submission.language);

        load_testcases(problem).await?;
        executor.code(submission.code.to_string());
        executor.set_id(submission.id.as_u128());

        let compilation_output = executor.prepare_code_env().await?.output;

        let mut validator = Validator::new(
            &problem.validation_type,
            &problem.problem_id,
            &submission.id,
        );

        validator.prepare_validator().await?;

        let mut test_ok: Vec<TestCaseResult> = vec![];
        let mut test_err: Vec<TestCaseError> = vec![];

        let _ = problem
            .test_cases
            .chunks(6)
            .map(|s| {
                let (ok, err): (
                    Vec<Result<TestCaseResult, _>>,
                    Vec<Result<_, TestCaseError>>,
                ) = s
                    .par_iter()
                    .map(|test_case| {
                        let output_file;
                        let input_file;
                        #[cfg(target_os = "linux")]
                        {
                            output_file = format!(
                                "/app/evaluator/playground/{0}/user_output_{1}.out",
                                submission.id.as_u128(),
                                test_case.id
                            );
                            input_file = format!(
                                "/resources/{}/input_{}.in",
                                problem.problem_id.as_u32(),
                                test_case.id.to_string().as_str()
                            );
                            info!("EXECUTING input_file =  {}, output_file = {} ", input_file, output_file);
                            _ = executor.execute_nsjail(&input_file, &output_file).map_err(
                                |op| match op {
                                    CodeExecutorError::InternalError(e) => {
                                        TestCaseError::InternalError(TestCaseResult {
                                            status: e.status,
                                            id: test_case.id,
                                            output: e.output,
                                        })
                                    }
                                    CodeExecutorError::ExternalError(e) => {
                                        TestCaseError::ExternalError(e)
                                    }
                                },
                            )?;
                        }
                        #[cfg(not(target_os = "linux"))]
                        {
                            output_file = format!(
                                "./playground/{0}/user_output_{1}.out",
                                submission.id.as_u128(),
                                test_case.id
                            );
                            input_file = format!(
                                "./resources/{}/input_{}.in",
                                problem.problem_id.as_u32(),
                                test_case.id.to_string().as_str()
                            );
                            info!("EXECUTING input_file =  {}, output_file = {} ", input_file, output_file);
                            _ = executor.execute(&input_file, &output_file).map_err(
                                |op| match op {
                                    CodeExecutorError::InternalError(e) => {
                                        TestCaseError::InternalError(TestCaseResult {
                                            status: e.status,
                                            id: test_case.id,
                                            output: e.output,
                                        })
                                    }
                                    CodeExecutorError::ExternalError(e) => {
                                        TestCaseError::ExternalError(e)
                                    }
                                },
                            )?;
                        }
                        info!("VALIDATING test case {}", test_case.id);
                        match validator.check_input(test_case) {
                            Ok(mut e) => {
                                if let Some(output) = e.output.as_mut() {
                                    let user_output =
                                        file_to_bytes(output_file).unwrap_or_default();
                                    output.stdout = user_output;
                                }
                                Ok(e)
                            }
                            Err(TestCaseError::InternalError(mut e)) => {
                                if let Some(output) = e.output.as_mut() {
                                    let user_output =
                                        file_to_bytes(output_file).unwrap_or_default();
                                    output.stdout = user_output;
                                }
                                Err(TestCaseError::InternalError(e))
                            }
                            s @ Err(TestCaseError::ExternalError(_)) => s,
                        }
                    })
                    .partition(Result::is_ok);

                let is_err = !err.is_empty();

                let (internal_error, external_error): (Vec<_>, Vec<_>) = err
                    .into_iter()
                    .map(Result::unwrap_err)
                    .partition_map(|e| match e {
                        TestCaseError::InternalError(err) => Either::Left(err),
                        err @ TestCaseError::ExternalError(_) => Either::Right(err),
                    });
                let ok_unwrap = ok.into_iter().map(Result::unwrap);
                test_ok.extend(ok_unwrap);
                test_ok.extend(internal_error);
                test_err.extend(external_error);
                if is_err {
                    return Err(());
                }
                Ok(())
            })
            .collect::<Result<Vec<_>, _>>();

        test_ok.sort_by_key(|b| b.id);

        if !test_err.is_empty() {
            let first_err = test_err.remove(0);
            return Err(first_err.into());
        };
        let overall_result_testcase = test_ok.iter().max_by_key(|testcase_result| {
            STATUS_PRECEDENCE
                .get(&testcase_result.status)
                .unwrap_or(&10)
        });

        let overall_result = if let Some(result) = overall_result_testcase {
            result.status.to_owned()
        } else {
            Status::UnknownError("Status can't be infered".to_string())
        };

        let _ = executor.destroy().await;

        Ok(ProblemExecutorResult {
            overall_result,
            test_cases_results: test_ok,
            prepare_output: compilation_output,
        })
    }
}

// "optimization" maybe I'm not sure, try to not use Box
fn get_executor(language: &Language) -> Box<dyn CodeExecutorImpl> {
    match language {
        Language::Python3 => {
            use crate::languages::python_3;
            Box::new(CodeExecutor::<python_3::Python3>::new())
        }
        Language::Cpp11 => {
            use crate::languages::cpp;
            Box::new(CodeExecutor::<cpp::Cpp11>::new())
        }
        Language::Cpp17 => {
            use crate::languages::cpp;
            Box::new(CodeExecutor::<cpp::Cpp17>::new())
        }
        Language::Java => {
            use crate::languages::java;
            Box::new(CodeExecutor::<java::Java>::new())
        }
        _ => todo!(),
    }
}
