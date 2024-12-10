use evaluator::{
    cli::init_tracing,
    consts::{CONFIGURATION, PLAYGROUND, RESOURCES},
    postgres::get_postgres_pool,
    problem_executor::ProblemExecutor,
    queue,
    store::FileSystemStore,
    submission::handle_message,
};
use lapin::Result;
use sqlx::{postgres::PgListener, PgPool};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let pg_pool: PgPool = get_postgres_pool().await;

    let problem_store = FileSystemStore::from(&pg_pool).await;
    let notification_channel = CONFIGURATION.postgres_queue.notification_channel.clone();
    let executor = ProblemExecutor::new(*PLAYGROUND, *RESOURCES, Box::new(problem_store));
    let mut listener = PgListener::connect_with(&pg_pool)
        .await
        .expect("Failed to connect to pg listener");
    listener
        .listen_all(vec![notification_channel.as_str()])
        .await
        .expect(format!("Failed to listen to channels {}", notification_channel).as_str());
    let pgmq = queue::PostgresQueue::new(&pg_pool).await;
    loop {
        info!("LOOPING");
        match listener.recv().await {
            Ok(_) => {
                info!("LISTENING TO CHANNEL");
                match pgmq.read().await {
                    Ok(Some(message)) => {
                        handle_message(&pgmq, &pg_pool, &executor, message).await;
                    },
                    Ok(None) => {
                        //continue;
                    },
                    Err(e) => {
                        println!("Error in pgmq: {:?}", e);
                        // handle notification to cluster administration
                        // that evaluator is not working
                    },
                };
            },
            Err(e) => {
                println!("Error in listener: {:?}", e);
                // handle notificatino to cluster administration
                // that evaluator is not working
            },
        }
    }
}
