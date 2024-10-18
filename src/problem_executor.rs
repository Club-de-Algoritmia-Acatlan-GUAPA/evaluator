use std::collections::HashMap;

use anyhow::Result;
use itertools::{Either, Itertools};
use primitypes::{
    contest::{Language, Submission},
    problem::{ProblemExecutorResult, TestCaseConfig, TestCaseInfo, TestCaseResult},
    status::{Status, STATUS_PRECEDENCE},
};
use rayon::prelude::*;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    code_executor::{CodeExecutor, CodeExecutorError, CodeExecutorImpl, CodeExecutorResult},
    configuration::EvaluationType,
    consts::{LANGUAGE, PLAYGROUND},
    languages::{compiled, interpreted},
    store::ProblemStore,
    types::TestCaseError,
    utils::file_to_bytes,
    validator::Validator,
};

#[derive(Debug)]
pub struct ProblemExecutor {
    playground: String,
    resources: String,
    problem_store: Box<dyn ProblemStore<Error = TestCaseError>>,
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
    pub fn new(
        playground: &str,
        resources: &str,
        store: Box<dyn ProblemStore<Error = TestCaseError>>,
    ) -> Self {
        Self {
            playground: playground.to_string(),
            resources: resources.to_string(),
            problem_store: store,
        }
    }

    #[instrument]
    pub async fn execute(
        &self,
        submission: &Submission,
    ) -> Result<ProblemExecutorResult, ProblemExecutorError> {
        let problem = self
            .problem_store
            .get_problem_by_id(&submission.problem_id)
            .await?;
        let mut executor: Box<dyn CodeExecutorImpl> = Self::get_executor(&submission.language);

        executor.set_code(String::from_utf8_lossy(&submission.code).to_string());
        executor.set_id(submission.id.as_u128());

        let mut validator = Validator::new(
            &problem.validation,
            &problem.id,
            &submission.id,
            self.resources.as_str(),
            self.playground.as_str(),
        );

        let compilation_output = match executor.prepare_code_env().await {
            Ok(result) => result.output,
            Err(e) => {
                let _ = executor.destroy().await;
                return Err(e.into());
            },
        };

        let mut test_ok: Vec<TestCaseResult> = vec![];
        let mut test_err: Vec<TestCaseError> = vec![];

        //let test_cases_config = self
        //    .problem_store
        //    .get_test_case_config(&problem.id)
        //    .await
        //    .map_err(|_| {
        //        ProblemExecutorError::ExternalError(anyhow!("Unable to get problem
        // config file"))    })?;

        let chunks = problem.test_cases.chunks(6);

        let test_cases_config = TestCaseConfig {
            test_cases: problem.test_cases.clone(),
            problem_id: problem.id.clone(),
        };
        validator.prepare_validator().await?;
        let _: Result<Vec<_>, _> = chunks
            .map(|test_case_chunk| {
                let (ok, err): (
                    Vec<Result<TestCaseResult, _>>,
                    Vec<Result<_, TestCaseError>>,
                ) = test_case_chunk
                    .par_iter()
                    .map(|test_case_id| {
                        let test_case_info = self
                            .problem_store
                            .load_testcase(&problem.id, test_case_id)?;

                        let user_output_file = format!(
                            "{}/{}/{}.out",
                            self.playground,
                            submission.id.as_u128(),
                            test_case_info.id
                        );
                        let input_file = test_case_info.stdin_path.as_str();

                        let execution_result = Self::execute_code(
                            executor.as_ref(),
                            input_file,
                            &user_output_file,
                            &test_case_info.id,
                        )?;
                        let mut validation_result =
                            Self::validate(&validator, &test_case_info, &user_output_file)?;
                        validation_result.duration = execution_result.duration;
                        validation_result.output = execution_result.output;
                        Ok(validation_result)
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

        let test_ok = Self::sort_test_cases_by_config_file(test_ok, &test_cases_config);

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

    #[inline]
    fn execute_code(
        executor: &dyn CodeExecutorImpl,
        input_file: &str,
        output_file: &str,
        test_case_id: &Uuid,
    ) -> Result<CodeExecutorResult, TestCaseError> {
        info!(
            "EXECUTING input_file =  {}, output_file = {} ",
            input_file, output_file
        );
        match executor.execute(input_file, output_file) {
            Ok(result) => Ok(result),
            Err(op) => match op {
                CodeExecutorError::InternalError(e) => {
                    Err(TestCaseError::InternalError(TestCaseResult {
                        status: e.status,
                        id: *test_case_id,
                        output: e.output,
                        duration: e.duration,
                    }))
                },
                CodeExecutorError::ExternalError(e) => Err(TestCaseError::ExternalError(e)),
            },
        }
    }

    fn sort_test_cases_by_config_file(
        tests: Vec<TestCaseResult>,
        config: &TestCaseConfig,
    ) -> Vec<TestCaseResult> {
        let mut map = HashMap::new();
        for test in tests {
            map.insert(test.id.clone(), test);
        }
        let mut res = vec![];
        for test_id in config.test_cases.iter() {
            if map.contains_key(test_id) {
                let test = map.remove(test_id).unwrap();
                res.push(test);
            }
        }
        res
    }

    fn validate(
        validator: &Validator,
        test_case: &TestCaseInfo,
        output_file: &str,
    ) -> Result<TestCaseResult, TestCaseError> {
        info!("VALIDATING test case {}", test_case.id);
        match validator.check_input(test_case) {
            Ok(mut e) => {
                if let Some(output) = e.output.as_mut() {
                    let user_output = file_to_bytes(output_file).unwrap_or_default();
                    output.stdout = user_output;
                }
                Ok(e)
            },
            Err(TestCaseError::InternalError(mut e)) => {
                if let Some(output) = e.output.as_mut() {
                    let user_output = file_to_bytes(output_file).unwrap_or_default();
                    output.stdout = user_output;
                }
                Err(TestCaseError::InternalError(e))
            },
            s @ Err(TestCaseError::ExternalError(_)) => s,
        }
    }

    fn get_executor(language: &Language) -> Box<dyn CodeExecutorImpl> {
        match LANGUAGE.get(language) {
            //Language::Python3 => {
            //    use crate::languages::python_3;
            //    Box::new(CodeExecutor::<python_3::Python3>::new(*PLAYGROUND))
            //},
            Some(cmd) => match cmd.eval_type {
                EvaluationType::Compiled => Box::new(CodeExecutor::<compiled::Compiled>::new2(
                    *PLAYGROUND,
                    language,
                )),
                EvaluationType::Interpreted => Box::new(
                    CodeExecutor::<interpreted::Interpreted>::new2(*PLAYGROUND, language),
                ),
                EvaluationType::Java => Box::new(CodeExecutor::<compiled::Compiled>::new2(
                    *PLAYGROUND,
                    language,
                )),
            },
            //Language::Cpp17 => {
            //    use crate::languages::cpp;
            //    Box::new(CodeExecutor::<cpp::Cpp17>::new(*PLAYGROUND))
            //},
            //Language::Java => {
            //    use crate::languages::java;
            //    Box::new(CodeExecutor::<java::Java>::new(*PLAYGROUND))
            //},
            _ => todo!(),
        }
    }
}
