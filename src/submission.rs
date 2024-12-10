use pgmq::Message;
use primitypes::contest::Submission;
use sqlx::PgPool;
use tracing::{error, info};

use crate::{
    postgres::store_submission_in_db,
    problem_executor::{ProblemExecutor, ProblemExecutorError},
    queue::PostgresQueue,
    types::EvaluatorError,
};

async fn process_submission(
    executor: &ProblemExecutor<'_>,
    res: Submission,
    pg_pool: &PgPool,
) -> std::result::Result<(), EvaluatorError> {
    match executor.execute(&res).await {
        Ok(ans) | Err(ProblemExecutorError::InternalError(ans)) => {
            store_submission_in_db(pg_pool, ans, res).await
        },
        Err(ProblemExecutorError::ExternalError(e)) => {
            error!("{e:?}");
            Err(e)
        },
    }
}
pub async fn handle_message(
    pgmq: &PostgresQueue,
    pg_pool: &PgPool,
    executor: &ProblemExecutor<'_>,
    message: Message<Submission>,
) -> () {
    info!(
        "Message: {:?}",
        serde_json::to_string(&message.message).unwrap()
    );

    let msg_id = message.msg_id;
    info!("Executing message {}", msg_id);
    let submission_id = message.message.id.clone();
    match process_submission(&executor, message.message, &pg_pool).await {
        Ok(_) | Err(EvaluatorError::ProblemNotFound) => {
            info!("Deleting message {}", msg_id);
            pgmq.delete(msg_id).await;
        },
        _ => {
            info!(
                "Submission id {} failed to process, cause {}",
                submission_id.as_u128().to_string(),
                msg_id
            );
        },
    }
}
