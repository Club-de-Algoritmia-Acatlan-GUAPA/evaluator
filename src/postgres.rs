use anyhow::{anyhow, Context};
use primitypes::{
    contest::Submission,
    problem::ProblemExecutorResult,
    status::{Status, StatusPG},
};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::{debug, info};

use crate::{consts::CONFIGURATION, types::EvaluatorError};
pub fn pg_uri() -> String {
    let config = &CONFIGURATION.postgres;
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.database
    )
}
pub async fn get_postgres_pool() -> PgPool {
    let pg_uri = pg_uri();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_uri)
        .await
        .context("Failed postgres connection")
        .unwrap();

    info!("RUNNING postgres on {}", pg_uri);
    pool
}

pub async fn store_submission_in_db(
    pool: &PgPool,
    ans: ProblemExecutorResult,
    res: Submission,
) -> Result<(), EvaluatorError> {
    let mes = &ans.overall_result;
    debug!("{:?}", mes);
    // TODO: if there is a contest_id, there should be a transaction
    store_submission_in_submission_table(pool, &ans, mes, &res).await?;
    if res.contest_id.is_some() {
        store_submission_in_contest_submission_table(pool, mes, &res).await?;
    }
    info!("STORE submission result");
    Ok(())
}

async fn store_submission_in_submission_table(
    pool: &PgPool,
    ans: &ProblemExecutorResult,
    mes: &Status,
    res: &Submission,
) -> std::result::Result<(), EvaluatorError> {
    //let subsec_millis = ans.total_duration.subsec_millis() as u64;
    let millis: i32 = ans.total_duration.as_millis().try_into().unwrap();
    //info!("Duration: {}s {}ms", secs, subsec_millis);
    //info!("{}", secs * 1000 + subsec_millis);
    let _ = match sqlx::query!(
        " 
                UPDATE submission
                SET output = $2, 
                status = $3,
                execution_time = $4
                WHERE id = $1
                RETURNING id
                ",
        res.id.as_bit_vec(),
        json!(ans),
        match_status_to_pg_status(&mes) as _,
        millis
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| EvaluatorError::GenericError(e.into()))?
    {
        Some(_) => Ok(()),
        None => Err(EvaluatorError::ProblemNotFound),
    };
    Ok(())
}
async fn store_submission_in_contest_submission_table(
    pool: &PgPool,
    mes: &Status,
    res: &Submission,
) -> std::result::Result<(), EvaluatorError> {
    let _ = sqlx::query!(
        " 
                    UPDATE contest_submission
                    SET status = $2
                    WHERE submission_id = $1
                    ",
        res.id.as_bit_vec(),
        match_status_to_pg_status(&mes) as _
    )
    .execute(pool)
    .await
    .map_err(|e| EvaluatorError::GenericError(e.into()))?;
    Ok(())
}
fn match_status_to_pg_status(status: &Status) -> StatusPG {
    match status {
        Status::Pending => StatusPG::Pending,
        Status::WrongAnswer => StatusPG::WrongAnswer,
        Status::Accepted => StatusPG::Accepted,
        Status::RuntimeError => StatusPG::RuntimeError,
        Status::TimeLimitExceeded => StatusPG::TimeLimitExceeded,
        Status::PartialPoints => StatusPG::PartialPoints,
        Status::CompilationError => StatusPG::CompilationError,
        Status::UnknownError(_) => StatusPG::UnknownError,
    }
}
