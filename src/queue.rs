use pgmq::{Message, PGMQueue, PgmqError};
use primitypes::contest::Submission;
use sqlx::PgPool;

use crate::consts::CONFIGURATION;
pub struct PostgresQueue {
    conn: PGMQueue,
    queue_name: String,
}

impl PostgresQueue {
    pub async fn new(pg_pool: &PgPool) -> Self {
        let conn = PGMQueue::new_with_pool(pg_pool.clone()).await;

        Self {
            conn,
            queue_name: CONFIGURATION.postgres_queue.queue.clone(),
        }
    }

    pub async fn delete(&self, msg_id: i64) {
        self.conn.delete(&self.queue_name, msg_id).await;
    }

    pub async fn read(&self) -> Result<Option<Message<Submission>>, PgmqError> {
        self.conn
            .read::<Submission>(&self.queue_name, Some(30))
            .await
    }
}
