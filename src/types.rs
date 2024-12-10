use primitypes::problem::TestCaseResult;
#[derive(Debug)]
pub enum EvaluatorError {
    ProblemNotFound,
    SubmissionIdNotFound,
    FailedTestLibCheckerCompilation(String),
    GenericError(anyhow::Error),
}
#[derive(Debug)]
pub enum TestCaseError {
    InternalError(TestCaseResult),
    ExternalError(EvaluatorError),
}

#[derive(Debug)]
pub enum TestCaseErrorExternalError {
    ProblemNotFound,
    GenericError(anyhow::Error),
}
impl<E> From<E> for TestCaseError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::ExternalError(EvaluatorError::GenericError(err.into()))
    }
}
