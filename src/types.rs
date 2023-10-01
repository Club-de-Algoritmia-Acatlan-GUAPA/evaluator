use primitypes::problem::TestCaseResult;
#[derive(Debug)]
pub enum TestCaseError {
    InternalError(TestCaseResult),
    ExternalError(anyhow::Error),
}
impl<E> From<E> for TestCaseError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::ExternalError(err.into())
    }
}
