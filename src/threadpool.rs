use flume;
use futures::future::join_all;
use std::cmp;
use std::num::NonZeroUsize;
use tokio::task::JoinHandle;

use crate::{
    code_executor::{CodeExecutorError, CodeExecutorImpl, CodeExecutorResult},
    types::{TestCaseError, TestCaseResult},
    validator::Validator,
};
use primitypes::problem::Problem;

pub struct ThreadPool {
    pool: Vec<JoinHandle<()>>,
    sender_code: SenderCode,
}

type ReceiverCode = flume::Receiver<(
    Box<dyn CodeExecutorImpl>,
    Problem,
    Validator,
    SenderResult,
    usize,
)>;
type SenderCode = flume::Sender<(
    Box<dyn CodeExecutorImpl>,
    Problem,
    Validator,
    SenderResult,
    usize,
)>;

type ReceiverResult = flume::Receiver<Result<TestCaseResult, TestCaseError>>;
type SenderResult = flume::Sender<Result<TestCaseResult, TestCaseError>>;

impl ThreadPool {
    pub fn new() -> Self {
        let threads = ThreadPool::get_threads();
        let (send_code, receive_code) = flume::bounded(ThreadPool::get_threads() * 2);

        let pool = (0..threads)
            .map(|_| tokio::task::spawn(tasked(receive_code.clone())))
            .collect::<Vec<_>>();

        ThreadPool {
            pool,
            sender_code: send_code,
        }
    }
    pub fn get_threads() -> usize {
        cmp::min(
            std::thread::available_parallelism()
                .map(NonZeroUsize::get)
                .unwrap_or(1),
            16,
        );
        4
    }
    pub fn send(
        &self,
        executor: Box<dyn CodeExecutorImpl>,
        problem: Problem,
        validator: Validator,
        test_case_id: usize,
        sender: SenderResult,
    ) {
        let _ = self
            .sender_code
            .send((executor, problem, validator, sender, test_case_id));
    }
    pub async fn join(self) -> futures::future::JoinAll<JoinHandle<()>> {
        join_all(self.pool)
    }
}

async fn tasked(receive_code: ReceiverCode) {
    while let Ok((executor, problem, validator, sender, test_case_id)) =
        receive_code.recv_async().await
    {
        let res = execute_code(executor, problem, validator, test_case_id);
        println!("{:?}", res);
        let _ = sender.send(res);
    }
}
fn execute_code(
    executor: Box<dyn CodeExecutorImpl>,
    problem: Problem,
    validator: Validator,
    test_case_id: usize,
) -> Result<TestCaseResult, TestCaseError> {
    let test_case = &problem.test_cases[test_case_id];

    let res: CodeExecutorResult;
    #[cfg(target_os = "linux")]
    {
        res = executor
            .execute_nsjail(
                format!(
                    "/resources/{}/input_{}.in",
                    problem.problem_id.as_u32(),
                    test_case.id.to_string().as_str()
                )
                .as_str(),
            )
            .map_err(|op| to_testcase_error(op, test_case.id))?;
    }
    #[cfg(not(target_os = "linux"))]
    {
        res = executor
            .execute(&test_case.input_case.clone(), vec![])
            .map_err(|op| to_testcase_error(op, test_case.id))?;
    }
    let status = validator.check_input(&test_case, &res.output.unwrap())?;
    Ok(status)
}
fn to_testcase_error(op: CodeExecutorError, id: i32) -> TestCaseError {
    match op {
        CodeExecutorError::InternalError(e) => TestCaseError::InternalError(TestCaseResult {
            status: e.status,
            id,
            output: e.output,
        }),
        CodeExecutorError::ExternalError(e) => TestCaseError::ExternalError(e),
    }
}
