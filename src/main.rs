use evaluator::{
    broker::MessageBroker,
    cli::init_tracing,
    consts::{CONFIGURATION, PLAYGROUND, RESOURCES},
    postgres::get_postgres_pool,
    problem_executor::{ProblemExecutor, ProblemExecutorError},
    redis::RedisConnection,
    store::FileSystemStore,
};
use lapin::{options::*, Result};
use primitypes::{
    contest::Submission,
    status::{Status, StatusPG},
};
use serde_json::json;
use sqlx::PgPool;
use tracing::{debug, info};
#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let mut broker = MessageBroker::new(&CONFIGURATION.rabbitmq).await;

    let redis_pool = RedisConnection::new(&CONFIGURATION.notifications, None).await;
    let notifier = redis_pool.get_notifier();
    let pg_pool: &'static PgPool = Box::leak(Box::new(get_postgres_pool().await));

    let problem_store = FileSystemStore::from(pg_pool).await;
    let executor = ProblemExecutor::new(*PLAYGROUND, *RESOURCES, Box::new(problem_store));
    //    Ok(store) => store,
    //    Err(_) => {
    //        // deliver ack because problem ID doesn't exists
    //        delivery
    //            .ack(BasicAckOptions::default())
    //            .await
    //            .expect("basic_ack");
    //        info!("ACK to rabbitmq");
    //        continue;
    //    },
    //};
    loop {
        let delivery = broker.on_message().await;
        if let Ok(Some(Ok(delivery))) = delivery {
            match serde_json::from_reader::<_, Submission>(&*delivery.data) {
                Ok(res) => {
                    match executor.execute(&res).await {
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
                            .execute(pg_pool)
                            .await
                            .unwrap();

                            info!("STORE submission result");
                            delivery
                                .ack(BasicAckOptions::default())
                                .await
                                .expect("basic_ack");
                            info!("ACK to rabbitmq");

                            match notifier
                                .notify(&format!("{mes}:{0}", res.id.as_u128()))
                                .await
                            {
                                Ok(_) => {
                                    info!("NOTIFIED to redis on channel_1");
                                },
                                Err(e) => {
                                    info!("UNSECCUSFUL noitifiacion , error: {}", e.to_string());
                                },
                            }
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
