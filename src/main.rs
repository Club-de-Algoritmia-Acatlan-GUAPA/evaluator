use anyhow::Context;
use evaluator::{
    cli::get_tracing_mode,
    configuration::get_configuration,
    problem_executor::{ProblemExecutor, ProblemExecutorError},
    store::ProblemStore,
};
use futures_lite::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, Result};
use primitypes::{
    contest::Submission,
    status::{Status, StatusPG},
};
use redis::Client;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tracing::{debug, info, instrument};
#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    let tracing_mode = get_tracing_mode();
    tracing_subscriber::fmt()
        .with_max_level(tracing_mode)
        .init();

    let config = get_configuration().expect("Unable to get configuration");

    let addr = format!("amqp://{}:{}", config.rabbitmq.host, config.rabbitmq.port);
    let con = ConnectionProperties::default();
    let conn = Connection::connect(&addr, con).await?;
    let channel = conn.create_channel().await?;
    let mut consumer = channel
        .basic_consume(
            &config.rabbitmq.queue,
            &config.rabbitmq.consumer,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
    info!(
        "RUNNING rabbitmq on address: {} and queue: {}",
        addr, config.rabbitmq.queue
    );

    let addr = format!("redis://{}:{}", config.redis.host, config.redis.port);
    let client = Client::open(addr.clone()).unwrap();
    let mut conn = client.get_connection().unwrap();
    info!("RUNNING redis on {}", addr);

    let pg_uri = format!(
        "postgres://{}@{}:{}/{}",
        config.postgres.user, config.postgres.host, config.postgres.port, config.postgres.database
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_uri)
        .await
        .context("Failed postgres connection")
        .unwrap();
    info!("RUNNING postgres on {}", pg_uri);

    let mut problem_store = ProblemStore::new();
    loop {
        let delivery = consumer.next().await;
        if let Some(Ok(delivery)) = delivery {
            match serde_json::from_reader::<_, Submission>(&*delivery.data) {
                Ok(res) => {
                    let executor = ProblemExecutor::new();
                    let problem = problem_store.get_problem(&res.problem_id).await.unwrap();

                    match executor.execute(&res, &problem).await {
                        Ok(ans) | Err(ProblemExecutorError::InternalError(ans)) => {
                            let mes = &ans.overall_result;

                            debug!("{:?}", mes);

                            let _ = sqlx::query!(
                                " 
                                UPDATE submission
                                SET output = $2 , status = $3
                                WHERE submission_id = $1
                            ",
                                res.id.as_bit_vec(),
                                json!(ans),
                                match_status_to_pg_status(&mes) as _
                            )
                            .execute(&pool)
                            .await
                            .unwrap();

                            info!("STORE submission result");
                            delivery
                                .ack(BasicAckOptions::default())
                                .await
                                .expect("basic_ack");
                            info!("ACK to rabbitmq");

                            redis::cmd("PUBLISH")
                                .arg("channel_1")
                                .arg(format!("{mes}:{0}", res.id.as_u128()))
                                .execute(&mut conn);
                            info!("NOTIFIED to redis on channel_1");
                        },
                        Err(e) => {
                            println!("{e:?}");
                        },
                    }
                    println!("---------------------------------");
                },
                Err(e) => {
                    println!("{}", e);
                },
            }
        } else {
            println!("Erroooooor in consumer");
        }
    }
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
